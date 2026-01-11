# Homebrew Formula for repo-tasks

This directory contains the Homebrew formula for `repo-tasks`.

## For Users

### Installation

Once the tap is published, users can install repo-tasks with:

```bash
# Add the tap
brew tap claydiffrient/repo-tasks

# Install repo-tasks
brew install repo-tasks
```

Or install directly:

```bash
brew install claydiffrient/repo-tasks/repo-tasks
```

### Updating

```bash
brew update
brew upgrade repo-tasks
```

## For Maintainers

### Setting Up the Tap Repository

To publish this formula as a Homebrew tap:

1. **Create a new GitHub repository named `homebrew-repo-tasks`**
   ```bash
   # On GitHub, create: claydiffrient/homebrew-repo-tasks
   ```

2. **Initialize the repository with the formula**
   ```bash
   git clone https://github.com/claydiffrient/homebrew-repo-tasks.git
   cd homebrew-repo-tasks
   mkdir -p Formula
   cp /path/to/repo-tasks/Formula/repo-tasks.rb Formula/
   git add Formula/repo-tasks.rb
   git commit -m "Add repo-tasks formula"
   git push origin main
   ```

3. **Users can now tap your formula**
   ```bash
   brew tap claydiffrient/repo-tasks
   brew install repo-tasks
   ```

### Updating the Formula for New Releases

When releasing a new version:

1. **Update version number** in `repo-tasks.rb`

2. **Update URLs** to point to the new release tag

3. **Update SHA256 checksums** for each binary:
   ```bash
   # Download the release artifacts
   curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-apple-darwin.tar.gz
   curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-apple-darwin.tar.gz
   curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-unknown-linux-gnu.tar.gz
   curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-unknown-linux-gnu.tar.gz

   # Calculate SHA256 checksums
   shasum -a 256 repo-tasks-x86_64-apple-darwin.tar.gz
   shasum -a 256 repo-tasks-aarch64-apple-darwin.tar.gz
   shasum -a 256 repo-tasks-x86_64-unknown-linux-gnu.tar.gz
   shasum -a 256 repo-tasks-aarch64-unknown-linux-gnu.tar.gz
   ```

4. **Replace the SHA256 placeholders** in the formula

5. **Test locally**:
   ```bash
   brew install --build-from-source ./Formula/repo-tasks.rb
   brew test repo-tasks
   brew uninstall repo-tasks
   ```

6. **Commit and push** to the tap repository

### Formula Structure

The formula uses architecture detection to provide the correct binary:

- **macOS Intel**: `repo-tasks-x86_64-apple-darwin.tar.gz`
- **macOS Apple Silicon**: `repo-tasks-aarch64-apple-darwin.tar.gz`
- **Linux x86_64**: `repo-tasks-x86_64-unknown-linux-gnu.tar.gz`
- **Linux ARM64**: `repo-tasks-aarch64-unknown-linux-gnu.tar.gz`

### Testing the Formula

```bash
# Audit the formula
brew audit --strict --online repo-tasks

# Test installation
brew install --verbose --debug ./Formula/repo-tasks.rb

# Test the installed binary
brew test repo-tasks

# Uninstall
brew uninstall repo-tasks
```

## Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [How to Create and Maintain a Tap](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- [Adding Software to Homebrew](https://docs.brew.sh/Adding-Software-to-Homebrew)
