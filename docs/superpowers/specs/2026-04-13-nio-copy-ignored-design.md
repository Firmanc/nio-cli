# Design Spec: `nio gtree create --copy-ignored`

Add an optional flag to the `gtree create` command to automatically copy git-ignored files and directories (e.g., `.env`, `node_modules`, `target`) from the current repository to the newly created worktree.

## 1. Problem Statement
When creating a new git worktree, git only checks out tracked files. However, modern development workflows often rely on ignored files like environment variables (`.env`) or large dependency folders (`node_modules`) that are not committed. Manually copying these files to every new worktree is repetitive and error-prone.

## 2. Core Features
- **Optional Flag**: Introduce `--copy-ignored` (short `-c`) to the `gtree create` command.
- **Ignored File Detection**: Automatically identify all files and directories ignored by git in the current worktree.
- **Recursive Copying**: Recursively copy the identified items to the same relative path in the new worktree.
- **Error Resilience**: Log any individual copy failures but continue with the remaining items.

## 3. Architecture & Technologies
- **Ignored Item Detection**: Use `git clean -ndX` to get a machine-readable list of ignored items.
- **Recursive Copying**: Use the `fs_extra` crate for robust, cross-platform recursive directory copying.
- **CLI Framework**: Update `clap` v4 definitions in `src/cli.rs`.
- **Command Routing**: Update `src/gtree.rs` to orchestrate the copying process after a successful `git worktree add`.

## 4. Implementation Details

### `src/git.rs`
- Add `list_ignored_items() -> Result<Vec<String>, GitError>`:
    - Executes `git clean -ndX`.
    - Parses lines starting with `Would remove ` to extract the relative path of the ignored item.
    - Handles edge cases like empty output or git errors.

### `src/gtree.rs`
- Update `create_worktree(..., copy_ignored: bool)`:
    - If `copy_ignored` is true, call `git::list_ignored_items()`.
    - Iterate through the returned paths.
    - Construct absolute source and destination paths.
    - Use `fs_extra::copy_items` to copy files and directories recursively.
    - Print status messages to `stderr`.

### `src/cli.rs`
- Update `GtreeCommands::Create` to include the `copy_ignored` flag.

## 5. Testing Strategy
- **Unit Tests**:
    - Mock the output of `git clean -ndX` and verify the parsing logic in `src/git.rs`.
- **Integration Tests**:
    - Create a temporary git repository with ignored files.
    - Run `nio gtree create --copy-ignored`.
    - Verify that the ignored files are present in the new worktree at the correct paths.
- **Manual Verification**:
    - Test with common ignored items like `.env` and `node_modules` (empty or small) in a real repository.

## 6. Safety and Performance
- **Large Directories**: Copying massive directories (like a fully populated `node_modules` or `target`) might take time. Status messages will keep the user informed.
- **Overwrites**: `fs_extra` will be configured to handle existing files safely (though they shouldn't exist in a fresh worktree).
