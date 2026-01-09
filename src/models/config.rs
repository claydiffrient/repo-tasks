use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project_name: String,
    pub statuses: Vec<String>,
    pub priorities: Vec<String>,
    #[serde(default)]
    pub auto_commit: bool,
}

impl Config {
    /// Create a new config with default values
    pub fn default(project_name: Option<String>) -> Self {
        let project_name = project_name.unwrap_or_else(|| {
            std::env::current_dir()
                .ok()
                .and_then(|p| {
                    p.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                })
                .unwrap_or_else(|| "my-project".to_string())
        });

        Config {
            project_name,
            statuses: vec![
                "todo".to_string(),
                "in-progress".to_string(),
                "testing".to_string(),
                "done".to_string(),
            ],
            priorities: vec![
                "Low".to_string(),
                "Medium".to_string(),
                "High".to_string(),
                "Critical".to_string(),
            ],
            auto_commit: false,
        }
    }

    /// Load config from .repo-tasks/config.json
    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        let content = fs::read_to_string(&path)
            .context(format!("Failed to read config file: {}", path.display()))?;

        serde_json::from_str(&content)
            .context("Failed to parse config.json")
    }

    /// Write config to .repo-tasks/config.json
    pub fn write(&self) -> Result<()> {
        let path = Self::config_path();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory: {}", parent.display()))?;
        }

        // Serialize to pretty JSON
        let content = serde_json::to_string_pretty(&self)
            .context("Failed to serialize config")?;

        fs::write(&path, content)
            .context(format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Check if repo-tasks is initialized (config file exists)
    pub fn is_initialized() -> bool {
        Self::config_path().exists()
    }

    /// Get the path to the config file
    fn config_path() -> PathBuf {
        PathBuf::from(".repo-tasks").join("config.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default(Some("test-project".to_string()));

        assert_eq!(config.project_name, "test-project");
        assert_eq!(config.statuses.len(), 4);
        assert_eq!(config.priorities.len(), 4);
        assert!(!config.auto_commit);
    }

    #[test]
    fn test_default_config_no_name() {
        let config = Config::default(None);

        // Should use current directory name or fallback
        assert!(!config.project_name.is_empty());
    }

    #[test]
    fn test_serialize_deserialize() {
        let config = Config::default(Some("test".to_string()));
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(config.project_name, deserialized.project_name);
        assert_eq!(config.statuses, deserialized.statuses);
        assert_eq!(config.priorities, deserialized.priorities);
    }
}
