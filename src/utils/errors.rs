use anyhow::{bail, Result};
use std::path::Path;

/// Check if repo-tasks is initialized, return helpful error if not
#[allow(dead_code)]
pub fn ensure_initialized() -> Result<()> {
    if !crate::Config::is_initialized() {
        bail!(
            "Not in a repo-tasks repository.\n\
            \n\
            To get started, run:\n\
            \n\
            \x1b[1m  tasks init\x1b[0m\n\
            \n\
            This will create a .repo-tasks/ directory in your current location."
        );
    }
    Ok(())
}

/// Provide a helpful error when a task is not found
#[allow(dead_code)]
pub fn task_not_found(slug_or_id: &str) -> anyhow::Error {
    anyhow::anyhow!(
        "Task not found: {}\n\
        \n\
        Try:\n\
        \x1b[1m  tasks list\x1b[0m           # List all tasks\n\
        \x1b[1m  tasks search {}\x1b[0m  # Search for tasks",
        slug_or_id,
        slug_or_id
    )
}

/// Provide context for file operation errors
#[allow(dead_code)]
pub fn file_context(path: &Path, operation: &str) -> String {
    format!("Failed to {} file: {}", operation, path.display())
}

/// Provide helpful error for invalid status
#[allow(dead_code)]
pub fn invalid_status(status: &str, valid_statuses: &[String]) -> anyhow::Error {
    anyhow::anyhow!(
        "Invalid status: '{}'\n\
        \n\
        Valid statuses are: {}",
        status,
        valid_statuses.join(", ")
    )
}

/// Provide helpful error for invalid priority
#[allow(dead_code)]
pub fn invalid_priority(priority: &str, valid_priorities: &[String]) -> anyhow::Error {
    anyhow::anyhow!(
        "Invalid priority: '{}'\n\
        \n\
        Valid priorities are: {}",
        priority,
        valid_priorities.join(", ")
    )
}
