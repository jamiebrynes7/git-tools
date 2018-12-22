// Internal
use crate::commands::process::*;
use crate::git_branch::GitBranch;

pub fn get_list_branches(show_remotes: bool) -> Result<Vec<GitBranch>, String> {
    let list_branches_command = match show_remotes {
        true => run_git_command(&vec!["branch".to_string(), "-a".to_string()]),
        false => run_git_command(&vec!["branch".to_string(), "--list".to_string()]),
    };
    if list_branches_command.code != 0 {
        return Err(format!(
            "git branch --list failed with:\n{}",
            list_branches_command.stderr
        ));
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
        .filter(|s| !s.contains("->"))
        .collect();

    Ok(parse_raw_branch_data(&cleaned_branches, "remotes"))
}

fn parse_raw_branch_data(raw_branch_data: &Vec<String>, remote_identifier: &str) -> Vec<GitBranch> {
    let branch_list = raw_branch_data
        .iter()
        .map(|s| match s.starts_with(remote_identifier) {
            true => {
                let remote_branch_name = s
                    .split((remote_identifier.to_string() + "/").as_str())
                    .nth(1)
                    .unwrap();

                let remote = remote_branch_name.split("/").nth(0).unwrap().to_string();
                let branch: String = remote_branch_name
                    .split("/")
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join("/");

                GitBranch {
                    name: branch,
                    remote_prefix: Some(remote),
                }
            }
            false => GitBranch {
                name: s.clone(),
                remote_prefix: None,
            },
        })
        .collect();

    return branch_list;
}
