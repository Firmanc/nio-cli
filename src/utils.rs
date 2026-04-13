use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilError {
    #[error("Failed to get current directory")]
    CurrentDirFailed,
    #[error("Failed to get file name from path")]
    NoFileName,
}

pub fn get_current_dir_name(current_dir: &Path) -> Result<String, UtilError> {
    current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
        .ok_or(UtilError::NoFileName)
}

pub fn build_worktree_path(current_dir: &Path, branch_name: &str) -> Result<PathBuf, UtilError> {
    let dir_name = get_current_dir_name(current_dir)?;
    let target_dir_name = format!("{}_{}", dir_name, branch_name);
    let mut target_path = current_dir.parent().ok_or(UtilError::NoFileName)?.to_path_buf();
    target_path.push(target_dir_name);
    Ok(target_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_dir_name() {
        let path = Path::new("/path/to/my-repo");
        assert_eq!(get_current_dir_name(path).unwrap(), "my-repo");
    }

    #[test]
    fn test_build_worktree_path() {
        let current_dir = Path::new("/path/to/my-repo");
        let branch = "feature-x";
        let expected = PathBuf::from("/path/to/my-repo_feature-x");
        assert_eq!(build_worktree_path(current_dir, branch).unwrap(), expected);
    }
}
