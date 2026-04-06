use std::path::Path;

use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Establish a SQLite connection to the given database path.
pub fn establish_connection(db_path: &Path) -> Result<SqliteConnection, Box<dyn std::error::Error>> {
    let db_url = db_path.to_string_lossy().to_string();
    let conn = SqliteConnection::establish(&db_url)?;
    Ok(conn)
}

/// Initialize the database: create parent directories and run pending migrations.
pub fn run_init(db_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut conn = establish_connection(db_path)?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("migration error: {e}"))?;

    Ok(())
}
