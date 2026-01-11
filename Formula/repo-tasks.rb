class RepoTasks < Formula
  desc "Fast, file-based task management for git repositories"
  homepage "https://github.com/claydiffrient/repo-tasks"
  version "0.1.0"
  license "MIT OR Apache-2.0"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_X86_64"
    else
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_AARCH64"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_LINUX_X86_64"
    else
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_LINUX_AARCH64"
    end
  end

  def install
    bin.install "repo-tasks"
  end

  test do
    system "#{bin}/repo-tasks", "--version"
    system "#{bin}/repo-tasks", "--help"
  end
end
