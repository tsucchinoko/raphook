use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const AVAILABLE_HOOKS: &[&str] = &[
    "pre-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-push",
];

fn validate_git_hooks_directory(repo_path: &str) -> io::Result<PathBuf> {
    let hooks_dir = Path::new(repo_path).join(".git").join("hooks");

    if !Path::new(repo_path).join(".git").exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Not a git repository: {}", repo_path),
        ));
    }

    Ok(hooks_dir)
}

fn remove_hook(hooks_dir: &Path, hook_name: &str) -> io::Result<bool> {
    let hook_path = hooks_dir.join(hook_name);
    if hook_path.exists() {
        fs::remove_file(hook_path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn uninstall(path: &str) -> io::Result<Vec<String>> {
    let hooks_dir = validate_git_hooks_directory(path)?;
    let mut removed_hooks = Vec::new();

    for &hook in AVAILABLE_HOOKS {
        if remove_hook(&hooks_dir, hook)? {
            removed_hooks.push(hook.to_string());
        }
    }

    Ok(removed_hooks)
}
