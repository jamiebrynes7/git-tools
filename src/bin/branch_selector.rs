extern crate git;
use git::commands::branches::get_list_branches;
use git::git_branch::{BranchOperations, GitBranch};
use git::utils::errors::*;

use std::io::{self, Write};
use std::process;

enum UserInputResult {
    Invalid(String),
    Valid(usize),
    Exit,
}

fn main() {
    let branch_list = match get_list_branches() {
        Ok(list) => list,
        Err(e) => {
            error_and_exit(e);
            panic!()
        }
    };
    let desired_branch_index = select_branch_index(&branch_list) - 1;

    match branch_list[desired_branch_index].checkout(true) {
        Ok(_) => {}
        Err(e) => error_and_exit(e),
    }
}

fn select_branch_index(branches: &Vec<GitBranch>) -> usize {
    println!("\nSelect a branch:");
    let mut branch_index: u8 = 1;
    for branch in branches {
        println!("  {}) {}", branch_index, branch.name);
        branch_index += 1;
    }

    loop {
        match get_user_input(branches.len()) {
            UserInputResult::Exit => process::exit(0),
            UserInputResult::Invalid(s) => println!("{}", s),
            UserInputResult::Valid(i) => return i,
        }
    }
}

fn get_user_input(max_value: usize) -> UserInputResult {
    print!("\n> ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from stdin.");

    match user_input.trim() {
        "q" => UserInputResult::Exit,
        x => match x.parse::<usize>() {
            Ok(i) => {
                if i > max_value || i == 0 {
                    UserInputResult::Invalid("Given branch index does not exist.".to_string())
                } else {
                    UserInputResult::Valid(i)
                }
            }
            Err(_) => UserInputResult::Invalid("Invalid input!".to_string()),
        },
    }
}
