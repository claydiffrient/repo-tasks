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

- ✅ macOS aarch64 (Apple Silicon) - **Working**
- ✅ macOS x86_64 (Intel) - **Working**
- ✅ Linux x86_64 (glibc) - **Working**
- ⬜ Linux x86_64 (musl - static) - **TODO**
- ✅ Windows x86_64 - **Working**

Consider using GitHub Actions matrix builds or cargo-cross for cross-compilation. Ensure all binaries are stripped and optimized (< 5MB target already met).

Reference: DESIGN.md Phase 4

## Progress (2026-01-10)

### ✅ Completed

**Cross-Platform Build Infrastructure** (commit c652076)

Fixed OpenSSL compilation issues by enabling `vendored-openssl` feature for git2 crate. This allows OpenSSL to be compiled from source during the build process, enabling cross-compilation without system dependencies.

**Working Platforms:**
- ✅ macOS aarch64 (Apple Silicon) - Build time: ~3 min
- ✅ macOS x86_64 (Intel) - Build time: ~3.5 min
- ✅ Linux x86_64 (glibc) - Build time: ~2.5 min
- ✅ Windows x86_64 (MSVC) - Build time: ~4 min

All binaries are automatically stripped and optimized via `profile.release` settings in Cargo.toml.

**CI Status:** All build release jobs passing (run 20874496180)

**Artifacts Generated:**
- `repo-tasks-x86_64-unknown-linux-gnu`
- `repo-tasks-aarch64-apple-darwin`
- `repo-tasks-x86_64-apple-darwin`
- `repo-tasks-x86_64-pc-windows-msvc`

### ⬜ Remaining

1. **Add Linux musl target** - Static binary for maximum portability
   - Target: `x86_64-unknown-linux-musl`
   - Requires: musl-gcc in CI environment
   - Benefit: Single binary that works on any Linux distro

2. **Optimize artifact naming** - Add version numbers to filenames
   - Current: `repo-tasks-<target>`
   - Proposed: `repo-tasks-v0.1.0-<target>`

3. **Add checksums** - Generate SHA256 checksums for each binary
   - For security verification
   - Standard practice for releases

4. **Test binaries** - Automated smoke tests for each platform
   - Verify binary runs and shows help
   - Check version output

5. **Compression** - Consider compressing artifacts (tar.gz, zip)
   - Smaller downloads
   - Grouped with checksums
