# Scripts

Utility scripts for repo-tasks.

## verify-release.sh

Verify the authenticity of a downloaded repo-tasks release binary using SHA256 checksums.

### Usage

```bash
./scripts/verify-release.sh <version> <artifact>
```

### Examples

**macOS Intel:**
```bash
# Download the binary
curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-apple-darwin.tar.gz

# Verify it
./scripts/verify-release.sh v0.1.0 repo-tasks-x86_64-apple-darwin.tar.gz
```

**macOS Apple Silicon:**
```bash
curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-apple-darwin.tar.gz
./scripts/verify-release.sh v0.1.0 repo-tasks-aarch64-apple-darwin.tar.gz
```

**Linux x86_64:**
```bash
curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-unknown-linux-gnu.tar.gz
./scripts/verify-release.sh v0.1.0 repo-tasks-x86_64-unknown-linux-gnu.tar.gz
```

**Windows:**
```powershell
# Download binary
Invoke-WebRequest -Uri "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-pc-windows-msvc.zip" -OutFile "repo-tasks-x86_64-pc-windows-msvc.zip"

# Download checksums
Invoke-WebRequest -Uri "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/checksums.txt" -OutFile "checksums.txt"

# Verify (PowerShell)
$expected = (Get-Content checksums.txt | Select-String "repo-tasks-x86_64-pc-windows-msvc.zip").ToString().Split()[0]
$actual = (Get-FileHash repo-tasks-x86_64-pc-windows-msvc.zip -Algorithm SHA256).Hash.ToLower()
if ($expected -eq $actual) { Write-Host "✓ Checksum verified" -ForegroundColor Green } else { Write-Host "✗ Checksum mismatch" -ForegroundColor Red }
```

### What it does

1. Downloads the `checksums.txt` file from the specified release
2. Extracts the expected SHA256 checksum for your artifact
3. Calculates the actual SHA256 checksum of your downloaded file
4. Compares them and reports success or failure

### Requirements

- `curl` - For downloading checksums
- `shasum` or `sha256sum` - For calculating checksums (built-in on macOS/Linux)

### Security

Always verify checksums when downloading binaries from the internet. This ensures:
- The file hasn't been corrupted during download
- The file hasn't been tampered with
- You're getting the authentic release from the repo-tasks project
