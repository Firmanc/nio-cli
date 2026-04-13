#!/bin/bash
set -e

# nio - Git Worktree Management CLI Installation Script

# Define variables
BIN_NAME="nio"
RELEASE_DIR="target/release"
TARGET_BIN_DIR="/usr/local/bin"
LOCAL_BIN_DIR="$HOME/.local/bin"

# 1. Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "Error: 'cargo' is not installed. Please install Rust first: https://www.rust-lang.org/tools/install"
    exit 1
fi

# 2. Build the project in release mode
echo "Building $BIN_NAME in release mode..."
cargo build --release

# 3. Determine installation directory
if [ -w "$TARGET_BIN_DIR" ]; then
    INSTALL_DIR="$TARGET_BIN_DIR"
    SUDO=""
elif [ -d "$LOCAL_BIN_DIR" ] && [[ ":$PATH:" == *":$LOCAL_BIN_DIR:"* ]]; then
    INSTALL_DIR="$LOCAL_BIN_DIR"
    SUDO=""
else
    # Default to /usr/local/bin and ask for sudo if not writable
    INSTALL_DIR="$TARGET_BIN_DIR"
    SUDO="sudo"
fi

# 4. Install the binary
echo "Installing $BIN_NAME to $INSTALL_DIR..."
if [ -n "$SUDO" ]; then
    echo "You may be prompted for your password to install to $INSTALL_DIR."
    $SUDO cp "$RELEASE_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
else
    cp "$RELEASE_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
fi

# 5. Shell Integration Instructions
echo "--------------------------------------------------------"
echo "Successfully installed $BIN_NAME to $INSTALL_DIR!"
echo ""
echo "To enable 'cd' support and auto-completions, add this to your shell profile:"
echo ""
echo "  # Zsh / Bash (~/.zshrc or ~/.bashrc)"
echo "  eval \"\$(nio init)\""
echo ""
echo "  # Fish (~/.config/fish/config.fish)"
echo "  nio init fish | source"
echo ""
echo "Restart your shell or source your config to apply changes."
echo "--------------------------------------------------------"
