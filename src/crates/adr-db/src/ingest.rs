use std::io::{self, BufRead};
use std::path::Path;

use diesel::RunQueryDsl;

use adr_db_lib::db;
use adr_db_lib::models::{JsonlTaskRecord, NewTaskSummary};
use adr_db_lib::schema::task_summaries;

pub fn run_ingest(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = db::establish_connection(db_path)?;

    let stdin = io::stdin();
    let mut errors = 0u64;

    for (line_num, line) in stdin.lock().lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<JsonlTaskRecord>(&line) {
            Ok(record) => {
                let new_summary = NewTaskSummary::from(record);
                if let Err(e) = diesel::insert_into(task_summaries::table)
                    .values(&new_summary)
                    .execute(&mut conn)
                {
                    eprintln!("line {}: insert error: {e}", line_num + 1);
                    errors += 1;
                }
            }
            Err(e) => {
                eprintln!("line {}: {e}", line_num + 1);
                errors += 1;
            }
        }
    }

    if errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}
