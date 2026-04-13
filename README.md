# nio

`nio` is a Rust-based CLI tool designed to simplify Git worktree management by enforcing a consistent naming convention and providing a fuzzy-searchable interface for switching between worktrees.

## Features

-   **`nio gtree create <branch>`**: Create a new worktree and branch. The worktree is created in a sibling directory following the convention `../[current-repo]_[branch]`.
-   **`nio gtree switch`**: Interactively select a worktree from a fuzzy-searchable list.
-   **`nio gtree remove <branch>`**: Remove a worktree based on its branch name.
-   **`nio init [shell]`**: Generate a shell initialization script for `cd` support and auto-completions.

## Getting Started

### Installation

1.  **Build the project**:
    ```bash
    cargo build --release
    ```

2.  **Move the binary to your PATH**:
    ```bash
    cp target/release/nio /usr/local/bin/nio
    ```

### Shell Integration

To enable the `cd` functionality (required for `nio gtree switch`) and auto-completions, add the following to your shell configuration file (e.g., `~/.zshrc`, `~/.bashrc`, or `~/.config/fish/config.fish`):

**Zsh / Bash**:
```bash
eval "$(nio init)"
```

**Fish**:
```fish
nio init fish | source
```

*Note: If `nio init` fails to detect your shell automatically, specify it as an argument (e.g., `nio init zsh`).*

## Development

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

### Local Dev Loop

1.  **Build**:
    ```bash
    cargo build
    ```

2.  **Run**:
    ```bash
    cargo run -- gtree create my-feature
    ```

3.  **Test**:
    ```bash
    cargo test
    ```

4.  **Lint**:
    ```bash
    cargo clippy
    ```

### Project Structure

-   `src/main.rs`: Entry point and command routing.
-   `src/cli.rs`: CLI definition using `clap`.
-   `src/gtree/`: Logic for the `gtree` subcommand (create, switch, remove).
-   `src/git.rs`: Git command wrappers and porcelain output parsing.
-   `src/init/`: Logic for shell detection and initialization script generation.
-   `src/utils.rs`: Path utilities and validation.

## License

[MIT](LICENSE)
