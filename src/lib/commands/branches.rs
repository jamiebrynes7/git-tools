use crate::commands::process::*;
use crate::errors::Result;
use crate::git_branch::GitBranch;

pub fn list_branches(show_remotes: bool) -> Result<Vec<GitBranch>> {
    let list_branches_command = match show_remotes {
        true => git(vec!["branch", "--all"])?,
        false => git(vec!["branch", "--list"])?,
    };

    list_branches_command.get_error()?;

    list_branches_command.lines()
        .iter()
        .map(|s| s.replace("*", "").trim().to_string())
        .filter(|s| !s.is_empty() && !s.contains("->"))
        .map(|s| {
            if s.starts_with("remotes") {
                Ok(GitBranch::from_remote(s)?)
            } else {
                Ok(GitBranch::from_local(s))
            }
        })
        .collect::<Result<Vec<GitBranch>>>()
}
