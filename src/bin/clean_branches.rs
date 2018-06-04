extern crate git_shared;
use git_shared::{error_and_exit, get_list_branches, get_pruned_branches, run_git_command,
                 GitBranch, ProcessOutput};

use std::io::{self, Write};

fn main() {
    let git_prune_origin_command = run_git_command(&vec![
        "remote".to_string(),
        "prune".to_string(),
        "origin".to_string(),
        "--dry-run".to_string(),
    ]);

    if git_prune_origin_command.code != 0 {
        error_and_exit(format!(
            "git prune remote origin failed with:\n{}",
            git_prune_origin_command.stderr
        ));
    }

    let pruned_branch_list = get_pruned_branches(git_prune_origin_command.stdout);
    let branch_list = get_list_branches();
    let branches_to_delete: Vec<GitBranch> = get_intersection(&pruned_branch_list, &branch_list);

    if branches_to_delete.len() == 0 {
        println!("Found no branches to delete!");
        return;
    }

    match get_user_confirmation(&branches_to_delete, &pruned_branch_list) {
        true => delete_branches(&branches_to_delete),
        false => println!("Aborting operation!"),
    }
}

fn get_intersection(
    remote_branches: &Vec<GitBranch>,
    local_branches: &Vec<GitBranch>,
) -> Vec<GitBranch> {
    let intersection = local_branches
        .iter()
        .cloned()
        .filter(|branch| {
            remote_branches
                .iter()
                .any(|remote_branch| branch.name == remote_branch.name)
        })
        .collect();

    return intersection;
}

fn get_user_confirmation(
    branches_to_delete: &Vec<GitBranch>,
    remote_branches: &Vec<GitBranch>,
) -> bool {
    println!("This will delete the following local and remote branches:");
    for branch in branches_to_delete {
        println!(" * {}", branch);
    }
    for branch in remote_branches {
        println!(" * {}", branch);
    }

    print!("\nEnter y to confirm: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from stdin.");

    match user_input.trim().to_lowercase().as_str() {
        "y" => true,
        _ => false,
    }
}

fn delete_branches(branches_to_delete: &Vec<GitBranch>) {
    for branch in branches_to_delete {
        match run_git_command(&vec![
            "branch".to_string(),
            "-D".to_string(),
            branch.name.clone(),
        ]) {
            ProcessOutput { code, .. } if code == 0 => println!("Deleting: {}", branch),
            ProcessOutput { stderr, .. } => error_and_exit(format!(
                "Failed to delete branch: {} with error: {}",
                branch, stderr
            )),
        };
    }

    match run_git_command(&vec![
        "remote".to_string(),
        "prune".to_string(),
        "origin".to_string(),
    ]) {
        ProcessOutput { code, .. } if code == 0 => println!("Deleting remotes..."),
        ProcessOutput { stderr, .. } => {
            error_and_exit(format!("Failed to delete remotes with error: {}", stderr))
        }
    };
}
