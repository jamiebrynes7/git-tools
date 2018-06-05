// Internal
use git_branch::{parse_raw_branch_data, GitBranch};
use commands::process::*;

pub fn get_list_branches() -> Result<Vec<GitBranch>, String> {
    let list_branches_command = run_git_command(&vec!["branch".to_string(), "--list".to_string()]);
    if list_branches_command.code != 0 {
        return Err(format!(
            "git branch --list failed with:\n{}",
            list_branches_command.stderr
        ))
    }

    let branches = list_branches_command
        .stdout
        .split("\n")
        .collect::<Vec<&str>>();
    let cleaned_branches: Vec<String> = branches
        .iter()
        .map(|s| s.replace("*", ""))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(parse_raw_branch_data(&cleaned_branches, "remotes"))
}
