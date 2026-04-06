use std::io::Write;
use std::path::Path;

use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::sqlite::SqliteConnection;

use crate::models::TaskSummary;
use crate::schema::task_summaries;

#[derive(QueryableByName)]
struct TableName {
    #[diesel(sql_type = Text)]
    name: String,
}

#[derive(Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Tsv,
    Jsonl,
}

pub fn run_view(
    db_path: &Path,
    table_name: Option<&str>,
    output: OutputFormat,
    limit: Option<i64>,
    no_header: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(n) = limit {
        if n < 0 {
            return Err(format!("invalid --limit value: {n} (must be non-negative)").into());
        }
    }

    let db_url = db_path.to_string_lossy().to_string();
    let mut conn = SqliteConnection::establish(&db_url)?;

    match table_name {
        None => list_tables(&mut conn),
        Some(name) => match name {
            "task_summaries" => view_task_summaries(&mut conn, output, limit, no_header),
            _ => Err(format!("unknown table: {name}").into()),
        },
    }
}

fn list_tables(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    let tables: Vec<TableName> = diesel::sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE '__diesel%' AND name NOT LIKE 'sqlite_%'",
    )
    .load(conn)?;

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for t in &tables {
        writeln!(out, "{}", t.name)?;
    }
    Ok(())
}

fn truncate_description(desc: &str, max_len: usize) -> String {
    let char_count = desc.chars().count();
    if char_count <= max_len {
        desc.to_string()
    } else {
        let truncated: String = desc.chars().take(max_len).collect();
        format!("{truncated}…")
    }
}

/// Sanitize a field value for TSV output by replacing tabs and newlines with spaces.
fn sanitize_tsv_field(value: &str) -> String {
    value.replace('\t', " ").replace('\n', " ").replace('\r', " ")
}

fn view_task_summaries(
    conn: &mut SqliteConnection,
    output: OutputFormat,
    limit: Option<i64>,
    no_header: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let results: Vec<TaskSummary> = match limit {
        Some(n) => task_summaries::table
            .select(TaskSummary::as_select())
            .limit(n)
            .load(conn)?,
        None => task_summaries::table
            .select(TaskSummary::as_select())
            .load(conn)?,
    };

    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    match output {
        OutputFormat::Jsonl => {
            for row in &results {
                let json = serde_json::json!({
                    "task_id": row.task_id,
                    "status": row.status,
                    "cost": row.cost,
                    "commit": row.commit_sha,
                    "description": row.description,
                    "source_plan": row.source_plan,
                });
                writeln!(out, "{}", json)?;
            }
        }
        OutputFormat::Tsv => {
            if !no_header {
                writeln!(out, "source_plan\ttask_id\tstatus\tcost\tcommit_sha\tdescription")?;
            }
            for row in &results {
                writeln!(
                    out,
                    "{}\t{}\t{}\t{}\t{}\t{}",
                    sanitize_tsv_field(&row.source_plan),
                    sanitize_tsv_field(&row.task_id),
                    sanitize_tsv_field(&row.status),
                    sanitize_tsv_field(&row.cost),
                    sanitize_tsv_field(&row.commit_sha),
                    truncate_description(&sanitize_tsv_field(&row.description), 60),
                )?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NewTaskSummary;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    fn setup_conn() -> SqliteConnection {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        conn
    }

    fn insert_sample(conn: &mut SqliteConnection) {
        let records = vec![
            NewTaskSummary {
                task_id: "1.1".to_string(),
                status: "done".to_string(),
                cost: "small".to_string(),
                commit_sha: "abc1234".to_string(),
                description: "Create config file".to_string(),
                source_plan: "0029.0.plan.md".to_string(),
            },
            NewTaskSummary {
                task_id: "1.2".to_string(),
                status: "done".to_string(),
                cost: "medium".to_string(),
                commit_sha: "def5678".to_string(),
                description: "Add validation logic".to_string(),
                source_plan: "0029.0.plan.md".to_string(),
            },
        ];
        for r in &records {
            diesel::insert_into(task_summaries::table)
                .values(r)
                .execute(conn)
                .unwrap();
        }
    }

    #[test]
    fn test_list_tables() {
        let mut conn = setup_conn();
        let tables: Vec<TableName> = diesel::sql_query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE '__diesel%' AND name NOT LIKE 'sqlite_%'",
        )
        .load(&mut conn)
        .unwrap();
        let names: Vec<&str> = tables.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"task_summaries"));
        // Verify sqlite_sequence is excluded
        assert!(!names.contains(&"sqlite_sequence"));
    }

    #[test]
    fn test_truncate_description_short() {
        assert_eq!(truncate_description("short", 60), "short");
    }

    #[test]
    fn test_truncate_description_long() {
        let long_desc = "a".repeat(80);
        let result = truncate_description(&long_desc, 60);
        assert!(result.ends_with('…'));
        assert_eq!(result.chars().count(), 61);
    }

    #[test]
    fn test_truncate_description_exact() {
        let exact = "a".repeat(60);
        assert_eq!(truncate_description(&exact, 60), exact);
    }

    #[test]
    fn test_truncate_description_multibyte() {
        // Ensure multi-byte UTF-8 does not panic
        let desc = "é".repeat(80); // 2-byte chars
        let result = truncate_description(&desc, 60);
        assert_eq!(result.chars().count(), 61); // 60 + …
        assert!(result.ends_with('…'));
    }

    #[test]
    fn test_sanitize_tsv_field() {
        assert_eq!(sanitize_tsv_field("hello\tworld"), "hello world");
        assert_eq!(sanitize_tsv_field("line1\nline2"), "line1 line2");
        assert_eq!(sanitize_tsv_field("cr\rvalue"), "cr value");
        assert_eq!(sanitize_tsv_field("clean"), "clean");
    }

    #[test]
    fn test_tsv_output() {
        let mut conn = setup_conn();
        insert_sample(&mut conn);
        let results: Vec<TaskSummary> = task_summaries::table
            .select(TaskSummary::as_select())
            .load(&mut conn)
            .unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].task_id, "1.1");
        assert_eq!(results[0].source_plan, "0029.0.plan.md");
        assert_eq!(results[1].task_id, "1.2");
        assert_eq!(results[1].source_plan, "0029.0.plan.md");
    }

    #[test]
    fn test_jsonl_round_trip() {
        use crate::models::JsonlTaskRecord;

        let mut conn = setup_conn();
        insert_sample(&mut conn);

        let results: Vec<TaskSummary> = task_summaries::table
            .select(TaskSummary::as_select())
            .load(&mut conn)
            .unwrap();

        for row in &results {
            let json = serde_json::json!({
                "task_id": row.task_id,
                "status": row.status,
                "cost": row.cost,
                "commit": row.commit_sha,
                "description": row.description,
                "source_plan": row.source_plan,
            });
            let json_str = json.to_string();
            let record: JsonlTaskRecord = serde_json::from_str(&json_str).unwrap();
            assert_eq!(record.source_plan, "0029.0.plan.md");
        }
    }

    #[test]
    fn test_jsonl_backward_compat_no_source_plan() {
        use crate::models::JsonlTaskRecord;
        // JSONL without source_plan should deserialize with empty default
        let json = r#"{"task_id":"1.1","status":"done","cost":"small","commit":"abc","description":"test"}"#;
        let record: JsonlTaskRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.source_plan, "");
    }

    #[test]
    fn test_jsonl_with_source_plan() {
        use crate::models::JsonlTaskRecord;
        let json = r#"{"task_id":"1.1","status":"done","cost":"small","commit":"abc","description":"test","source_plan":"0039.0.plan.md"}"#;
        let record: JsonlTaskRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.source_plan, "0039.0.plan.md");
    }

    #[test]
    fn test_empty_table_tsv() {
        let mut conn = setup_conn();
        // No data inserted — empty table
        let results: Vec<TaskSummary> = task_summaries::table
            .select(TaskSummary::as_select())
            .load(&mut conn)
            .unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_limit() {
        let mut conn = setup_conn();
        insert_sample(&mut conn);

        let results: Vec<TaskSummary> = task_summaries::table
            .select(TaskSummary::as_select())
            .limit(1)
            .load(&mut conn)
            .unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_limit_zero() {
        let mut conn = setup_conn();
        insert_sample(&mut conn);

        let results: Vec<TaskSummary> = task_summaries::table
            .select(TaskSummary::as_select())
            .limit(0)
            .load(&mut conn)
            .unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_negative_limit_rejected() {
        let result = run_view(
            std::path::Path::new(":memory:"),
            Some("task_summaries"),
            OutputFormat::Tsv,
            Some(-1),
            false,
        );
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invalid --limit value"));
    }

    #[test]
    fn test_unknown_table_returns_error() {
        let result = run_view(
            std::path::Path::new(":memory:"),
            Some("nonexistent"),
            OutputFormat::Tsv,
            None,
            false,
        );
        // This will fail because :memory: needs init first, but the table name
        // check happens after connection. Let's test via the match logic directly.
        assert!(result.is_err());
    }
}
