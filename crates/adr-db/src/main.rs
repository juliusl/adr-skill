use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod fetch;
mod ingest;
mod view;

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
    /// Inspect database contents (diagnostic, no stability guarantees).
    ///
    /// WARNING: This command is proto-porcelain. Output format, flags, and
    /// behavior may change at any time. Do not depend on this output in
    /// scripts or downstream tooling.
    View {
        /// Table name to inspect. Omit to list all tables.
        table_name: Option<String>,
        /// Output format: tsv (default) or jsonl
        #[arg(long, default_value = "tsv")]
        output: view::OutputFormat,
        /// Limit number of output rows
        #[arg(long)]
        limit: Option<i64>,
        /// Suppress header row in TSV mode
        #[arg(long)]
        no_header: bool,
        /// Path to the SQLite database file
        #[arg(long, default_value = ".adr/var/adr.db")]
        db_path: PathBuf,
    },
    /// Fetch a work item from a remote and output normalized JSONL
    Fetch {
        /// Remote type (e.g., gitea)
        remote: String,
        /// Work item ID
        id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { db_path } => {
            if let Err(e) = adr_db_lib::db::run_init(&db_path) {
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
        Commands::View {
            table_name,
            output,
            limit,
            no_header,
            db_path,
        } => {
            if let Err(e) = view::run_view(
                &db_path,
                table_name.as_deref(),
                output,
                limit,
                no_header,
            ) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        Commands::Fetch { remote, id } => {
            if let Err(e) = fetch::run_fetch(&remote, &id) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    }
}
