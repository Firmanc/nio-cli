// src/init/mod.rs
use crate::cli::Shell;
use std::env;

pub fn detect_shell() -> Shell {
    let shell_path = env::var("SHELL").unwrap_or_default();
    detect_from_path(&shell_path)
}

fn detect_from_path(path: &str) -> Shell {
    if path.ends_with("/zsh") {
        Shell::Zsh
    } else if path.ends_with("/fish") {
        Shell::Fish
    } else {
        // Default to Bash for unknown or common paths (/bin/bash, /usr/bin/bash, etc.)
        Shell::Bash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_from_path() {
        assert_eq!(detect_from_path("/bin/zsh"), Shell::Zsh);
        assert_eq!(detect_from_path("/usr/local/bin/fish"), Shell::Fish);
        assert_eq!(detect_from_path("/bin/bash"), Shell::Bash);
        assert_eq!(detect_from_path("unknown"), Shell::Bash);
    }
}
