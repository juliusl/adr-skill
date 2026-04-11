mod embed;
mod init;
mod install;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "adr-skills",
    version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_COMMIT_SHA"), ")"),
    about = "Install and manage ADR skills and agents"
)]
struct Cli {
    /// Override the install base path (default: ~/.copilot)
    #[arg(long, global = true)]
    prefix: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install skills and/or agents
    Install {
        #[command(subcommand)]
        target: InstallTarget,
    },
    /// Bootstrap .adr/ directory in the current project
    Init,
    /// Full setup: install all + init (replaces make install-user)
    Setup,
}

#[derive(Subcommand)]
enum InstallTarget {
    /// Install skill definitions to ~/.copilot/skills/
    Skills,
    /// Install agent definitions to ~/.copilot/agents/
    Agents,
    /// Install both skills and agents
    All,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { target } => match target {
            InstallTarget::Skills => install::install_skills(&cli.prefix),
            InstallTarget::Agents => install::install_agents(&cli.prefix),
            InstallTarget::All => install::install_all(&cli.prefix),
        },
        Commands::Init => {
            let path = std::env::current_dir().expect("Could not determine current directory");
            init::init_project(&path);
        }
        Commands::Setup => {
            install::install_all(&cli.prefix);
            let path = std::env::current_dir().expect("Could not determine current directory");
            init::init_project(&path);
            println!("\n=== Setup complete ===");
        }
    }
}
