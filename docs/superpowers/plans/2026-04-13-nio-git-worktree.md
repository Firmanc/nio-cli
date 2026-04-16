# nio - Git Worktree Management CLI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust CLI tool (`nio`) that simplifies git worktree management by enforcing a consistent naming convention (`../[current-repo]_[branch]`) and providing a fuzzy-searchable interface for switching between worktrees.

**Architecture:** The CLI is built with `clap` (v4) for argument parsing and `dialoguer` for an `fzf`-like interactive selection menu. It interacts with Git by shelling out to `std::process::Command` to execute `git worktree` commands, using standard library path manipulation to calculate target directories based on the current repository's folder name. The application logic is split into command routing (`main.rs`, `gtree/mod.rs`), git interaction (`git.rs`), and path utilities (`utils.rs`).

**Tech Stack:** Rust, `clap` (v4), `dialoguer`

---

### Task 1: Project Setup and Dependencies

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`

- [ ] **Step 1: Initialize the project and add dependencies**

```bash
cargo init --bin
cargo add clap --features derive
cargo add dialoguer
cargo add thiserror
cargo add anyhow
```

- [ ] **Step 2: Verify compilation**

```bash
cargo build
```
Expected: PASS (Compiles successfully)

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs
git commit -m "chore: initialize project and add dependencies"
```

### Task 2: Implement Path Utilities (`src/utils.rs`)

**Files:**
- Create: `src/utils.rs`
- Modify: `src/main.rs` (to declare module)
- Test: `src/utils.rs` (inline module)

- [ ] **Step 1: Declare the `utils` module in `src/main.rs`**

```rust
// src/main.rs
mod utils;

fn main() {
    println!("Hello, world!");
}
```

- [ ] **Step 2: Write tests for path utilities**

```rust
// src/utils.rs
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("Failed to get current directory")]
    CurrentDirFailed,
    #[error("Failed to get file name from path")]
    NoFileName,
}

pub fn get_current_dir_name(current_dir: &Path) -> Result<String, UtilError> {
    current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
        .ok_or(UtilError::NoFileName)
}

pub fn build_worktree_path(current_dir: &Path, branch_name: &str) -> Result<PathBuf, UtilError> {
    let dir_name = get_current_dir_name(current_dir)?;
    let target_dir_name = format!("{}_{}", dir_name, branch_name);
    let mut target_path = current_dir.parent().ok_or(UtilError::NoFileName)?.to_path_buf();
    target_path.push(target_dir_name);
    Ok(target_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_dir_name() {
        let path = Path::new("/path/to/my-repo");
        assert_eq!(get_current_dir_name(path).unwrap(), "my-repo");
    }

    #[test]
    fn test_build_worktree_path() {
        let current_dir = Path::new("/path/to/my-repo");
        let branch = "feature-x";
        let expected = PathBuf::from("/path/to/my-repo_feature-x");
        assert_eq!(build_worktree_path(current_dir, branch).unwrap(), expected);
    }
}
```

- [ ] **Step 3: Run tests to verify they pass**

```bash
cargo test -- lib
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/utils.rs
git commit -m "feat: implement path construction utilities"
```

### Task 3: Implement Git Interaction Layer (`src/git.rs`)

**Files:**
- Create: `src/git.rs`
- Modify: `src/main.rs` (to declare module)
- Test: `src/git.rs` (inline module)

- [ ] **Step 1: Declare the `git` module in `src/main.rs`**

```rust
// src/main.rs
mod utils;
mod git;

fn main() {
    println!("Hello, world!");
}
```

- [ ] **Step 2: Implement git wrapper and list parsing**

```rust
// src/git.rs
use std::process::Command;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Failed to execute git command: {0}")]
    ExecutionError(#[from] std::io::Error),
    #[error("Git command failed with status: {0}")]
    CommandFailed(String),
    #[error("Failed to parse worktree list output")]
    ParseError,
}

#[derive(Debug, PartialEq)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
}

pub fn run_git_command(args: &[&str]) -> Result<String, GitError> {
    let output = Command::new("git")
        .args(args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(GitError::CommandFailed(stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn add_worktree(target_path: &Path, branch_name: &str) -> Result<(), GitError> {
    let path_str = target_path.to_str().unwrap_or_default();
    run_git_command(&["worktree", "add", "-b", branch_name, path_str])?;
    Ok(())
}

pub fn remove_worktree(target_path: &Path) -> Result<(), GitError> {
    let path_str = target_path.to_str().unwrap_or_default();
    run_git_command(&["worktree", "remove", path_str])?;
    Ok(())
}

pub fn list_worktrees() -> Result<Vec<Worktree>, GitError> {
    let output = run_git_command(&["worktree", "list", "--porcelain"])?;
    parse_worktree_list(&output)
}

fn parse_worktree_list(output: &str) -> Result<Vec<Worktree>, GitError> {
    let mut worktrees = Vec::new();
    let mut current_path = String::new();

    for line in output.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            current_path = path.to_string();
        } else if let Some(branch) = line.strip_prefix("branch ") {
            // Strip the full ref path to just get the branch name
            let branch_name = branch.replace("refs/heads/", "");
            if !current_path.is_empty() {
                worktrees.push(Worktree {
                    path: current_path.clone(),
                    branch: branch_name,
                });
                current_path.clear();
            }
        }
    }

    Ok(worktrees)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_worktree_list() {
        let output = "\
worktree /path/to/my-repo
branch refs/heads/main

worktree /path/to/my-repo_feature-x
branch refs/heads/feature-x

worktree /path/to/my-repo_fix-bug
branch refs/heads/fix-bug
";
        let expected = vec![
            Worktree {
                path: "/path/to/my-repo".to_string(),
                branch: "main".to_string(),
            },
            Worktree {
                path: "/path/to/my-repo_feature-x".to_string(),
                branch: "feature-x".to_string(),
            },
            Worktree {
                path: "/path/to/my-repo_fix-bug".to_string(),
                branch: "fix-bug".to_string(),
            },
        ];
        assert_eq!(parse_worktree_list(output).unwrap(), expected);
    }
}
```

- [ ] **Step 3: Run tests to verify they pass**

```bash
cargo test -- lib
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/git.rs
git commit -m "feat: implement git command wrappers and parser"
```

### Task 4: CLI Definition (`src/cli.rs`)

**Files:**
- Create: `src/cli.rs`
- Modify: `src/main.rs` (to declare module)

- [ ] **Step 1: Declare the `cli` module in `src/main.rs`**

```rust
// src/main.rs
mod utils;
mod git;
mod cli;

fn main() {
    println!("Hello, world!");
}
```

- [ ] **Step 2: Implement `clap` CLI structure**

```rust
// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nio")]
#[command(about = "Git Worktree Management CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage git worktrees
    Gtree {
        #[command(subcommand)]
        command: GtreeCommands,
    },
}

#[derive(Subcommand)]
pub enum GtreeCommands {
    /// Create a new worktree and branch
    Create {
        /// The name of the new branch and worktree suffix
        branch_name: String,
    },
    /// Interactively select a worktree to switch to
    Switch,
    /// Remove an existing worktree by branch name
    Remove {
        /// The name of the branch to remove the worktree for
        branch_name: String,
    },
}
```

- [ ] **Step 3: Verify compilation**

```bash
cargo check
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/cli.rs
git commit -m "feat: define CLI structure with clap"
```

### Task 5: Implement `gtree` Command Logic (`src/gtree.rs`)

**Files:**
- Create: `src/gtree.rs`
- Modify: `src/main.rs` (to declare module and route commands)

- [ ] **Step 1: Declare the `gtree` module in `src/main.rs`**

```rust
// src/main.rs
mod utils;
mod git;
mod cli;
mod gtree;

use clap::Parser;
use cli::{Cli, Commands};
use std::env;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gtree { command } => {
            let current_dir = env::current_dir()?;
            gtree::handle_command(command, &current_dir)?;
        }
    }

    Ok(())
}
```

- [ ] **Step 2: Implement command handling logic**

```rust
// src/gtree.rs
use std::path::Path;
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use crate::cli::GtreeCommands;
use crate::{git, utils};

pub fn handle_command(command: &GtreeCommands, current_dir: &Path) -> Result<()> {
    match command {
        GtreeCommands::Create { branch_name } => create_worktree(current_dir, branch_name)?,
        GtreeCommands::Switch => switch_worktree()?,
        GtreeCommands::Remove { branch_name } => remove_worktree(current_dir, branch_name)?,
    }
    Ok(())
}

fn create_worktree(current_dir: &Path, branch_name: &str) -> Result<()> {
    let target_path = utils::build_worktree_path(current_dir, branch_name)
        .context("Failed to build target worktree path")?;
    
    git::add_worktree(&target_path, branch_name)
        .context("Failed to execute git worktree add")?;
        
    eprintln!("Successfully created worktree for branch '{}' at {:?}", branch_name, target_path);
    Ok(())
}

fn switch_worktree() -> Result<()> {
    let worktrees = git::list_worktrees().context("Failed to list worktrees")?;
    
    if worktrees.is_empty() {
        eprintln!("No worktrees found.");
        return Ok(());
    }

    let items: Vec<String> = worktrees.iter()
        .map(|w| format!("{} ({})", w.branch, w.path))
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a worktree to switch to")
        .default(0)
        .items(&items)
        .interact_opt()?;

    if let Some(index) = selection {
        let selected_path = &worktrees[index].path;
        // Print the absolute path to stdout for the shell wrapper to catch
        println!("{}", selected_path);
    } else {
        eprintln!("Selection cancelled.");
    }

    Ok(())
}

fn remove_worktree(current_dir: &Path, branch_name: &str) -> Result<()> {
    let target_path = utils::build_worktree_path(current_dir, branch_name)
        .context("Failed to build target worktree path")?;
        
    if !target_path.exists() {
        anyhow::bail!("Worktree path does not exist: {:?}", target_path);
    }

    git::remove_worktree(&target_path)
        .context("Failed to execute git worktree remove")?;
        
    eprintln!("Successfully removed worktree for branch '{}' at {:?}", branch_name, target_path);
    Ok(())
}
```

- [ ] **Step 3: Verify compilation**

```bash
cargo build
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/main.rs src/gtree.rs
git commit -m "feat: implement create, switch, and remove logic for gtree command"
```
