---
ID: '20260110141321'
Title: Create GitHub releases workflow
Priority: High
Tags:
- devops
- cd
- github-actions
---

Create .github/workflows/release.yml that triggers on version tags (v*). Should create GitHub releases, build binaries for all platforms, and attach them as release assets. Include smoke tests for built binaries.