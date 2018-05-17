use std::io::{self, Write};
use std::process;

struct ProcessOutput {
    stdout: String,
    stderr: String,
    code: i32
}

enum UserInputResult {
    Invalid(String),
    Valid(usize),
    Exit
}

fn main() 
{
    let list_branches_command = run_git_command(&vec!["branch".to_string(), "--list".to_string()]);

    if !list_branches_command.stderr.is_empty()
    {
        error_and_exit(format!("git branch --list failed with:\n{}", list_branches_command.stderr))
    }

    let branch_list = get_list_branches(list_branches_command.stdout);
    let desired_branch_index = select_branch_index(&branch_list) - 1;

    let switch_branch_command = run_git_command(&vec!["checkout".to_string(), branch_list[desired_branch_index].clone()]);
    if switch_branch_command.code != 0 {
        error_and_exit(format!("Switching branches failed with:\n{}", switch_branch_command.stderr))
    }
}

fn error_and_exit(error_message: String) {
    println!("{}", error_message);
    process::exit(1);
}

fn run_git_command(args: &Vec<String>) -> ProcessOutput {
    let command_response = process::Command::new("git").args(args.clone()).output();

    let out = match command_response {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e)
    };

    let stdout = match String::from_utf8(out.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let stderr = match String::from_utf8(out.stderr) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let code = out.status.code().expect(&format!("No error code in command: git {:?}", args));

    ProcessOutput { stdout, stderr, code }
}

fn get_list_branches(stdout: String) -> Vec<String> {
    let branches = stdout.split("\n").collect::<Vec<&str>>();
    let cleaned_branches: Vec<String> = branches.iter()
        .map(|s| s.replace("*", ""))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    return cleaned_branches
}

fn select_branch_index(branches: &Vec<String>) -> usize {

    println!("\nSelect a branch:");
    let mut branch_index: u8 = 1;
    for branch in branches {
        println!("  {}) {}", branch_index, branch);
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