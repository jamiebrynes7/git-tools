use crate::commands::process::*;
use crate::errors::Result;
use crate::git_branch::GitBranch;

pub fn list_pruned_branches<S: AsRef<str>>(remote: S) -> Result<Vec<GitBranch>> {
    let process = git(vec!["remote", "prune", remote.as_ref(), "--dry-run"])?;
    process.get_error()?;

    process.lines().iter()
        .filter(|s| s.contains("*"))
        .map(|s| s.split("]").nth(1).unwrap().trim().to_string())
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            if s.starts_with(remote.as_ref()) {
                return Some(GitBranch::from_prune_ref(s));
            }

            None
        })
        .collect::<Result<Vec<GitBranch>>>()
}

pub fn prune_branches<S: AsRef<str>>(remote: S) -> Result<()> {
    git(vec!["remote", "prune", remote.as_ref()])?.get_error()?;
    Ok(())
}
