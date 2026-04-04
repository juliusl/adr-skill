use std::path::Path;

use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_init(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let db_url = db_path.to_string_lossy().to_string();
    let mut conn = SqliteConnection::establish(&db_url)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("migration error: {e}"))?;

    Ok(())
}
