use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod models;

use commands::{init, list, new, show};
use models::{Config, Task};

#[derive(Parser)]
#[command(name = "tasks")]
#[command(about = "Fast task management for git repositories", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repo-tasks repository
    Init {
        /// Project name (defaults to current directory name)
        #[arg(short, long)]
        project_name: Option<String>,
    },
    /// Create a new task
    New,
    /// List tasks in a given status
    List {
        /// Status to list (defaults to "todo")
        status: Option<String>,
    },
    /// Show details of a specific task
    Show {
        /// Task slug or ID
        slug_or_id: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project_name } => {
            init(project_name)?;
        }
        Commands::New => {
            new()?;
        }
        Commands::List { status } => {
            list(status)?;
        }
        Commands::Show { slug_or_id } => {
            show(slug_or_id)?;
        }
    }

    Ok(())
}
