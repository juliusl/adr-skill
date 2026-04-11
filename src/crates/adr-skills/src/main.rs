mod embed;
mod init;
mod install;

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
        /// Participation mode
        #[arg(long, default_value = "autonomous")]
        participation: Participation,
        /// Enable auto-commit on task completion
        #[arg(long, default_value = "true")]
        auto_commit: String,
        /// Enable auto-delegate to implement-adr
        #[arg(long, default_value = "true")]
        auto_delegate: String,
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
    },
    /// Full setup: install skills, agents, and bootstrap project directory
    Setup {
        /// Participation mode
        #[arg(long, default_value = "autonomous")]
        participation: Participation,
        /// Enable auto-commit on task completion
        #[arg(long, default_value = "true")]
        auto_commit: String,
        /// Enable auto-delegate to implement-adr
        #[arg(long, default_value = "true")]
        auto_delegate: String,
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { dry_run, force, target } => match target {
            InstallTarget::Skills => install::install_skills(&cli.prefix, dry_run, force),
            InstallTarget::Agents => install::install_agents(&cli.prefix, dry_run, force),
            InstallTarget::All => install::install_all(&cli.prefix, dry_run, force),
        },
        Commands::Init {
            participation, auto_commit, auto_delegate, scope,
            tpm, review, tech_writer, ux_review, dx_review, code_review,
        } => {
            let path = std::env::current_dir().expect("Could not determine current directory");
            let config = init::InitConfig {
                participation: participation.to_string(),
                auto_commit, auto_delegate, scope,
                tpm, review, tech_writer, ux_review, dx_review, code_review,
            };
            init::init_project(&path, &config);
        }
        Commands::Setup {
            participation, auto_commit, auto_delegate, scope,
            tpm, review, tech_writer, ux_review, dx_review, code_review,
        } => {
            install::install_all(&cli.prefix, false, false);
            let path = std::env::current_dir().expect("Could not determine current directory");
            let config = init::InitConfig {
                participation: participation.to_string(),
                auto_commit, auto_delegate, scope,
                tpm, review, tech_writer, ux_review, dx_review, code_review,
            };
            init::init_project(&path, &config);
            println!("\n=== Setup complete ===");
        }
    }
}
