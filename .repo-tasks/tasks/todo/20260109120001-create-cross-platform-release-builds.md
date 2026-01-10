---
ID: "20260109120001"
Title: Create cross-platform release builds
Priority: Critical
Tags:
  - phase-4
  - distribution
  - release
---

Set up cross-platform release builds for multiple platforms:

- macOS aarch64 (Apple Silicon)
- macOS x86_64 (Intel)
- Linux x86_64 (glibc)
- Linux x86_64 (musl - static)
- Windows x86_64

Consider using GitHub Actions matrix builds or cargo-cross for cross-compilation. Ensure all binaries are stripped and optimized (< 5MB target already met).

Reference: DESIGN.md Phase 4
