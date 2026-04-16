// src/git.rs
use std::process::Command;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Failed to execute git command: {0}")]
    ExecutionError(#[from] std::io::Error),
    #[error("Git command failed with status: {0}")]
    CommandFailed(String),
}

#[derive(Debug, PartialEq)]
pub struct Worktree {
    pub path: String,
    pub branch: String,
}

pub fn run_git_command(args: &[&str]) -> Result<String, GitError> {
    let output = Command::new("git")
        .args(args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(GitError::CommandFailed(stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn add_worktree(target_path: &Path, branch_name: &str) -> Result<(), GitError> {
    let path_str = target_path.to_str().unwrap_or_default();
    run_git_command(&["worktree", "add", "-b", branch_name, path_str])?;
    Ok(())
}

pub fn add_worktree_existing(target_path: &Path, branch_name: &str) -> Result<(), GitError> {
    let path_str = target_path.to_str().unwrap_or_default();
    run_git_command(&["worktree", "add", path_str, branch_name])?;
    Ok(())
}

pub fn branch_exists(branch_name: &str) -> Result<bool, GitError> {
    let status = Command::new("git")
        .args(["show-ref", "--verify", "--quiet", &format!("refs/heads/{}", branch_name)])
        .status()?;
    Ok(status.success())
}

pub fn remove_worktree(target_path: &Path, force: bool) -> Result<(), GitError> {
    let path_str = target_path.to_str().unwrap_or_default();
    if force {
        run_git_command(&["worktree", "remove", "--force", path_str])?;
    } else {
        run_git_command(&["worktree", "remove", path_str])?;
    }
    Ok(())
}

pub fn list_ignored_items() -> Result<Vec<String>, GitError> {
    let output = run_git_command(&["clean", "-ndX"])?;
    Ok(parse_ignored_list(&output))
}

fn parse_ignored_list(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            // "Would remove path/to/file"
            line.strip_prefix("Would remove ").map(|s| s.trim().to_string())
        })
        .collect()
}

pub fn list_worktrees() -> Result<Vec<Worktree>, GitError> {
    let output = run_git_command(&["worktree", "list", "--porcelain"])?;
    parse_worktree_list(&output)
}

fn parse_worktree_list(output: &str) -> Result<Vec<Worktree>, GitError> {
    let mut worktrees = Vec::new();
    let mut current_path = String::new();

    for line in output.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            current_path = path.to_string();
        } else if let Some(branch) = line.strip_prefix("branch ") {
            // Strip the full ref path to just get the branch name
            let branch_name = branch.replace("refs/heads/", "");
            if !current_path.is_empty() {
                worktrees.push(Worktree {
                    path: current_path.clone(),
                    branch: branch_name,
                });
                current_path.clear();
            }
        }
    }

    Ok(worktrees)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_worktree_list() {
        let output = "\
worktree /path/to/my-repo
branch refs/heads/main

worktree /path/to/my-repo_feature-x
branch refs/heads/feature-x

worktree /path/to/my-repo_fix-bug
branch refs/heads/fix-bug
";
        let expected = vec![
            Worktree {
                path: "/path/to/my-repo".to_string(),
                branch: "main".to_string(),
            },
            Worktree {
                path: "/path/to/my-repo_feature-x".to_string(),
                branch: "feature-x".to_string(),
            },
            Worktree {
                path: "/path/to/my-repo_fix-bug".to_string(),
                branch: "fix-bug".to_string(),
            },
        ];
        assert_eq!(parse_worktree_list(output).unwrap(), expected);
    }

    #[test]
    fn test_parse_ignored_list() {
        let output = "\
Would remove .env
Would remove node_modules/
Would remove target/
";
        let expected = vec![
            ".env".to_string(),
            "node_modules/".to_string(),
            "target/".to_string(),
        ];
        assert_eq!(parse_ignored_list(output), expected);
    }
}
