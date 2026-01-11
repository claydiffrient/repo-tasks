# cargo-dist Evaluation

**Date:** January 10, 2026
**Task:** [20260110141320] Evaluate cargo-dist for release automation
**Status:** Research Complete

## Executive Summary

After evaluating cargo-dist against our current custom GitHub Actions workflow, we recommend **keeping the custom workflow** for now. While cargo-dist is a powerful tool with many features, our custom solution already meets our needs effectively and provides more transparency and control.

## What is cargo-dist?

[cargo-dist](https://github.com/axodotdev/cargo-dist) is a comprehensive Rust application packaging and distribution tool developed by Axo. It aims to automate the entire release process from building binaries to distributing them across platforms.

### Key Features

1. **Cross-Platform Builds**
   - Supports multiple target platforms (macOS, Linux, Windows)
   - Built-in cross-compilation via cargo-zigbuild and cargo-xwin
   - Support for static linking (musl) on Linux
   - Generates both x86_64 and ARM64 (aarch64) binaries

2. **Installer Generation**
   - Shell and PowerShell installers with checksum validation
   - MSI installers for Windows (requires WiX v3)
   - Homebrew formula support
   - npm installer support
   - Automatic dependency detection for package managers

3. **CI/CD Integration**
   - Auto-generates GitHub Actions workflows
   - Self-hosting: push a git tag to trigger releases
   - Integrates with release-plz for version management
   - Changelog generation support

4. **Distribution Features**
   - Creates GitHub releases automatically
   - Hosts artifacts
   - Publishes to package managers
   - Announcement capabilities

## Current Workflow Capabilities

Our custom `.github/workflows/release.yml` already provides:

✅ Cross-platform builds (5 targets: macOS x86_64/ARM64, Linux x86_64/ARM64, Windows x86_64)
✅ GitHub release creation
✅ Binary packaging (tar.gz/zip)
✅ SHA256 checksum generation and verification
✅ Smoke tests (--version, --help)
✅ Platform-specific installation instructions
✅ Combined checksums.txt file
✅ Release asset uploads

## Comparison

| Feature | cargo-dist | Current Workflow | Winner |
|---------|-----------|------------------|--------|
| **Cross-platform builds** | ✅ Yes (via cargo-zigbuild/xwin) | ✅ Yes (native + cross) | Tie |
| **Binary packaging** | ✅ Automatic | ✅ Manual (tar.gz/zip) | cargo-dist |
| **Checksums** | ✅ Built-in | ✅ Custom implementation | Tie |
| **GitHub releases** | ✅ Automatic | ✅ Custom implementation | Tie |
| **Smoke tests** | ❌ No | ✅ Yes | Current |
| **Installers (shell/PowerShell)** | ✅ Auto-generated | ❌ No | cargo-dist |
| **MSI installer (Windows)** | ✅ Yes (requires WiX) | ❌ No | cargo-dist |
| **Homebrew formula** | ✅ Auto-generated | ❌ No (planned separately) | cargo-dist |
| **npm installer** | ✅ Yes | ❌ No | cargo-dist |
| **Customization** | ⚠️ Limited to config | ✅ Full control | Current |
| **Transparency** | ⚠️ Generated code | ✅ Explicit workflow | Current |
| **Learning curve** | ⚠️ Moderate | ✅ Standard GitHub Actions | Current |
| **Maintenance** | ✅ Maintained by Axo | ⚠️ Self-maintained | cargo-dist |
| **Setup complexity** | ⚠️ `dist init` + config | ✅ Already done | Current |

## Pros of cargo-dist

1. **Comprehensive Solution**: All-in-one tool covering builds, packaging, and distribution
2. **Installer Generation**: Automatically creates shell, PowerShell, MSI, and package manager installers
3. **Maintained by Professionals**: Axo maintains the tool with regular updates
4. **Package Manager Integration**: Built-in Homebrew and npm support
5. **Modern Tooling**: Uses cargo-zigbuild for better cross-compilation
6. **Self-Hosting**: Simple git tag push triggers full pipeline
7. **Ecosystem Integration**: Works well with release-plz, git-cliff, etc.

## Cons of cargo-dist

1. **Additional Dependency**: Adds another tool to learn and maintain
2. **Less Transparent**: Generated workflows are harder to understand and debug
3. **Configuration-Based**: Limited customization compared to writing workflows directly
4. **Setup Overhead**: Requires running `dist init` and configuring Cargo.toml
5. **Windows MSI Requires WiX**: Extra toolchain needed (though pre-installed in GitHub CI)
6. **Overkill for Simple Needs**: We don't need all features (npm, MSI, etc.)
7. **Lock-in Risk**: Dependent on Axo's maintenance and direction

## Recommendation: Keep Custom Workflow

### Reasoning

1. **Current Workflow is Sufficient**: We already have everything we need:
   - Cross-platform builds working
   - Checksums implemented
   - GitHub releases automated
   - Clear, maintainable code

2. **Transparency Matters**: Our custom workflow is explicit and easy to understand. Anyone can read `release.yml` and know exactly what happens during a release.

3. **Flexibility**: We can easily add smoke tests, custom validation, or any other steps. cargo-dist would require working within its configuration model.

4. **No Migration Cost**: Switching would require:
   - Running `dist init`
   - Updating Cargo.toml
   - Testing generated workflows
   - Removing our custom workflow
   - Re-learning the release process

5. **Feature Overlap**: Most cargo-dist benefits (installers, Homebrew) are nice-to-have but not essential. We can add them incrementally if needed.

6. **Simple is Better**: Our workflow is ~200 lines of straightforward YAML. cargo-dist adds abstraction that we don't need yet.

### When to Reconsider

Consider cargo-dist in the future if:
- We need Windows MSI installers
- We want auto-generated Homebrew formulas
- We need npm package distribution
- We start maintaining multiple Rust projects (reusability)
- We want auto-generated shell installers with complex logic
- Team lacks GitHub Actions expertise (cargo-dist abstracts complexity)

## Action Items

- [x] Evaluate cargo-dist features
- [x] Compare with current workflow
- [x] Document findings
- [x] Make recommendation

**Decision:** Continue with custom GitHub Actions workflow for now. Monitor cargo-dist development for future reconsideration.

## References

- [cargo-dist GitHub Repository](https://github.com/axodotdev/cargo-dist)
- [Fully Automated Releases for Rust Projects](https://blog.orhun.dev/automated-rust-releases/)
- [cargo-dist on crates.io](https://crates.io/crates/cargo-dist)
- [cargo-dist Releases](https://github.com/axodotdev/cargo-dist/releases)
- [cargo-dist MSI Installer Documentation](https://github.com/axodotdev/cargo-dist/blob/main/book/src/installers/msi.md)

## Appendix: Sample cargo-dist Configuration

If we were to adopt cargo-dist, the setup would look like:

```toml
# In Cargo.toml
[workspace.metadata.dist]
# The preferred cargo-dist version to use in CI (Cargo.toml SemVer syntax)
cargo-dist-version = "0.26.0"
# CI backends to support
ci = ["github"]
# The installers to generate for each app
installers = ["shell", "powershell"]
# Target platforms to build apps for
targets = [
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc"
]
# Whether to auto-include files like README and LICENSE
auto-includes = true
```

Then run `cargo dist init` to generate `.github/workflows/release.yml`.
