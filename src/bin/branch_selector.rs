extern crate clap;
use clap::{App, Arg};

extern crate git;
use git::commands::branches::get_list_branches;
use git::git_branch::{BranchOperations, GitBranch};
use git::utils::errors::*;

use std::io::{self, Write};
use std::process;

struct Arguments {
    show_remotes: bool,
}

enum UserInputResult {
    Invalid(String),
    Valid(usize),
    Exit,
}

fn main() {
    let args = get_arguments();

    let branch_list = match get_list_branches(args.show_remotes) {
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

fn get_arguments() -> Arguments {
    let matches = App::new("Git Select Branch")
        .version("0.2.0")
        .author("Jamie Brynes <jamiebrynes7@gmail.com>")
        .about("Simple branch selector.")
        .arg(
            Arg::with_name("show-remotes")
                .short("r")
                .long("show-remotes")
                .help("Flag to indicate whether to show remote branches as well.")
                .takes_value(false),
        )
        .get_matches();

    let show_remotes = matches.is_present("show-remotes");

    Arguments {
        show_remotes: show_remotes,
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
