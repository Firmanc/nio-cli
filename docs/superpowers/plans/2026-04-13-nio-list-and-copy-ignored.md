# nio - List and Copy Ignored Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the `nio gtree list` command and the `--copy-ignored` flag for `nio gtree create`.

**Architecture:** 
- The `list` command will fetch worktrees using the existing `git::list_worktrees` function and print them in a formatted list.
- The `--copy-ignored` flag will use `git clean -ndX` to identify ignored files and directories, and the `fs_extra` crate to recursively copy them to the new worktree.

**Tech Stack:** Rust, `clap` (v4), `fs_extra`

---

### Task 1: Implement `gtree list` Command

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/gtree.rs`

- [ ] **Step 1: Add `List` command to `src/cli.rs`**

```rust
// src/cli.rs
// ...
#[derive(Subcommand)]
pub enum GtreeCommands {
    /// Create a new worktree and branch
    Create {
        /// The name of the new branch and worktree suffix
        branch_name: String,
    },
    /// Interactively select a worktree to switch to
    Switch,
    /// List all git worktrees
    #[command(alias = "ls")]
    List,
    /// Remove an existing worktree by branch name
    Remove {
        /// The name of the branch to remove the worktree for
        branch_name: String,
    },
}
```

- [ ] **Step 2: Implement `list_worktrees` in `src/gtree.rs`**

```rust
// src/gtree.rs
// ...
pub fn handle_command(command: &GtreeCommands, current_dir: &Path) -> Result<()> {
    match command {
        GtreeCommands::Create { branch_name } => create_worktree(current_dir, branch_name)?,
        GtreeCommands::Switch => switch_worktree()?,
        GtreeCommands::List => list_worktrees()?, // Add this
        GtreeCommands::Remove { branch_name } => remove_worktree(current_dir, branch_name)?,
    }
    Ok(())
}

// ...

fn list_worktrees() -> Result<()> {
    let worktrees = git::list_worktrees().context("Failed to list worktrees")?;
    
    if worktrees.is_empty() {
        eprintln!("No worktrees found.");
        return Ok(());
    }

    // Find the longest branch name for alignment
    let max_branch_len = worktrees.iter()
        .map(|w| w.branch.len())
        .max()
        .unwrap_or(0);

    for w in worktrees {
        println!("{:<width$}  {}", w.branch, w.path, width = max_branch_len);
    }

    Ok(())
}
```

- [ ] **Step 3: Verify compilation and output**

Run: `cargo build`
Run: `cargo run -- gtree list`
Expected: Output showing all worktrees (at least the current one).

- [ ] **Step 4: Commit**

```bash
git add src/cli.rs src/gtree.rs
git commit -m "feat: implement gtree list command"
```

### Task 2: Setup `fs_extra` and Update CLI for `--copy-ignored`

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/cli.rs`

- [ ] **Step 1: Add `fs_extra` to `Cargo.toml`**

Run: `cargo add fs_extra`

- [ ] **Step 2: Add `copy_ignored` flag to `GtreeCommands::Create` in `src/cli.rs`**

```rust
// src/cli.rs
// ...
#[derive(Subcommand)]
pub enum GtreeCommands {
    /// Create a new worktree and branch
    Create {
        /// The name of the new branch and worktree suffix
        branch_name: String,
        /// Copy git-ignored files (e.g., .env, node_modules) to the new worktree
        #[arg(short, long)]
        copy_ignored: bool,
    },
    // ...
}
```

- [ ] **Step 3: Verify compilation**

Run: `cargo check`
Expected: PASS (with a few warnings in `gtree.rs` about mismatched `create_worktree` signature)

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml src/cli.rs
git commit -m "chore: add fs_extra dependency and update gtree create cli"
```

### Task 3: Implement Ignored Items Detection (`src/git.rs`)

**Files:**
- Modify: `src/git.rs`
- Test: `src/git.rs` (inline test)

- [ ] **Step 1: Implement `list_ignored_items` in `src/git.rs`**

```rust
// src/git.rs
// ...
pub fn list_ignored_items() -> Result<Vec<String>, GitError> {
    let output = run_git_command(&["clean", "-ndX"])?;
    Ok(parse_ignored_list(&output))
}

fn parse_ignored_list(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            // "Would remove path/to/file"
            line.strip_prefix("Would remove ").map(|s| s.trim().to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // ...
    #[test]
    fn test_parse_ignored_list() {
        let output = "\
Would remove .env
Would remove node_modules/
Would remove target/
";
        let expected = vec![
            ".env".to_string(),
            "node_modules/".to_string(),
            "target/".to_string(),
        ];
        assert_eq!(parse_ignored_list(output), expected);
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test git`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add src/git.rs
git commit -m "feat: implement ignored items detection"
```

### Task 4: Implement Copy Logic in `gtree create`

**Files:**
- Modify: `src/gtree.rs`

- [ ] **Step 1: Implement `copy_ignored_items` in `src/gtree.rs`**

```rust
// src/gtree.rs
use std::fs;
use fs_extra::dir::{copy, CopyOptions};
// ...

pub fn handle_command(command: &GtreeCommands, current_dir: &Path) -> Result<()> {
    match command {
        GtreeCommands::Create { branch_name, copy_ignored } => {
            create_worktree(current_dir, branch_name, *copy_ignored)?
        },
        // ...
    }
    Ok(())
}

fn create_worktree(current_dir: &Path, branch_name: &str, copy_ignored: bool) -> Result<()> {
    let target_path = utils::build_worktree_path(current_dir, branch_name)
        .context("Failed to build target worktree path")?;
    
    git::add_worktree(&target_path, branch_name)
        .context("Failed to execute git worktree add")?;
        
    eprintln!("Successfully created worktree for branch '{}' at {:?}", branch_name, target_path);

    if copy_ignored {
        copy_ignored_items(current_dir, &target_path)?;
    }

    Ok(())
}

fn copy_ignored_items(source_dir: &Path, target_dir: &Path) -> Result<()> {
    let ignored_items = git::list_ignored_items().context("Failed to list ignored items")?;
    
    if ignored_items.is_empty() {
        eprintln!("No ignored files to copy.");
        return Ok(());
    }

    eprintln!("Copying ignored files...");
    
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    for item in ignored_items {
        let source_path = source_dir.join(&item);
        if !source_path.exists() {
            continue;
        }

        // We use copy_items which handles both files and directories
        let items_to_copy = vec![source_path];
        if let Err(e) = fs_extra::copy_items(&items_to_copy, target_dir, &options) {
            eprintln!("Warning: Failed to copy {}: {}", item, e);
        } else {
            eprintln!("  Copied {}", item);
        }
    }

    Ok(())
}
// ...
```

- [ ] **Step 2: Verify compilation**

Run: `cargo build`
Expected: PASS

- [ ] **Step 3: Manual Verification**

1. Create a dummy ignored file: `echo "SECRET=123" > .env.test`
2. Add it to `.gitignore`: `echo ".env.test" >> .gitignore`
3. Run: `cargo run -- gtree create test-copy -c`
4. Check if `../[repo]_test-copy/.env.test` exists and has the correct content.

- [ ] **Step 4: Commit**

```bash
git add src/gtree.rs
git commit -m "feat: implement --copy-ignored flag for gtree create"
```
