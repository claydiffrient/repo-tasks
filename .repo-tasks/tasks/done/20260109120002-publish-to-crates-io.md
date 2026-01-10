---
ID: '20260109120002'
Title: Publish to crates.io
Priority: High
Tags:
- phase-4
- distribution
- release
---

Publish repo-tasks to crates.io for easy installation via `cargo install repo-tasks`.

Steps:
1. Ensure Cargo.toml metadata is complete (description, repository, license, keywords, categories)
2. Verify README.md is included
3. Test `cargo package` to check what will be published
4. Run `cargo publish --dry-run` first
5. Publish with `cargo publish`
6. Update README with crates.io badge and installation instructions

Reference: DESIGN.md Phase 4