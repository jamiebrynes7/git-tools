extern crate git_shared;
use git_shared::{ GitBranch, error_and_exit, run_git_command, get_list_branches };

use std::io::{self, Write};
use std::process;

enum UserInputResult {
    Invalid(String),
    Valid(usize),
    Exit
}

fn main() {
    let branch_list = get_list_branches();
    let desired_branch_index = select_branch_index(&branch_list) - 1;

    let switch_branch_command = run_git_command(&vec!["checkout".to_string(), branch_list[desired_branch_index].name.clone() ]);
    if switch_branch_command.code != 0 {
        error_and_exit(format!("Switching branches failed with:\n{}", switch_branch_command.stderr))
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
            UserInputResult::Valid(i) => return i
        }
    }
}

fn get_user_input(max_value: usize) -> UserInputResult {
    print!("\n> ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read from stdin.");

    match user_input.trim() {
        "q" => UserInputResult::Exit,
        x => {
            match x.parse::<usize>() {
                Ok(i) => {
                    if i > max_value || i == 0 {
                        UserInputResult::Invalid("Given branch index does not exist.".to_string())
                    }
                    else {
                        UserInputResult::Valid(i)
                    }
                },
                Err(_) => UserInputResult::Invalid("Invalid input!".to_string())
            }
        }
    }
}