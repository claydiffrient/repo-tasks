---
ID: '20260110142106'
Title: Create commit message parser
Priority: High
Tags:
- git-hooks
- parsing
- core
---

Implement parser to extract task IDs and keywords from commit messages. Support multiple formats: [TASKID], #TASKID, task/TASKID. Detect keywords like [done], [testing], [wip], closes #TASKID, fixes #TASKID. Unit tests for various commit message formats.