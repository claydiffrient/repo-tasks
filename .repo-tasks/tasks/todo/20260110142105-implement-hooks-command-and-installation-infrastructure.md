---
ID: '20260110142105'
Title: Implement hooks command and installation infrastructure
Priority: High
Tags:
- git-hooks
- core
- infrastructure
---

Create 'tasks hooks' command with subcommands: install, uninstall, list. Implement hook template management (embed templates in binary or as resources). Handle hook installation to .git/hooks/, backup existing hooks, make executable. Track installed hooks in registry file.