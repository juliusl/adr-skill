use clap::{Parser, Subcommand};

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
    Init,
    /// Read JSONL from stdin and persist each record to the data store
    Ingest,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            eprintln!("init: not yet implemented");
        }
        Commands::Ingest => {
            eprintln!("ingest: not yet implemented");
        }
    }
}
