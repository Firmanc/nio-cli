// src/main.rs
mod utils;
mod git;
mod cli;
mod gtree;
mod init;
mod kill;

use clap::Parser;
use cli::{Cli, Commands};
use std::env;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gtree { command } => {
            let current_dir = env::current_dir()?;
            gtree::handle_command(command, &current_dir)?;
        }
        Commands::Init { shell } => {
            init::handle_init(*shell)?;
        }
        Commands::Kill { identifiers } => {
            kill::handle_command(identifiers)?;
        }
    }

    Ok(())
}
