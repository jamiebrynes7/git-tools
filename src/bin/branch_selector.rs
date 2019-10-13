use structopt::StructOpt;

use git::*;

use std::io::{self, Write};
use std::process;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "git-branch-selector",
    about = "Quickly switch between Git branches."
)]
struct Arguments {
    /// Denotes whether to list remote branches.
    #[structopt(short = "r", long)]
    show_remotes: bool,
}

enum UserInputResult {
    Invalid(String),
    Valid(usize),
    Exit,
}

fn main() -> Result<()> {
    let args: Arguments = Arguments::from_args();

    let branch_list = list_branches(args.show_remotes)?;
    let desired_branch_index = select_branch_index(&branch_list)? - 1;

    branch_list[desired_branch_index].checkout()
}

fn select_branch_index(branches: &Vec<GitBranch>) -> Result<usize> {
    println!("\nSelect a branch:");
    let mut branch_index: u8 = 1;
    for branch in branches {
        println!("  {}) {}", branch_index, branch);
        branch_index += 1;
    }

    loop {
        match get_user_input(branches.len())? {
            UserInputResult::Exit => process::exit(0),
            UserInputResult::Invalid(s) => println!("{}", s),
            UserInputResult::Valid(i) => return Ok(i),
        }
    }
}

fn get_user_input(max_value: usize) -> Result<UserInputResult> {
    print!("\n> ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    match user_input.trim() {
        "q" => Ok(UserInputResult::Exit),
        x => match x.parse::<usize>() {
            Ok(i) => {
                if i > max_value || i == 0 {
                    Ok(UserInputResult::Invalid(
                        "Given branch index does not exist.".to_string(),
                    ))
                } else {
                    Ok(UserInputResult::Valid(i))
                }
            }
            Err(_) => Ok(UserInputResult::Invalid("Invalid input!".to_string())),
        },
    }
}
