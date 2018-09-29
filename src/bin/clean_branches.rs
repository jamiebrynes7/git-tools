extern crate clap;
use clap::{App, Arg};

extern crate git;
use git::commands::branches::get_list_branches;
use git::commands::prune::{get_pruned_branches, prune_branches};
use git::git_branch::GitBranch;
use git::utils::errors::*;

use std::io::{self, Write};

struct Arguments {
    remote_name: String,
}

fn main() {
    let args = get_arguments();

    let pruned_branch_list = match get_pruned_branches(args.remote_name.clone()) {
        Ok(list) => list,
        Err(e) => {
            error_and_exit(e);
            panic!()
        }
    };
    let branch_list = match get_list_branches(false /* No remotes*/) {
        Ok(list) => list,
        Err(e) => {
            error_and_exit(e);
            panic!()
        }
    };
    let branches_to_delete: Vec<GitBranch> = get_intersection(&pruned_branch_list, &branch_list);

    if branches_to_delete.len() == 0 {
        println!("Found no branches to delete!");
        return;
    }

    match get_user_confirmation(&branches_to_delete, &pruned_branch_list) {
        true => delete_branches(&branches_to_delete, args.remote_name.clone()),
        false => println!("Aborting operation!"),
    }
}

fn get_arguments() -> Arguments {
    let matches = App::new("Git Clean Branches")
        .version("0.3.0")
        .author("Jamie Brynes <jamiebrynes7@gmail.com>")
        .about("Cleans remote and local branches that have been deleted.")
        .arg(
            Arg::with_name("remote-name")
                .short("n")
                .long("remote-name")
                .value_name("NAME")
                .help("The remote to base the clean off of. Default value is \"origin\"")
                .takes_value(true),
        ).get_matches();

    let remote_name = matches.value_of("remote-name").unwrap_or("origin");

    Arguments {
        remote_name: remote_name.to_string(),
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
        }).collect();

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

fn delete_branches(branches_to_delete: &Vec<GitBranch>, remote_name: String) {
    for branch in branches_to_delete {
        match branch.delete(true) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }

    match prune_branches(remote_name) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}
