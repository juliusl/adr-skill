//! CLI entry point for the adr-skills installer.

mod embed;
mod init;
mod install;
mod new_problem;

use clap::{Parser, Subcommand, ValueEnum};
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

#[derive(Clone, ValueEnum)]
enum Participation {
    Guided,
    Autonomous,
}

impl std::fmt::Display for Participation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Participation::Guided => write!(f, "guided"),
            Participation::Autonomous => write!(f, "autonomous"),
        }
    }
}

/// Shared flags for Init and Setup commands.
#[derive(clap::Args, Clone)]
struct InitArgs {
    /// Participation mode
    #[arg(long, default_value = "autonomous")]
    participation: Participation,
    /// Enable auto-commit on task completion
    #[arg(long, default_value_t = true)]
    auto_commit: bool,
    /// Enable auto-delegate to implement-adr
    #[arg(long, default_value_t = true)]
    auto_delegate: bool,
    /// Author scope (project or user)
    #[arg(long, default_value = "user")]
    scope: String,
    /// TPM agent name
    #[arg(long, default_value = "juliusl-tpm-v2")]
    tpm: String,
    /// Review agent name
    #[arg(long, default_value = "juliusl-editor-v5")]
    review: String,
    /// Tech writer agent name
    #[arg(long, default_value = "juliusl-tech-writer-v1")]
    tech_writer: String,
    /// UX review agent name
    #[arg(long, default_value = "juliusl-ux-reviewer-v1")]
    ux_review: String,
    /// DX review agent name
    #[arg(long, default_value = "juliusl-dx-reviewer-v1")]
    dx_review: String,
    /// Code review agents (comma-separated)
    #[arg(long, default_value = "juliusl-code-reviewer-analytics-v5,juliusl-code-reviewer-sweep-v5")]
    code_review: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Install skills and/or agents
    Install {
        /// Preview changes without writing files
        #[arg(long)]
        dry_run: bool,
        /// Bypass version check and overwrite unconditionally
        #[arg(long)]
        force: bool,
        #[command(subcommand)]
        target: InstallTarget,
    },
    /// Bootstrap .adr/ directory in the current project
    Init {
        #[command(flatten)]
        args: InitArgs,
    },
    /// Full setup: install skills, agents, and bootstrap project directory
    Setup {
        #[command(flatten)]
        args: InitArgs,
    },
    /// Create a new ADR artifact
    New {
        #[command(subcommand)]
        target: NewTarget,
    },
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

#[derive(Subcommand)]
enum NewTarget {
    /// Create a new problem file (opens TUI for structured input)
    Problem {
        /// Problem title
        title: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { dry_run, force, target } => match target {
            InstallTarget::Skills => install::install_skills(&cli.prefix, dry_run, force),
            InstallTarget::Agents => install::install_agents(&cli.prefix, dry_run, force),
            InstallTarget::All => install::install_all(&cli.prefix, dry_run, force),
        },
        Commands::Init { args } => {
            let path = std::env::current_dir().expect("Could not determine current directory");
            let config = init::InitConfig::from_args(&args);
            init::init_project(&path, &config);
        }
        Commands::Setup { args } => {
            install::install_all(&cli.prefix, false, false);
            let path = std::env::current_dir().expect("Could not determine current directory");
            let config = init::InitConfig::from_args(&args);
            init::init_project(&path, &config);
            println!("\n=== Setup complete ===");
        }
        Commands::New { target } => match target {
            NewTarget::Problem { title } => {
                let title = title.join(" ");
                if title.is_empty() {
                    eprintln!("Error: problem title is required");
                    eprintln!("Usage: adr-skills new problem <title...>");
                    std::process::exit(1);
                }
                match new_problem::run_new_problem(title) {
                    Ok(Some(path)) => {
                        println!("Saved: {}", path.display());
                        println!("\nNext: invoke solve-adr to explore this problem");
                    }
                    Ok(None) => {
                        println!("Cancelled.");
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
    }
}
