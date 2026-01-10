---
ID: '20260110142108'
Title: Implement prepare-commit-msg hook
Priority: Medium
Tags:
- git-hooks
- enhancement
- commit-template
---

Create prepare-commit-msg hook that auto-injects task references into commit message template. Detect task ID from branch name (task/TASKID pattern). Add task context to commit template (title, status). Enhance developer experience when committing.