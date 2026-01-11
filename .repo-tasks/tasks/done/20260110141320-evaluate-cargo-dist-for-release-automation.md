---
ID: '20260110141320'
Title: Evaluate cargo-dist for release automation
Priority: Medium
Tags:
- research
- devops
- cd
---

Research cargo-dist as a modern release automation tool. Evaluate if it fits our needs for: cross-platform builds, GitHub releases, binary distribution, and installer generation. Document pros/cons and make a decision on whether to use it or build custom workflow.

## Evaluation Summary

**Decision: Keep custom GitHub Actions workflow**

After comprehensive evaluation, cargo-dist is a powerful tool but our custom workflow already meets our needs effectively. Key findings:

### cargo-dist Advantages
- Auto-generates installers (shell, PowerShell, MSI, Homebrew)
- Comprehensive all-in-one solution
- Professionally maintained by Axo
- Good ecosystem integration (release-plz, git-cliff)

### Current Workflow Advantages
- Full transparency and control
- Already implemented and working
- Includes smoke tests
- Easier to customize
- No migration cost

### When to Reconsider
- Need Windows MSI installers
- Want auto-generated Homebrew formulas
- Need npm package distribution
- Managing multiple Rust projects

See `docs/cargo-dist-evaluation.md` for full analysis.