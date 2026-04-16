# Design Spec: `nio` - Git Worktree Management CLI

A Rust-based CLI tool to streamline the management and navigation of git worktrees using a specific naming convention and interactive selection.

## 1. Problem Statement
Managing git worktrees can involve repetitive manual path calculations and navigation. `nio` simplifies this by enforcing a consistent naming convention (`../[current-repo]_[branch]`) and providing a fuzzy-searchable interface for switching between them.

## 2. Core Features
- **Create**: Add a new worktree and branch with a standardized name and location.
- **Switch**: Interactively select a worktree from a list and navigate to its directory.
- **Remove**: Cleanly remove a worktree based on its branch name and the standard location.

## 3. Architecture & Technologies
- **Language**: Rust
- **CLI Framework**: `clap` (v4) for command-line argument parsing.
- **Interactive UI**: `dialoguer` with `FuzzySelect` for an `fzf`-like selection experience.
- **Git Interaction**: `std::process::Command` to execute `git worktree` commands.
- **Path Manipulation**: `std::path` and `std::env` for reliable directory calculations across different environments.

## 4. Command Specifications

### `nio gtree create [branch-name]`
- **Logic**:
    1. Get the current working directory name (`current-folder-name`).
    2. Construct the target path: `../[current-folder-name]_[branch-name]`.
    3. Execute: `git worktree add -b [branch-name] [target-path]`.
- **Validation**: Ensure we are inside a git repository before proceeding.

### `nio gtree switch`
- **Logic**:
    1. Run `git worktree list --porcelain` to get a machine-readable list of worktrees.
    2. Parse the output to extract the directory path and branch name for each worktree.
    3. Present a `FuzzySelect` menu to the user.
    4. On selection, print the absolute path of the selected worktree to `stdout`.
- **Note**: This command requires a shell wrapper (see section 5) to perform the `cd` action in the user's shell.

### `nio gtree remove [branch-name]`
- **Logic**:
    1. Get the current working directory name (`current-folder-name`).
    2. Construct the target path: `../[current-folder-name]_[branch-name]`.
    3. Execute: `git worktree remove [target-path]`.
- **Validation**: Confirm the worktree path exists before attempting removal.

## 5. Shell Integration
To enable `cd` functionality, users will add a small function to their shell configuration (`.zshrc`, `.bashrc`, etc.):

```bash
nio() {
    if [[ "$1" == "gtree" && "$2" == "switch" ]]; then
        local target=$(command nio gtree switch)
        if [[ -n "$target" ]]; then
            cd "$target"
        fi
    else
        command nio "$@"
    fi
}
```

## 6. Project Structure
- `src/main.rs`: CLI entry point and command routing.
- `src/gtree/mod.rs`: Logic for the `gtree` subcommand.
- `src/git.rs`: Helper functions for interacting with the `git` CLI.
- `src/utils.rs`: Path and directory utilities.

## 7. Testing Strategy
- **Unit Tests**: Test path construction and git output parsing logic.
- **Integration Tests**: Mock the `git` command to verify that `nio` calls it with the expected arguments.
- **Manual Verification**: Test the full flow (create, switch, remove) within a real git repository.
