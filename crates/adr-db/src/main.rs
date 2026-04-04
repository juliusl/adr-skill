use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod init;
mod ingest;
mod models;
mod schema;

#[derive(Parser)]
#[command(name = "adr-db")]
#[command(about = "Plumbing CLI for ingesting JSONL data into a local SQLite database")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the data store (create schema, ensure .adr/var/ exists)
    Init {
        /// Path to the SQLite database file
        #[arg(long, default_value = ".adr/var/adr.db")]
        db_path: PathBuf,
    },
    /// Read JSONL from stdin and persist each record to the data store
    Ingest {
        /// Path to the SQLite database file
        #[arg(long, default_value = ".adr/var/adr.db")]
        db_path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { db_path } => {
            if let Err(e) = init::run_init(&db_path) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        Commands::Ingest { db_path } => {
            if let Err(e) = ingest::run_ingest(&db_path) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    }
}
