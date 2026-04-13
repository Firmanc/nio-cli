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
