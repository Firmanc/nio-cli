// src/init/mod.rs
use crate::cli::{Cli, Shell};
use clap::CommandFactory;
use clap_complete::{generate, shells};
use std::env;
use std::io;

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

pub fn handle_init(shell_arg: Option<Shell>) -> anyhow::Result<()> {
    let shell = shell_arg.unwrap_or_else(detect_shell);

    let wrapper = match shell {
        Shell::Bash | Shell::Zsh => format!(
            r#"nio() {{
    if [[ "$1" == "gtree" && "$2" == "switch" ]]; then
        local target=$(command nio gtree switch "${{@:3}}")
        if [[ -n "$target" ]]; then
            cd "$target"
        fi
    else
        command nio "$@"
    fi
}}
"#
        ),
        Shell::Fish => format!(
            r#"function nio
    if test "$argv[1]" = "gtree"; and test "$argv[2]" = "switch"
        set -l target (command nio gtree switch $argv[3..-1])
        if test -n "$target"
            cd "$target"
        end
    else
        command nio $argv
    end
end
"#
        ),
    };

    println!("{}", wrapper);

    // Generate completions
    let mut cmd = Cli::command();
    let bin_name = "nio";

    match shell {
        Shell::Bash => generate(shells::Bash, &mut cmd, bin_name, &mut io::stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut cmd, bin_name, &mut io::stdout()),
        Shell::Fish => generate(shells::Fish, &mut cmd, bin_name, &mut io::stdout()),
    }

    Ok(())
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
