extern crate git_shared;
use git_shared::{ ProcessOutput, error_and_exit, run_git_command, get_list_branches };

use std::collections::HashSet;
use std::iter::FromIterator;
use std::io::{self, Write};

fn main() {
    let git_prune_origin_command = run_git_command(&vec!["remote".to_string(),
                                                         "prune".to_string(),
                                                         "origin".to_string(),
                                                         "--dry-run".to_string()]);

    if git_prune_origin_command.code != 0 {
        error_and_exit(format!("git prune remote origin failed with:\n{}", git_prune_origin_command.stderr));
    }

    let pruned_branch_list : HashSet<String> = HashSet::from_iter(get_pruned_branches(git_prune_origin_command.stdout).iter().cloned());
    let branch_list : HashSet<String> = HashSet::from_iter(get_list_branches().iter().cloned());
    let branches_to_delete: Vec<String> = pruned_branch_list.intersection(&branch_list)
        .map(|s| s.to_string())
        .collect();

    if branches_to_delete.len() == 0 {
        println!("Found no branches to delete!");
        return
    }

    match get_user_confirmation(&branches_to_delete) {
        true => delete_branches(&branches_to_delete),
        false => println!("Aborting operation!")
    }
}

fn get_pruned_branches(stdout: String) -> Vec<String> {
    let pruned_branches: Vec<String> = stdout.split("\n")
        .collect::<Vec<&str>>().iter()
        .filter(|s| s.contains("*"))
        .map(|s| s.split("origin/").nth(1).unwrap().to_string())
        .collect();

    return pruned_branches
}

fn get_user_confirmation(branches_to_delete: &Vec<String>) -> bool {
    println!("This will delete the following local and remote branches:");
    for branch in branches_to_delete {
        println!(" * { }", branch);
    }

    print!("\nEnter y to confirm: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read from stdin.");

    match user_input.trim().to_lowercase().as_str() {
        "y" => true,
        _ => false
    }
}

fn delete_branches(branches_to_delete: &Vec<String>) {
    for branch in branches_to_delete {
        match run_git_command(&vec!["branch".to_string(), "-D".to_string(), branch.clone()]) {
            ProcessOutput{code, ..} if code == 0 => println!("Deleting: {}", branch),
            ProcessOutput{stderr, ..} => error_and_exit(format!("Failed to delete branch: {} with error: {}", branch, stderr)),
        };
    }

    match run_git_command(&vec!["remote".to_string(), "prune".to_string(), "origin".to_string()]) {
        ProcessOutput{code, ..} if code == 0 => println!("Deleting remotes..."),
        ProcessOutput{stderr, ..} => error_and_exit(format!("Failed to delete remotes with error: {}", stderr)),
    };
}