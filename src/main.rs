use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod models;
mod utils;

use commands::{hooks_install, hooks_list, hooks_uninstall, init, list, move_task, new, open, save, search, show, update};
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
    New {
        /// Task title
        #[arg(short, long)]
        title: Option<String>,
        /// Priority level (Critical, High, Medium, Low)
        #[arg(short, long)]
        priority: Option<String>,
        /// Comma-separated tags
        #[arg(short = 'g', long)]
        tags: Option<String>,
        /// Task description/notes
        #[arg(short, long)]
        notes: Option<String>,
    },
    /// List tasks in a given status
    List {
        /// Status to list (defaults to "todo")
        status: Option<String>,
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
        /// Filter by tag
        #[arg(short, long)]
        tag: Option<String>,
    },
    /// Show details of a specific task
    Show {
        /// Task slug or ID
        slug_or_id: String,
    },
    /// Update task properties
    Update {
        /// Task slug or ID
        slug_or_id: String,
    },
    /// Move a task to a different status
    Move {
        /// Task slug or ID
        slug_or_id: String,
        /// New status (todo, in-progress, testing, done)
        new_status: String,
    },
    /// Open a task in your default editor
    Open {
        /// Task slug or ID
        slug_or_id: String,
    },
    /// Search for tasks by content
    Search {
        /// Search query (regex supported)
        query: String,
    },
    /// Commit changes to the repository
    Save {
        /// Commit message (auto-generated if not provided)
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Manage git hooks for task automation
    Hooks {
        #[command(subcommand)]
        subcommand: HooksSubcommand,
    },
}

#[derive(Subcommand)]
enum HooksSubcommand {
    /// Install git hooks for task automation
    Install {
        /// Install specific hook (pre-commit, post-commit, prepare-commit-msg, post-checkout)
        hook_name: Option<String>,
    },
    /// Uninstall repo-tasks git hooks
    Uninstall,
    /// List installed git hooks
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { project_name } => {
            init(project_name)?;
        }
        Commands::New {
            title,
            priority,
            tags,
            notes,
        } => {
            new(title, priority, tags, notes)?;
        }
        Commands::List {
            status,
            priority,
            tag,
        } => {
            list(status, priority, tag)?;
        }
        Commands::Show { slug_or_id } => {
            show(slug_or_id)?;
        }
        Commands::Update { slug_or_id } => {
            update(slug_or_id)?;
        }
        Commands::Move {
            slug_or_id,
            new_status,
        } => {
            move_task(slug_or_id, new_status)?;
        }
        Commands::Open { slug_or_id } => {
            open(slug_or_id)?;
        }
        Commands::Search { query } => {
            search(query)?;
        }
        Commands::Save { message } => {
            save(message)?;
        }
        Commands::Hooks { subcommand } => match subcommand {
            HooksSubcommand::Install { hook_name } => {
                hooks_install(hook_name)?;
            }
            HooksSubcommand::Uninstall => {
                hooks_uninstall()?;
            }
            HooksSubcommand::List => {
                hooks_list()?;
            }
        },
    }

    Ok(())
}
