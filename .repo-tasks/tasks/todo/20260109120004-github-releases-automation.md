---
ID: "20260109120004"
Title: Set up GitHub releases automation
Priority: Medium
Tags:
  - phase-4
  - ci-cd
  - automation
---

Automate GitHub releases with binary attachments:

1. Create `.github/workflows/release.yml` triggered on version tags (v*)
2. Build binaries for all target platforms (reuse cross-platform build config)
3. Create GitHub release with changelog
4. Attach compiled binaries as release assets
5. Optionally: Generate checksums for verification
6. Consider using cargo-dist or similar tools for automation

This will enable users to download pre-built binaries directly from GitHub releases.

Reference: DESIGN.md Phase 4
