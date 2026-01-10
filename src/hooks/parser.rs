use regex::Regex;
use std::sync::OnceLock;

/// Parsed information from a commit message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInfo {
    /// Task IDs found in the commit message
    pub task_ids: Vec<String>,
    /// Status keywords found with their target status
    pub status_keywords: Vec<StatusKeyword>,
}

/// A status keyword found in a commit message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusKeyword {
    /// The keyword that was matched
    pub keyword: String,
    /// The target status for this keyword
    pub target_status: String,
}

impl CommitInfo {
    /// Create a new empty CommitInfo
    pub fn new() -> Self {
        CommitInfo {
            task_ids: Vec::new(),
            status_keywords: Vec::new(),
        }
    }

    /// Check if any task IDs were found
    pub fn has_task_ids(&self) -> bool {
        !self.task_ids.is_empty()
    }

    /// Check if any status keywords were found
    pub fn has_status_keywords(&self) -> bool {
        !self.status_keywords.is_empty()
    }

    /// Get the first task ID if available
    pub fn first_task_id(&self) -> Option<&str> {
        self.task_ids.first().map(|s| s.as_str())
    }

    /// Get the first status keyword if available
    pub fn first_status_keyword(&self) -> Option<&StatusKeyword> {
        self.status_keywords.first()
    }
}

impl Default for CommitInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a commit message and extract task IDs and status keywords
///
/// Supports multiple task ID formats:
/// - `[TASKID]` - e.g., [20260110142106]
/// - `#TASKID` - e.g., #20260110142106
/// - `task/TASKID` - e.g., task/20260110142106
/// - `closes #TASKID` - e.g., closes #20260110142106
/// - `fixes #TASKID` - e.g., fixes #20260110142106
///
/// Detects status keywords:
/// - `[done]`, `[complete]`, `[finished]`, `closes`, `fixes` → done
/// - `[testing]`, `[review]`, `[ready]` → testing
/// - `[wip]`, `[in-progress]`, `[started]` → in-progress
pub fn parse_commit_message(message: &str) -> CommitInfo {
    let mut info = CommitInfo::new();

    // Extract task IDs
    info.task_ids = extract_task_ids(message);

    // Extract status keywords
    info.status_keywords = extract_status_keywords(message);

    info
}

/// Extract all task IDs from a commit message
fn extract_task_ids(message: &str) -> Vec<String> {
    static TASK_ID_REGEX: OnceLock<Regex> = OnceLock::new();

    let regex = TASK_ID_REGEX.get_or_init(|| {
        // Match task IDs in various formats:
        // [20260110142106], #20260110142106, task/20260110142106
        // closes #20260110142106, fixes #20260110142106
        Regex::new(r"(?:(?:closes|fixes)\s+)?(?:\[|#|task/)(\d{14})(?:\])?").unwrap()
    });

    let mut task_ids = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for cap in regex.captures_iter(message) {
        if let Some(id) = cap.get(1) {
            let task_id = id.as_str().to_string();
            // Only add unique task IDs
            if seen.insert(task_id.clone()) {
                task_ids.push(task_id);
            }
        }
    }

    task_ids
}

/// Extract status keywords from a commit message
fn extract_status_keywords(message: &str) -> Vec<StatusKeyword> {
    let mut keywords = Vec::new();
    let message_lower = message.to_lowercase();

    // Check for "done" keywords
    let done_patterns = [
        r"\[done\]",
        r"\[complete\]",
        r"\[completed\]",
        r"\[finished\]",
        r"\bcloses\s+#\d{14}",
        r"\bfixes\s+#\d{14}",
    ];

    static DONE_REGEX: OnceLock<Vec<Regex>> = OnceLock::new();
    let regexes = DONE_REGEX.get_or_init(|| {
        done_patterns.iter()
            .map(|p| Regex::new(p).unwrap())
            .collect()
    });

    for (i, regex) in regexes.iter().enumerate() {
        if regex.is_match(&message_lower) {
            let keyword_name = match i {
                0 => "done",
                1 | 2 => "complete",
                3 => "finished",
                4 => "closes",
                5 => "fixes",
                _ => "done",
            };
            keywords.push(StatusKeyword {
                keyword: keyword_name.to_string(),
                target_status: "done".to_string(),
            });
            break; // Only add one "done" keyword
        }
    }

    // Check for "testing" keywords
    let testing_patterns = [r"\[testing\]", r"\[review\]", r"\[ready\]"];

    for pattern in &testing_patterns {
        if Regex::new(pattern).unwrap().is_match(&message_lower) {
            let keyword_name = if pattern.contains("testing") {
                "testing"
            } else if pattern.contains("review") {
                "review"
            } else {
                "ready"
            };
            keywords.push(StatusKeyword {
                keyword: keyword_name.to_string(),
                target_status: "testing".to_string(),
            });
            break; // Only add one "testing" keyword
        }
    }

    // Check for "in-progress" keywords
    let in_progress_patterns = [r"\[wip\]", r"\[in-progress\]", r"\[started\]"];

    for pattern in &in_progress_patterns {
        if Regex::new(pattern).unwrap().is_match(&message_lower) {
            let keyword_name = if pattern.contains("wip") {
                "wip"
            } else if pattern.contains("in-progress") {
                "in-progress"
            } else {
                "started"
            };
            keywords.push(StatusKeyword {
                keyword: keyword_name.to_string(),
                target_status: "in-progress".to_string(),
            });
            break; // Only add one "in-progress" keyword
        }
    }

    keywords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_message() {
        let info = parse_commit_message("");
        assert_eq!(info.task_ids.len(), 0);
        assert_eq!(info.status_keywords.len(), 0);
        assert!(!info.has_task_ids());
        assert!(!info.has_status_keywords());
    }

    #[test]
    fn test_extract_task_id_bracket_format() {
        let info = parse_commit_message("[20260110142106] Add new feature");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
        assert_eq!(info.first_task_id(), Some("20260110142106"));
    }

    #[test]
    fn test_extract_task_id_hash_format() {
        let info = parse_commit_message("Fix bug #20260110142106");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
    }

    #[test]
    fn test_extract_task_id_task_prefix_format() {
        let info = parse_commit_message("Update task/20260110142106 implementation");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
    }

    #[test]
    fn test_extract_task_id_closes_format() {
        let info = parse_commit_message("Implement feature closes #20260110142106");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
    }

    #[test]
    fn test_extract_task_id_fixes_format() {
        let info = parse_commit_message("Fix critical bug fixes #20260110142106");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
    }

    #[test]
    fn test_extract_multiple_task_ids() {
        let info = parse_commit_message("[20260110142106] Related to #20260109120000");
        assert_eq!(info.task_ids.len(), 2);
        assert!(info.task_ids.contains(&"20260110142106".to_string()));
        assert!(info.task_ids.contains(&"20260109120000".to_string()));
    }

    #[test]
    fn test_deduplicate_task_ids() {
        let info = parse_commit_message("[20260110142106] Fix #20260110142106");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
    }

    #[test]
    fn test_extract_done_keyword() {
        let info = parse_commit_message("[20260110142106] Complete feature [done]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "done");
        assert_eq!(info.status_keywords[0].keyword, "done");
    }

    #[test]
    fn test_extract_complete_keyword() {
        let info = parse_commit_message("Feature implementation [complete]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "done");
        assert_eq!(info.status_keywords[0].keyword, "complete");
    }

    #[test]
    fn test_extract_finished_keyword() {
        let info = parse_commit_message("All tests passing [finished]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "done");
    }

    #[test]
    fn test_extract_closes_keyword() {
        let info = parse_commit_message("Implement auth closes #20260110142106");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "done");
        assert_eq!(info.status_keywords[0].keyword, "closes");
    }

    #[test]
    fn test_extract_testing_keyword() {
        let info = parse_commit_message("Ready for QA [testing]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "testing");
        assert_eq!(info.status_keywords[0].keyword, "testing");
    }

    #[test]
    fn test_extract_review_keyword() {
        let info = parse_commit_message("Ready for code review [review]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "testing");
    }

    #[test]
    fn test_extract_wip_keyword() {
        let info = parse_commit_message("Work in progress [wip]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "in-progress");
        assert_eq!(info.status_keywords[0].keyword, "wip");
    }

    #[test]
    fn test_extract_in_progress_keyword() {
        let info = parse_commit_message("Starting implementation [in-progress]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "in-progress");
    }

    #[test]
    fn test_combined_task_id_and_status() {
        let info = parse_commit_message("[20260110142106] Implement auth [done]");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "done");
        assert!(info.has_task_ids());
        assert!(info.has_status_keywords());
    }

    #[test]
    fn test_case_insensitive_keywords() {
        let info = parse_commit_message("Feature complete [DONE]");
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "done");
    }

    #[test]
    fn test_no_false_positives_for_partial_matches() {
        let info = parse_commit_message("Update documentation for task 123");
        assert_eq!(info.task_ids.len(), 0); // "123" is not a valid 14-digit task ID
    }

    #[test]
    fn test_multiline_commit_message() {
        let message = r#"[20260110142106] Implement user authentication

This commit adds JWT-based authentication to the API.
Includes login, logout, and token refresh.

[done]"#;
        let info = parse_commit_message(message);
        assert_eq!(info.task_ids, vec!["20260110142106"]);
        assert_eq!(info.status_keywords.len(), 1);
        assert_eq!(info.status_keywords[0].target_status, "done");
    }

    #[test]
    fn test_real_world_commit_examples() {
        // Example 1: Feature with task ID and done marker
        let info = parse_commit_message("[20260110142106] Add user profile page [done]");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
        assert_eq!(info.status_keywords[0].target_status, "done");

        // Example 2: Bug fix with closes
        let info = parse_commit_message("Fix login bug closes #20260109120000");
        assert_eq!(info.task_ids, vec!["20260109120000"]);
        assert_eq!(info.status_keywords[0].target_status, "done");

        // Example 3: WIP commit
        let info = parse_commit_message("#20260110142106 Work on API endpoints [wip]");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
        assert_eq!(info.status_keywords[0].target_status, "in-progress");

        // Example 4: Ready for review
        let info = parse_commit_message("task/20260110142106 Refactor auth module [review]");
        assert_eq!(info.task_ids, vec!["20260110142106"]);
        assert_eq!(info.status_keywords[0].target_status, "testing");
    }
}
