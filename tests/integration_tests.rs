use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("repo-tasks").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Fast task management for git repositories"));
}

#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("repo-tasks").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("init")
        .arg("--project-name")
        .arg("test-project")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized repo-tasks"));

    // Verify structure was created
    assert!(temp_dir.path().join(".repo-tasks/config.json").exists());
    assert!(temp_dir.path().join(".repo-tasks/tasks/todo").exists());
    assert!(temp_dir.path().join(".repo-tasks/tasks/in-progress").exists());
    assert!(temp_dir.path().join(".repo-tasks/tasks/testing").exists());
    assert!(temp_dir.path().join(".repo-tasks/tasks/done").exists());
}

#[test]
fn test_list_empty() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    Command::cargo_bin("repo-tasks")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    // List should show no tasks
    let mut cmd = Command::cargo_bin("repo-tasks").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks"));
}

#[test]
fn test_show_nonexistent_task() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    Command::cargo_bin("repo-tasks")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    // Try to show non-existent task
    let mut cmd = Command::cargo_bin("repo-tasks").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("show")
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Task not found"));
}

#[test]
fn test_move_task() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    Command::cargo_bin("repo-tasks")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    // Create a test task file
    let task_path = temp_dir
        .path()
        .join(".repo-tasks/tasks/todo/20260108000001-test-task.md");
    fs::write(
        &task_path,
        r#"---
ID: "20260108000001"
Title: Test Task
Priority: High
---

Test body
"#,
    )
    .unwrap();

    // Move the task
    let mut cmd = Command::cargo_bin("repo-tasks").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("move")
        .arg("test-task")
        .arg("done")
        .assert()
        .success()
        .stdout(predicate::str::contains("Moved task"));

    // Verify task was moved
    assert!(!task_path.exists());
    assert!(temp_dir
        .path()
        .join(".repo-tasks/tasks/done/20260108000001-test-task.md")
        .exists());
}

#[test]
fn test_search() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    Command::cargo_bin("repo-tasks")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    // Create a test task file with searchable content
    fs::write(
        temp_dir
            .path()
            .join(".repo-tasks/tasks/todo/20260108000001-search-test.md"),
        r#"---
ID: "20260108000001"
Title: Search Test Task
Priority: High
---

This is a unique_search_term that should be found
"#,
    )
    .unwrap();

    // Search for the unique term
    let mut cmd = Command::cargo_bin("repo-tasks").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("search")
        .arg("unique_search_term")
        .assert()
        .success()
        .stdout(predicate::str::contains("search-test"));
}

#[test]
fn test_list_with_filter() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    Command::cargo_bin("repo-tasks")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    // Create tasks with different priorities
    fs::write(
        temp_dir
            .path()
            .join(".repo-tasks/tasks/todo/20260108000001-high-priority.md"),
        r#"---
ID: "20260108000001"
Title: High Priority Task
Priority: High
---
"#,
    )
    .unwrap();

    fs::write(
        temp_dir
            .path()
            .join(".repo-tasks/tasks/todo/20260108000002-low-priority.md"),
        r#"---
ID: "20260108000002"
Title: Low Priority Task
Priority: Low
---
"#,
    )
    .unwrap();

    // List with priority filter
    let mut cmd = Command::cargo_bin("repo-tasks").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("list")
        .arg("--priority")
        .arg("High")
        .assert()
        .success()
        .stdout(predicate::str::contains("high-priority"))
        .stdout(predicate::str::contains("low-priority").not());
}
