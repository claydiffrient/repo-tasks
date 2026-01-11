---
ID: '20260109120003'
Title: Create Homebrew formula and tap
Priority: High
Tags:
- phase-4
- distribution
- homebrew
- macos
---

Create a Homebrew formula for easy installation on macOS via `brew install repo-tasks`.

Options:
1. Submit to homebrew-core (requires significant downloads/popularity)
2. Create personal tap at claydiffrient/homebrew-repo-tasks

For personal tap:
- Create new GitHub repo: homebrew-repo-tasks
- Add Formula/repo-tasks.rb with proper bottle definitions for both architectures
- Test installation locally
- Update README with brew tap instructions

Reference: DESIGN.md Phase 4, https://docs.brew.sh/Formula-Cookbook

## Implementation Complete

✅ **Created Homebrew Formula** (`Formula/repo-tasks.rb`)
- Supports macOS Intel (x86_64) and Apple Silicon (aarch64)
- Supports Linux x86_64 and ARM64
- Uses pre-built binaries from GitHub releases for fast installation
- Includes basic smoke tests (--version, --help)

✅ **Created Formula README** (`Formula/README.md`)
- Instructions for users to install
- Complete maintainer guide for setting up the tap
- Step-by-step guide for updating formula for new releases
- Instructions for calculating SHA256 checksums
- Testing and auditing commands

✅ **Updated Main README**
- Added Homebrew installation section
- Instructions for tapping and installing
- Note about availability after first release

## Next Steps (After v0.1.0 Release)

1. **Create the tap repository on GitHub:**
   ```bash
   # Create repo: claydiffrient/homebrew-repo-tasks
   # Then initialize it:
   git clone https://github.com/claydiffrient/homebrew-repo-tasks.git
   cd homebrew-repo-tasks
   mkdir -p Formula
   cp Formula/repo-tasks.rb Formula/
   ```

2. **Update SHA256 checksums in formula:**
   ```bash
   # After v0.1.0 release, download binaries and calculate checksums:
   curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-apple-darwin.tar.gz
   shasum -a 256 repo-tasks-x86_64-apple-darwin.tar.gz
   # Repeat for all 4 architectures, update formula
   ```

3. **Test the formula locally:**
   ```bash
   brew install --build-from-source ./Formula/repo-tasks.rb
   brew test repo-tasks
   ```

4. **Publish to GitHub:**
   ```bash
   cd homebrew-repo-tasks
   git add Formula/repo-tasks.rb
   git commit -m "Add repo-tasks formula v0.1.0"
   git push origin main
   ```

5. **Users can then install with:**
   ```bash
   brew tap claydiffrient/repo-tasks
   brew install repo-tasks
   ```

## Resources

- [Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [How to Create and Maintain a Tap](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- [Adding Software to Homebrew](https://docs.brew.sh/Adding-Software-to-Homebrew)