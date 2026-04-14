# Design Spec: `nio gtree list`

Add a `list` (alias `ls`) subcommand to `gtree` to provide a simple, readable list of all active git worktrees.

## 1. Problem Statement
While `nio gtree switch` provides an interactive way to select a worktree, users often need a quick, non-interactive way to view all active worktrees and their locations directly in the terminal for scripting or quick reference.

## 2. Core Features
- **Simple List**: Output a space-aligned list of branch names and their corresponding absolute paths.
- **Alias**: Support `ls` as a shorthand for `list`.
- **Non-Interactive**: Unlike `switch`, this command performs no action and requires no user input.

## 3. Architecture & Technologies
- **Git Interaction**: Reuse the existing `git::list_worktrees()` function in `src/git.rs`.
- **CLI Framework**: Update `clap` v4 definitions in `src/cli.rs`.
- **Formatting**: Use standard `println!` with tab-based or fixed-width alignment for clarity.

## 4. Implementation Details

### `src/cli.rs`
- Add `List` (alias `ls`) to `GtreeCommands` enum.

### `src/gtree.rs`
- Add `list_worktrees() -> Result<()>`:
    - Calls `git::list_worktrees()`.
    - Iterates through the `Vec<Worktree>`.
    - Formats and prints each worktree to `stdout`.
    - Example output:
      ```text
      main    /path/to/repo
      feat-x  /path/to/repo_feat-x
      ```

### `src/main.rs`
- Route the `List` command in `gtree::handle_command`.

## 5. Testing Strategy
- **Unit Tests**:
    - The underlying parser in `src/git.rs` is already tested.
- **Manual Verification**:
    - Run `nio gtree list` and `nio gtree ls` in a repository with multiple worktrees.
    - Verify that the output is correctly aligned and readable.
