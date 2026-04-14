// src/cli.rs
use clap::{Parser, Subcommand, ValueEnum};

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
    /// Generate shell initialization script
    Init {
        /// The shell to generate the script for (bash, zsh, fish)
        #[arg(value_enum)]
        shell: Option<Shell>,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

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
