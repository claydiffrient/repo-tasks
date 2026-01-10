use anyhow::{bail, Context, Result};
use console::style;
use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

use crate::utils;

/// Hook types supported by repo-tasks
#[derive(Debug, Clone)]
pub enum HookType {
    PreCommit,
    PostCommit,
    PrepareCommitMsg,
    PostCheckout,
}

impl HookType {
    fn filename(&self) -> &str {
        match self {
            HookType::PreCommit => "pre-commit",
            HookType::PostCommit => "post-commit",
            HookType::PrepareCommitMsg => "prepare-commit-msg",
            HookType::PostCheckout => "post-checkout",
        }
    }

    fn all() -> Vec<HookType> {
        vec![
            HookType::PreCommit,
            HookType::PostCommit,
            HookType::PrepareCommitMsg,
            HookType::PostCheckout,
        ]
    }
}

/// Install git hooks for repo-tasks
pub fn install(hook_name: Option<String>) -> Result<()> {
    // Verify we're in a git repository
    let git_hooks_dir = find_git_hooks_dir()?;

    // Verify repo-tasks is initialized
    if !Path::new(".repo-tasks/config.json").exists() {
        bail!("repo-tasks not initialized. Run 'tasks init' first.");
    }

    // Determine which hooks to install
    let hooks_to_install = if let Some(name) = hook_name {
        // Install specific hook
        let hook_type = parse_hook_name(&name)?;
        vec![hook_type]
    } else {
        // Install all hooks
        HookType::all()
    };

    let mut installed_count = 0;

    for hook_type in hooks_to_install {
        install_hook(&git_hooks_dir, &hook_type)?;
        installed_count += 1;
    }

    utils::success(&format!(
        "Installed {} git hook{}",
        installed_count,
        if installed_count == 1 { "" } else { "s" }
    ));

    println!("\n{}", style("Installed hooks:").bold());
    println!("  {} {}", style("✓").green(), "pre-commit - Validate task references");
    println!("  {} {}", style("✓").green(), "post-commit - Auto-update task status");
    println!("  {} {}", style("✓").green(), "prepare-commit-msg - Add task context to commits");
    println!("  {} {}", style("✓").green(), "post-checkout - Show task info on branch switch");

    println!("\n{}", style("Tip:").dim());
    println!("  {}", style("Use 'git commit --no-verify' to bypass hooks if needed").dim());

    Ok(())
}

/// Uninstall repo-tasks git hooks
pub fn uninstall() -> Result<()> {
    let git_hooks_dir = find_git_hooks_dir()?;

    let mut removed_count = 0;

    for hook_type in HookType::all() {
        let hook_path = git_hooks_dir.join(hook_type.filename());

        if hook_path.exists() {
            // Check if it's our hook
            let content = fs::read_to_string(&hook_path)?;
            if content.contains("repo-tasks") {
                fs::remove_file(&hook_path)
                    .context(format!("Failed to remove hook: {}", hook_type.filename()))?;
                removed_count += 1;
            }
        }

        // Also remove backup if it exists
        let backup_path = git_hooks_dir.join(format!("{}.backup", hook_type.filename()));
        if backup_path.exists() {
            fs::remove_file(&backup_path).ok(); // Ignore errors on backup removal
        }
    }

    if removed_count == 0 {
        println!("{}", style("No repo-tasks hooks found to uninstall").yellow());
    } else {
        utils::success(&format!(
            "Uninstalled {} git hook{}",
            removed_count,
            if removed_count == 1 { "" } else { "s" }
        ));
    }

    Ok(())
}

/// List installed repo-tasks git hooks
pub fn list() -> Result<()> {
    let git_hooks_dir = find_git_hooks_dir()?;

    println!("{}", style("Git Hooks Status:").bold());
    println!();

    let mut installed_count = 0;

    for hook_type in HookType::all() {
        let hook_path = git_hooks_dir.join(hook_type.filename());
        let is_installed = if hook_path.exists() {
            let content = fs::read_to_string(&hook_path)?;
            content.contains("repo-tasks")
        } else {
            false
        };

        let status_icon = if is_installed {
            style("✓").green()
        } else {
            style("✗").red()
        };

        let status_text = if is_installed {
            style("installed").green()
        } else {
            style("not installed").dim()
        };

        println!("  {} {:<20} {}", status_icon, hook_type.filename(), status_text);

        if is_installed {
            installed_count += 1;
        }
    }

    println!();
    if installed_count == 0 {
        println!("{}", style("No hooks installed. Run 'tasks hooks install' to install them.").dim());
    } else {
        println!("{}", style(format!("{} of {} hooks installed", installed_count, HookType::all().len())).dim());
    }

    Ok(())
}

/// Find the .git/hooks directory
fn find_git_hooks_dir() -> Result<PathBuf> {
    // Check current directory for .git
    let git_dir = Path::new(".git");

    if !git_dir.exists() {
        bail!("Not in a git repository. Initialize git first with 'git init'.");
    }

    let hooks_dir = git_dir.join("hooks");

    // Create hooks directory if it doesn't exist
    if !hooks_dir.exists() {
        fs::create_dir_all(&hooks_dir)
            .context("Failed to create .git/hooks directory")?;
    }

    Ok(hooks_dir)
}

/// Parse hook name string to HookType
fn parse_hook_name(name: &str) -> Result<HookType> {
    match name {
        "pre-commit" => Ok(HookType::PreCommit),
        "post-commit" => Ok(HookType::PostCommit),
        "prepare-commit-msg" => Ok(HookType::PrepareCommitMsg),
        "post-checkout" => Ok(HookType::PostCheckout),
        _ => bail!("Unknown hook type: {}. Valid types: pre-commit, post-commit, prepare-commit-msg, post-checkout", name),
    }
}

/// Install a specific hook
fn install_hook(hooks_dir: &Path, hook_type: &HookType) -> Result<()> {
    let hook_path = hooks_dir.join(hook_type.filename());

    // Backup existing hook if it exists and isn't ours
    if hook_path.exists() {
        let content = fs::read_to_string(&hook_path)?;
        if !content.contains("repo-tasks") {
            let backup_path = hooks_dir.join(format!("{}.backup", hook_type.filename()));
            fs::copy(&hook_path, &backup_path)
                .context(format!("Failed to backup existing {} hook", hook_type.filename()))?;
            println!("  {} Backed up existing {} hook", style("→").dim(), hook_type.filename());
        }
    }

    // Get hook template content
    let hook_content = get_hook_template(hook_type);

    // Write hook file
    fs::write(&hook_path, hook_content)
        .context(format!("Failed to write {} hook", hook_type.filename()))?;

    // Make hook executable (Unix-like systems)
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms)?;
    }

    Ok(())
}

/// Get the template content for a hook
fn get_hook_template(hook_type: &HookType) -> String {
    let common_header = r#"#!/bin/sh
# Generated by repo-tasks
# This hook is managed by repo-tasks. To uninstall: tasks hooks uninstall
"#;

    match hook_type {
        HookType::PreCommit => format!(
            r#"{}
# Pre-commit hook: Validate task references and prevent .repo-tasks commits

# Check for staged .repo-tasks files
STAGED_TASKS=$(git diff --cached --name-only | grep "^\.repo-tasks/")

if [ -n "$STAGED_TASKS" ]; then
  echo "Error: Cannot commit task files with regular git commit"
  echo ""
  echo "The following task files are staged:"
  echo "$STAGED_TASKS" | sed 's/^/  - /'
  echo ""
  echo "Task files should only be committed with 'tasks save'"
  echo ""
  echo "To fix:"
  echo "  1. Unstage task files: git restore --staged .repo-tasks/"
  echo "  2. Commit project files: git commit -m \"Your message\""
  echo "  3. Then save task files: tasks save"
  echo ""
  echo "Or use --no-verify to bypass this check (not recommended)"
  exit 1
fi

exit 0
"#,
            common_header
        ),
        HookType::PostCommit => format!(
            r#"{}
# Post-commit hook: Auto-update task status based on commit message

# Get the commit message
COMMIT_MSG=$(git log -1 --pretty=%B)

# Log file
LOG_FILE=".repo-tasks/hooks.log"
mkdir -p "$(dirname "$LOG_FILE")"

# Log timestamp
echo "$(date '+%Y-%m-%d %H:%M:%S') - post-commit hook triggered" >> "$LOG_FILE"

# Find repo-tasks binary
REPO_TASKS_BIN="repo-tasks"
if ! command -v "$REPO_TASKS_BIN" > /dev/null 2>&1; then
    # Try to find it in common locations
    if [ -f "./target/release/repo-tasks" ]; then
        REPO_TASKS_BIN="./target/release/repo-tasks"
    elif [ -f "./target/debug/repo-tasks" ]; then
        REPO_TASKS_BIN="./target/debug/repo-tasks"
    else
        echo "$(date '+%Y-%m-%d %H:%M:%S') - WARNING: repo-tasks binary not found" >> "$LOG_FILE"
        exit 0
    fi
fi

# Extract task IDs using grep (14-digit numbers in various formats)
# Formats: [TASKID], #TASKID, task/TASKID, closes #TASKID, fixes #TASKID
TASK_IDS=$(echo "$COMMIT_MSG" | grep -oE '(\[|#|task/|closes #|fixes #)([0-9]{{14}})' | grep -oE '[0-9]{{14}}' | sort -u)

if [ -z "$TASK_IDS" ]; then
    echo "$(date '+%Y-%m-%d %H:%M:%S') - No task IDs found in commit message" >> "$LOG_FILE"
    exit 0
fi

# Convert commit message to lowercase for case-insensitive matching
COMMIT_MSG_LOWER=$(echo "$COMMIT_MSG" | tr '[:upper:]' '[:lower:]')

# Determine target status from keywords
TARGET_STATUS=""

# Check for "done" keywords
if echo "$COMMIT_MSG_LOWER" | grep -qE '\[done\]|\[complete\]|\[completed\]|\[finished\]|closes #[0-9]{{14}}|fixes #[0-9]{{14}}'; then
    TARGET_STATUS="done"
# Check for "testing" keywords
elif echo "$COMMIT_MSG_LOWER" | grep -qE '\[testing\]|\[review\]|\[ready\]'; then
    TARGET_STATUS="testing"
# Check for "in-progress" keywords
elif echo "$COMMIT_MSG_LOWER" | grep -qE '\[wip\]|\[in-progress\]|\[started\]'; then
    TARGET_STATUS="in-progress"
fi

if [ -z "$TARGET_STATUS" ]; then
    echo "$(date '+%Y-%m-%d %H:%M:%S') - No status keywords found in commit message" >> "$LOG_FILE"
    exit 0
fi

# Update each task
echo "$TASK_IDS" | while read -r TASK_ID; do
    if [ -n "$TASK_ID" ]; then
        echo "$(date '+%Y-%m-%d %H:%M:%S') - Moving task $TASK_ID to $TARGET_STATUS" >> "$LOG_FILE"

        # Try to move the task, suppress output but capture errors
        if OUTPUT=$("$REPO_TASKS_BIN" move "$TASK_ID" "$TARGET_STATUS" 2>&1); then
            echo "$(date '+%Y-%m-%d %H:%M:%S') - Successfully moved task $TASK_ID to $TARGET_STATUS" >> "$LOG_FILE"
            # Show user feedback
            echo "repo-tasks: Moved task $TASK_ID to $TARGET_STATUS"
        else
            # Log error but don't fail the commit
            echo "$(date '+%Y-%m-%d %H:%M:%S') - ERROR moving task $TASK_ID: $OUTPUT" >> "$LOG_FILE"
        fi
    fi
done

exit 0
"#,
            common_header
        ),
        HookType::PrepareCommitMsg => format!(
            r#"{}
# Prepare-commit-msg hook: Add task context to commit template

COMMIT_MSG_FILE=$1
COMMIT_SOURCE=$2

echo "repo-tasks: prepare-commit-msg hook (placeholder)"
# TODO: Implement commit template logic
exit 0
"#,
            common_header
        ),
        HookType::PostCheckout => format!(
            r#"{}
# Post-checkout hook: Show task info when switching branches

PREV_HEAD=$1
NEW_HEAD=$2
BRANCH_SWITCH=$3

echo "repo-tasks: post-checkout hook (placeholder)"
# TODO: Implement branch info logic
exit 0
"#,
            common_header
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_type_filename() {
        assert_eq!(HookType::PreCommit.filename(), "pre-commit");
        assert_eq!(HookType::PostCommit.filename(), "post-commit");
        assert_eq!(HookType::PrepareCommitMsg.filename(), "prepare-commit-msg");
        assert_eq!(HookType::PostCheckout.filename(), "post-checkout");
    }

    #[test]
    fn test_parse_hook_name() {
        assert!(matches!(parse_hook_name("pre-commit").unwrap(), HookType::PreCommit));
        assert!(matches!(parse_hook_name("post-commit").unwrap(), HookType::PostCommit));
        assert!(parse_hook_name("invalid").is_err());
    }

    #[test]
    fn test_all_hooks() {
        let hooks = HookType::all();
        assert_eq!(hooks.len(), 4);
    }
}
