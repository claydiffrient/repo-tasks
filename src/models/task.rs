use anyhow::{Context, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    #[serde(rename = "Blocks", skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<String>>,

    #[serde(rename = "DependsOn", skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,

    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip)]
    pub body: String,

    #[serde(skip)]
    pub slug: String,

    #[serde(skip)]
    pub status: String,
}

impl Task {
    /// Create a new task with the given title and priority
    pub fn new(title: String, priority: String) -> Self {
        let id = Self::generate_id();
        let slug = Self::generate_slug(&title);

        Task {
            id,
            title,
            priority: Some(priority),
            blocks: None,
            depends_on: None,
            tags: None,
            body: String::new(),
            slug,
            status: "todo".to_string(),
        }
    }

    /// Generate a unique ID based on current timestamp
    pub fn generate_id() -> String {
        Local::now().format("%Y%m%d%H%M%S").to_string()
    }

    /// Generate a slug from the title
    pub fn generate_slug(title: &str) -> String {
        slugify(title)
    }

    /// Get the file path for this task in the given status directory
    pub fn file_path(&self, status: &str) -> PathBuf {
        PathBuf::from(".repo-tasks")
            .join("tasks")
            .join(status)
            .join(format!("{}-{}.md", self.id, self.slug))
    }

    /// Parse frontmatter and body from markdown content
    pub fn parse_frontmatter(content: &str) -> Result<(Self, String)> {
        // Split content by frontmatter delimiters
        let parts: Vec<&str> = content.splitn(3, "---").collect();

        if parts.len() < 3 {
            anyhow::bail!("Invalid task format: missing frontmatter delimiters");
        }

        // Parse YAML frontmatter (skip first empty part)
        let frontmatter = parts[1].trim();
        let mut task: Task =
            serde_yaml::from_str(frontmatter).context("Failed to parse task frontmatter")?;

        // Extract body
        let body = parts[2].trim().to_string();
        task.body = body.clone();

        Ok((task, body))
    }

    /// Load a task from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context(format!("Failed to read task file: {}", path.display()))?;

        let (mut task, _) = Self::parse_frontmatter(&content)?;

        // Extract status from directory path
        if let Some(parent) = path.parent() {
            if let Some(status) = parent.file_name() {
                task.status = status.to_string_lossy().to_string();
            }
        }

        // Extract slug from filename
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            if let Some(slug_part) = filename_str.strip_suffix(".md") {
                if let Some((_, slug)) = slug_part.split_once('-') {
                    task.slug = slug.to_string();
                }
            }
        }

        Ok(task)
    }

    /// Save the task to a file
    pub fn to_file(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {}", parent.display()))?;
        }

        // Serialize frontmatter
        let frontmatter =
            serde_yaml::to_string(&self).context("Failed to serialize task frontmatter")?;

        // Construct full content
        let content = format!("---\n{}---\n\n{}", frontmatter, self.body);

        // Write to file
        fs::write(path, content)
            .context(format!("Failed to write task file: {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(
            Task::generate_slug("Create Requirements Document"),
            "create-requirements-document"
        );
        assert_eq!(
            Task::generate_slug("Add new feature: User Authentication"),
            "add-new-feature-user-authentication"
        );
    }

    #[test]
    fn test_generate_id() {
        let id = Task::generate_id();

        // IDs should be numeric strings
        assert!(id.chars().all(|c| c.is_numeric()));
        assert_eq!(id.len(), 14); // YYYYMMDDHHmmSS format
    }

    #[test]
    fn test_new_task() {
        let task = Task::new("Test Task".to_string(), "High".to_string());

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.priority, Some("High".to_string()));
        assert_eq!(task.slug, "test-task");
        assert_eq!(task.status, "todo");
        assert_eq!(task.body, "");
    }

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
ID: "20260108120000"
Title: Test Task
Priority: High
---

This is the task body."#;

        let (task, body) = Task::parse_frontmatter(content).unwrap();

        assert_eq!(task.id, "20260108120000");
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.priority, Some("High".to_string()));
        assert_eq!(body, "This is the task body.");
    }
}
