use std::io::{self, Write};
use std::process;

struct ProcessOutput {
    stdout: String,
    stderr: String
}

fn main() 
{
    let list_branches_command = run_git_command(vec!["branch".to_string(), "--list".to_string()]);

    if !list_branches_command.stderr.is_empty()
    {
        panic!("git branch --list failed with:\n{}", list_branches_command.stderr)
    }

    let branch_list = get_list_branches(list_branches_command.stdout);
    let desired_branch_index = (select_branch_index(&branch_list) - 1) as usize;

    let switch_branch_command = run_git_command(vec!["checkout".to_string(), branch_list[desired_branch_index].clone()]);

    if !switch_branch_command.stdout.is_empty() {
        panic!("Switching branches failed with:\n{}", switch_branch_command.stderr)
    }

}

fn run_git_command(args: Vec<String>) -> ProcessOutput {
    let command_response = process::Command::new("git").args(args).output();

    let out = match command_response {
        Err(e) => panic!("Error: {}", e),
        Ok(v) => v 
    };

    let stdout = match String::from_utf8(out.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let stderr = match String::from_utf8(out.stderr) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    ProcessOutput { stdout: stdout, stderr: stderr }
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

fn select_branch_index(branches: &Vec<String>) -> u8 {

    println!("\nSelect a branch:");
    let mut branch_index: u8 = 1;
    for branch in branches {
        println!("  {}) {}", branch_index, branch);
        branch_index += 1;
    }

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text)
            .expect("Failed to read from stdin");

        let trimmed = input_text.trim();
        if trimmed == "q" {
            println!("Exiting!");
            return 0
        }
        match trimmed.parse::<u8>() {
            Ok(i) =>  {
                if i as usize > branches.len() {
                    println!("Given branch index too large!");
                }
                else {
                    return i
                }
            },
            Err(_) => { },
        };
    }
}