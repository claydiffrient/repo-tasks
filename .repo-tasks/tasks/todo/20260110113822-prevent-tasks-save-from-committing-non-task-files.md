---
ID: '20260110113822'
Title: Prevent tasks save from committing non-task files
Priority: High
Tags:
- cli
- safety
- validation
---

Add validation to the 'tasks save' command to ensure it only commits files within the .repo-tasks directory.

## Problem

Currently, 'tasks save' will commit any staged files, not just task files in .repo-tasks/. This violates the intended workflow where:
- Project code changes should be committed with regular git commands
- Task file changes should be committed with 'tasks save'
- These two types of commits should never mix

## Proposed Solution

Before committing, check all staged files and:
1. Verify all staged files are within .repo-tasks/
2. If any files are outside .repo-tasks/, abort with error
3. Show clear error message listing the offending files
4. Suggest unstaging non-task files or using git commit directly

## Implementation

In src/commands/save.rs:
- Use git2 to get list of staged files
- Check each file path starts with '.repo-tasks/'
- If validation fails, show error and abort
- If validation passes, proceed with commit

## Example Error Message

```
Error: Cannot commit non-task files with 'tasks save'

The following staged files are outside .repo-tasks/:
  - Cargo.toml
  - src/main.rs

To fix this:
  1. Commit project files separately: git commit -m "Your message"
  2. Then use 'tasks save' for task files only

Or unstage non-task files: git restore --staged <file>
```

## Testing

- Stage only .repo-tasks files → should work
- Stage mixed files → should error
- Stage no files → should handle gracefully
- Show helpful error with file list