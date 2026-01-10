---
ID: '20260110142109'
Title: Implement post-checkout hook
Priority: Low
Tags:
- git-hooks
- enhancement
- checkout-info
---

Create post-checkout hook that displays task information when switching branches. Extract task ID from branch name, show task details (title, status, priority). Suggest moving task to in-progress if currently in todo. Improve workflow awareness.