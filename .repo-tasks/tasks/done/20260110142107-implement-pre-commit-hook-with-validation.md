---
ID: '20260110142107'
Title: Implement pre-commit hook with validation
Priority: High
Tags:
- git-hooks
- validation
- pre-commit
- safety
---

Create pre-commit hook that validates task references and prevents .repo-tasks files from being committed via regular git. Warn if task reference missing, validate task exists, prevent commits if task already done (configurable). Block commits containing .repo-tasks files with helpful error message directing users to use 'tasks save'.