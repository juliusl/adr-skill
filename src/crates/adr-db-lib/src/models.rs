use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::task_summaries;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = task_summaries)]
#[allow(dead_code)]
pub struct TaskSummary {
    pub id: i32,
    pub task_id: String,
    pub status: String,
    pub cost: String,
    pub commit_sha: String,
    pub description: String,
    pub ingested_at: String,
    pub source_plan: String,
}

#[derive(Insertable)]
#[diesel(table_name = task_summaries)]
pub struct NewTaskSummary {
    pub task_id: String,
    pub status: String,
    pub cost: String,
    pub commit_sha: String,
    pub description: String,
    pub source_plan: String,
}

/// Represents a single JSONL record from extract-summary.awk
#[derive(Deserialize)]
pub struct JsonlTaskRecord {
    pub task_id: String,
    pub status: String,
    pub cost: String,
    pub commit: String,
    pub description: String,
    #[serde(default)]
    pub source_plan: String,
}

impl From<JsonlTaskRecord> for NewTaskSummary {
    fn from(record: JsonlTaskRecord) -> Self {
        NewTaskSummary {
            task_id: record.task_id,
            status: record.status,
            cost: record.cost,
            commit_sha: record.commit,
            description: record.description,
            source_plan: record.source_plan,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonl_deserialization() {
        let json = r#"{"task_id":"1.1","status":"done","cost":"small","commit":"abc1234","description":"Create config file"}"#;
        let record: JsonlTaskRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.task_id, "1.1");
        assert_eq!(record.status, "done");
        assert_eq!(record.cost, "small");
        assert_eq!(record.commit, "abc1234");
        assert_eq!(record.description, "Create config file");
        assert_eq!(record.source_plan, ""); // default when absent
    }

    #[test]
    fn test_jsonl_to_new_task_summary() {
        let json = r#"{"task_id":"2.1","status":"done","cost":"medium","commit":"def5678","description":"Add validation logic"}"#;
        let record: JsonlTaskRecord = serde_json::from_str(json).unwrap();
        let summary = NewTaskSummary::from(record);
        assert_eq!(summary.task_id, "2.1");
        assert_eq!(summary.commit_sha, "def5678");
        assert_eq!(summary.source_plan, ""); // default when absent
    }

    #[test]
    fn test_round_trip_insert_query() {
        use diesel::prelude::*;
        use diesel::sqlite::SqliteConnection;
        use diesel_migrations::MigrationHarness;

        let mut conn = SqliteConnection::establish(":memory:").unwrap();
        conn.run_pending_migrations(crate::db::MIGRATIONS).unwrap();

        let new = NewTaskSummary {
            task_id: "1.1".to_string(),
            status: "done".to_string(),
            cost: "small".to_string(),
            commit_sha: "abc1234".to_string(),
            description: "Create config file".to_string(),
            source_plan: "0029.0.plan.md".to_string(),
        };

        diesel::insert_into(crate::schema::task_summaries::table)
            .values(&new)
            .execute(&mut conn)
            .unwrap();

        let results: Vec<TaskSummary> = crate::schema::task_summaries::table
            .select(TaskSummary::as_select())
            .load(&mut conn)
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].task_id, "1.1");
        assert_eq!(results[0].status, "done");
        assert_eq!(results[0].cost, "small");
        assert_eq!(results[0].commit_sha, "abc1234");
        assert_eq!(results[0].description, "Create config file");
    }
}
