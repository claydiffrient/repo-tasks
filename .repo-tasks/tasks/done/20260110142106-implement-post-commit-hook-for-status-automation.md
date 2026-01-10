---
ID: '20260110142106'
Title: Implement post-commit hook for status automation
Priority: High
Tags:
- git-hooks
- automation
- post-commit
---

Create post-commit hook that automatically updates task status based on commit message keywords. Parse commit message, extract task ID and status keywords, call 'tasks move' to update status. Handle errors gracefully (never block commits). Log actions to .repo-tasks/hooks.log.