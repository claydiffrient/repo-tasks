use anyhow::{bail, Result};
use console::style;
use dialoguer::{Input, Select};

use crate::utils;
use crate::{Config, Task};

/// Create a new task, either interactively or with command-line arguments
pub fn new(
    title_arg: Option<String>,
    priority_arg: Option<String>,
    tags_arg: Option<String>,
    notes_arg: Option<String>,
) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let config = Config::load()?;

    // Determine if we're in non-interactive mode
    let non_interactive = title_arg.is_some();

    let title = if let Some(t) = title_arg {
        // Non-interactive mode
        if t.trim().is_empty() {
            bail!("Task title cannot be empty");
        }
        t.trim().to_string()
    } else {
        // Interactive mode - check if we have a TTY
        if !atty::is(atty::Stream::Stdin) {
            bail!(
                "Not running in a terminal and no title provided.\n\
                \n\
                Use non-interactive mode:\n\
                \n\
                \x1b[1m  tasks new --title \"Your task title\" --priority High\x1b[0m\n\
                \n\
                Or run in an interactive terminal."
            );
        }

        // Prompt for title
        let input: String = Input::new().with_prompt("Task title").interact_text()?;

        if input.trim().is_empty() {
            bail!("Task title cannot be empty");
        }
        input.trim().to_string()
    };

    let priority = if let Some(p) = priority_arg {
        // Validate priority
        if !config.priorities.contains(&p) {
            bail!(
                "Invalid priority: '{}'\n\
                \n\
                Valid priorities are: {}",
                p,
                config.priorities.join(", ")
            );
        }
        p
    } else if non_interactive {
        // Default to Medium in non-interactive mode
        "Medium".to_string()
    } else {
        // Interactive mode - prompt for priority
        let priority_index = Select::new()
            .with_prompt("Priority")
            .items(&config.priorities)
            .default(1) // Default to "Medium" (index 1)
            .interact()?;

        config.priorities[priority_index].clone()
    };

    // Create task
    let mut task = Task::new(title, priority);

    // Add tags if provided
    if let Some(tags_str) = tags_arg {
        let tags: Vec<String> = tags_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if !tags.is_empty() {
            task.tags = Some(tags);
        }
    }

    // Add notes/body if provided
    if let Some(notes) = notes_arg {
        task.body = notes;
    }

    let path = task.file_path("todo");

    // Save task
    task.to_file(&path)?;

    utils::success(&format!("Created task: {}", style(&task.slug).bold()));
    println!("  ID: {}", style(&task.id).dim());
    println!(
        "  Priority: {}",
        utils::priority_badge(task.priority.as_ref().unwrap())
    );
    if let Some(tags) = &task.tags {
        println!("  Tags: {}", utils::tags(tags));
    }
    println!("  File: {}", style(path.display()).dim());

    Ok(())
}
