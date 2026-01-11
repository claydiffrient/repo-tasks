#!/usr/bin/env node

/**
 * Post-install check to verify repo-tasks CLI is installed.
 * This script is designed to provide helpful guidance rather than fail the installation.
 */

import { execSync } from "child_process";

// ANSI color codes
const YELLOW = "\x1b[33m";
const BLUE = "\x1b[34m";
const RESET = "\x1b[0m";
const BOLD = "\x1b[1m";

function checkBinary(): boolean {
  try {
    execSync("command -v repo-tasks", { stdio: "ignore" });
    return true;
  } catch {
    try {
      execSync("command -v tasks", { stdio: "ignore" });
      return true;
    } catch {
      return false;
    }
  }
}

function printWarning() {
  console.error(`
${YELLOW}${BOLD}⚠️  repo-tasks CLI not found${RESET}

${BOLD}The @claydiffrient/repo-tasks-mcp-server package requires the repo-tasks CLI to be installed.${RESET}

Install it using one of these methods:

  ${BLUE}•${RESET} ${BOLD}Homebrew:${RESET}  brew install claydiffrient/tap/repo-tasks
  ${BLUE}•${RESET} ${BOLD}Cargo:${RESET}     cargo install repo-tasks
  ${BLUE}•${RESET} ${BOLD}From source:${RESET}
    git clone https://github.com/claydiffrient/repo-tasks.git
    cd repo-tasks
    cargo install --path .

${BOLD}For more information:${RESET} ${BLUE}https://github.com/claydiffrient/repo-tasks${RESET}
`);
}

// Main check
if (!checkBinary()) {
  printWarning();
}

// Exit with 0 (success) so we don't fail the npm install
process.exit(0);
