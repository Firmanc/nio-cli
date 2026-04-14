// src/gtree.rs
use std::path::Path;
use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use fs_extra::dir::CopyOptions;
use crate::cli::GtreeCommands;
use crate::{git, utils};

pub fn handle_command(command: &GtreeCommands, current_dir: &Path) -> Result<()> {
    match command {
        GtreeCommands::Create { branch_name, copy_ignored } => {
            create_worktree(current_dir, branch_name, *copy_ignored)?
        },
        GtreeCommands::Switch => switch_worktree()?,
        GtreeCommands::List => list_worktrees()?,
        GtreeCommands::Remove { branch_name } => remove_worktree(current_dir, branch_name)?,
    }
    Ok(())
}

fn create_worktree(current_dir: &Path, branch_name: &str, copy_ignored: bool) -> Result<()> {
    let target_path = utils::build_worktree_path(current_dir, branch_name)
        .context("Failed to build target worktree path")?;
    
    git::add_worktree(&target_path, branch_name)
        .context("Failed to execute git worktree add")?;
        
    eprintln!("Successfully created worktree for branch '{}' at {:?}", branch_name, target_path);

    if copy_ignored {
        copy_ignored_items(current_dir, &target_path)?;
    }

    Ok(())
}

fn copy_ignored_items(source_dir: &Path, target_dir: &Path) -> Result<()> {
    let ignored_items = git::list_ignored_items().context("Failed to list ignored items")?;
    
    if ignored_items.is_empty() {
        eprintln!("No ignored files to copy.");
        return Ok(());
    }

    eprintln!("Copying ignored files...");
    
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    for item in ignored_items {
        let source_path = source_dir.join(&item);
        if !source_path.exists() {
            continue;
        }

        // We use copy_items which handles both files and directories
        let items_to_copy = vec![source_path];
        if let Err(e) = fs_extra::copy_items(&items_to_copy, target_dir, &options) {
            eprintln!("Warning: Failed to copy {}: {}", item, e);
        } else {
            eprintln!("  Copied {}", item);
        }
    }

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

fn list_worktrees() -> Result<()> {
    let worktrees = git::list_worktrees().context("Failed to list worktrees")?;
    
    if worktrees.is_empty() {
        eprintln!("No worktrees found.");
        return Ok(());
    }

    // Find the longest branch name for alignment
    let max_branch_len = worktrees.iter()
        .map(|w| w.branch.len())
        .max()
        .unwrap_or(0);

    for w in worktrees {
        println!("{:<width$}  {}", w.branch, w.path, width = max_branch_len);
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
