use commands::process::*;
use git_branch::GitBranch;

pub fn get_pruned_branches(remote: String) -> Result<Vec<GitBranch>, String> {
    match run_prune_branches(remote, true) {
        Ok(process_output) => {
            let pruned_branches: Vec<String> = process_output
                .stdout
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .filter(|s| s.contains("*"))
                .map(|s| s.replace("*", ""))
                .map(|s| s.split("]").nth(1).unwrap().to_string())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            Ok(parse_raw_branch_data(&pruned_branches, "origin"))
        }
        Err(message) => Err(message),
    }
}

pub fn prune_branches(remote: String) -> Result<(), String> {
    match run_prune_branches(remote, false) {
        Ok(_) => Ok(()),
        Err(message) => Err(message),
    }
}

fn run_prune_branches(remote: String, dry_run: bool) -> Result<ProcessOutput, String> {
    let mut args = vec!["remote".to_string(), "prune".to_string(), remote.clone()];

    if dry_run {
        args.push("--dry-run".to_string());
    }

    let git_prune_origin_command = run_git_command(&args);

    if git_prune_origin_command.code != 0 {
        return Err(format!(
            "git prune remote {} failed with:\n{}",
            remote, git_prune_origin_command.stderr
        ));
    }

    return Ok(git_prune_origin_command);
}

fn parse_raw_branch_data(raw_branch_data: &Vec<String>, remote_identifier: &str) -> Vec<GitBranch> {
    let branch_list = raw_branch_data
        .iter()
        .map(|s| match s.starts_with(remote_identifier) {
            true => {
                let branch_name = s
                    .split((remote_identifier.to_string() + "/").as_str())
                    .nth(1)
                    .unwrap();

                GitBranch {
                    name: branch_name.to_string(),
                    remote_prefix: Some(remote_identifier.to_string()),
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
