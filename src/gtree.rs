// src/gtree.rs
use std::path::Path;
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use crate::cli::GtreeCommands;
use crate::{git, utils};

pub fn handle_command(command: &GtreeCommands, current_dir: &Path) -> Result<()> {
    match command {
        GtreeCommands::Create { branch_name } => create_worktree(current_dir, branch_name)?,
        GtreeCommands::Switch => switch_worktree()?,
        GtreeCommands::Remove { branch_name } => remove_worktree(current_dir, branch_name)?,
    }
    Ok(())
}

fn create_worktree(current_dir: &Path, branch_name: &str) -> Result<()> {
    let target_path = utils::build_worktree_path(current_dir, branch_name)
        .context("Failed to build target worktree path")?;
    
    git::add_worktree(&target_path, branch_name)
        .context("Failed to execute git worktree add")?;
        
    eprintln!("Successfully created worktree for branch '{}' at {:?}", branch_name, target_path);
    Ok(())
}

fn switch_worktree() -> Result<()> {
    let worktrees = git::list_worktrees().context("Failed to list worktrees")?;
    
    if worktrees.is_empty() {
        eprintln!("No worktrees found.");
        return Ok(());
    }

    let items: Vec<String> = worktrees.iter()
        .map(|w| format!("{} ({})", w.branch, w.path))
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a worktree to switch to")
        .default(0)
        .items(&items)
        .interact_opt()?;

    if let Some(index) = selection {
        let selected_path = &worktrees[index].path;
        // Print the absolute path to stdout for the shell wrapper to catch
        println!("{}", selected_path);
    } else {
        eprintln!("Selection cancelled.");
    }

    Ok(())
}

fn remove_worktree(current_dir: &Path, branch_name: &str) -> Result<()> {
    let target_path = utils::build_worktree_path(current_dir, branch_name)
        .context("Failed to build target worktree path")?;
        
    if !target_path.exists() {
        anyhow::bail!("Worktree path does not exist: {:?}", target_path);
    }

    git::remove_worktree(&target_path)
        .context("Failed to execute git worktree remove")?;
        
    eprintln!("Successfully removed worktree for branch '{}' at {:?}", branch_name, target_path);
    Ok(())
}
