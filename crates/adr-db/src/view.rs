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

pub fn run_view(
    db_path: &Path,
    table_name: Option<&str>,
    output: &str,
    limit: Option<i64>,
    no_header: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_url = db_path.to_string_lossy().to_string();
    let mut conn = SqliteConnection::establish(&db_url)?;

    match table_name {
        None => list_tables(&mut conn),
        Some(name) => match name {
            "task_summaries" => view_task_summaries(&mut conn, output, limit, no_header),
            _ => {
                eprintln!("error: unknown table: {name}");
                std::process::exit(1);
            }
        },
    }
}

fn list_tables(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    let tables: Vec<TableName> = diesel::sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE '__diesel%'",
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
    if desc.len() <= max_len {
        desc.to_string()
    } else {
        let truncated: String = desc.chars().take(max_len).collect();
        format!("{truncated}…")
    }
}

fn view_task_summaries(
    conn: &mut SqliteConnection,
    output: &str,
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
        "jsonl" => {
            for row in &results {
                let json = serde_json::json!({
                    "task_id": row.task_id,
                    "status": row.status,
                    "cost": row.cost,
                    "commit": row.commit_sha,
                    "description": row.description,
                });
                writeln!(out, "{}", json)?;
            }
        }
        _ => {
            // TSV output
            if !no_header {
                writeln!(out, "task_id\tstatus\tcost\tcommit_sha\tdescription")?;
            }
            for row in &results {
                writeln!(
                    out,
                    "{}\t{}\t{}\t{}\t{}",
                    row.task_id,
                    row.status,
                    row.cost,
                    row.commit_sha,
                    truncate_description(&row.description, 60),
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
            },
            NewTaskSummary {
                task_id: "1.2".to_string(),
                status: "done".to_string(),
                cost: "medium".to_string(),
                commit_sha: "def5678".to_string(),
                description: "Add validation logic".to_string(),
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
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE '__diesel%'",
        )
        .load(&mut conn)
        .unwrap();
        let names: Vec<&str> = tables.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"task_summaries"));
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
        // 60 chars + the … character
        assert_eq!(result.chars().count(), 61);
    }

    #[test]
    fn test_truncate_description_exact() {
        let exact = "a".repeat(60);
        assert_eq!(truncate_description(&exact, 60), exact);
    }
}
