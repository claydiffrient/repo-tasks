use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

use crate::{Config, Task};

/// Open a task in the default editor
pub fn open(slug_or_id: String) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    // Find the task
    let tasks_base = PathBuf::from(".repo-tasks/tasks");
    let mut found_path: Option<PathBuf> = None;

    for entry in WalkDir::new(&tasks_base)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let filename = entry.file_name().to_string_lossy();

        if filename.contains(&slug_or_id) {
            // Verify it's a valid task
            if Task::from_file(entry.path()).is_ok() {
                found_path = Some(entry.path().to_path_buf());
                break;
            }
        }
    }

    let path = match found_path {
        Some(p) => p,
        None => bail!("Task not found: {}", slug_or_id),
    };

    // Determine editor to use
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            // Platform-specific defaults
            if cfg!(target_os = "macos") {
                "open".to_string()
            } else if cfg!(target_os = "windows") {
                "notepad".to_string()
            } else {
                "vi".to_string()
            }
        });

    println!("Opening {} in {}...", path.display(), editor);

    // Open the file in the editor
    let status = Command::new(&editor)
        .arg(&path)
        .status()
        .context(format!("Failed to open editor: {}", editor))?;

    if !status.success() {
        bail!("Editor exited with error code: {:?}", status.code());
    }

    println!("✓ Closed editor");

    Ok(())
}
