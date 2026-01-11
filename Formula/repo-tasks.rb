class RepoTasks < Formula
  desc "Fast, file-based task management for git repositories"
  homepage "https://github.com/claydiffrient/repo-tasks"
  version "0.1.0"
  license "MIT OR Apache-2.0"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-apple-darwin.tar.gz"
      sha256 "bf1f4a473ddab3ae22fef92bed8f192833f1d0da1d55378c44f481fcaa026117"
    else
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-apple-darwin.tar.gz"
      sha256 "90de473a7c6937e4857e42f027fa810b9645e7e9653cd61edf1b29521cfdf004"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "05b030405d957e6362553e2a440afb0ac8d42277d00489e3be9d884fb73abc35"
    else
      url "https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "62d1d285a8446b87b0c895e16350a697039a541900a5edc9233f12dcfc706f71"
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
