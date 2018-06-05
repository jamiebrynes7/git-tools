use std::process;

// Internal
use git_branch::GitBranch;

pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: i32,
}

pub fn get_list_branches() -> Vec<GitBranch> {
    let list_branches_command = run_git_command(&vec!["branch".to_string(), "--list".to_string()]);
    if list_branches_command.code != 0 {
        error_and_exit(format!(
            "git branch --list failed with:\n{}",
            list_branches_command.stderr
        ))
    }

    let branches = list_branches_command
        .stdout
        .split("\n")
        .collect::<Vec<&str>>();
    let cleaned_branches: Vec<String> = branches
        .iter()
        .map(|s| s.replace("*", ""))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    return parse_list_branch(&cleaned_branches, "remotes");
}
pub fn get_pruned_branches(stdout: String) -> Vec<GitBranch> {
    let pruned_branches: Vec<String> = stdout
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|s| s.contains("*"))
        .map(|s| s.replace("*", ""))
        .map(|s| s.split("]").nth(1).unwrap().to_string())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    return parse_list_branch(&pruned_branches, "origin");
}

fn parse_list_branch(raw_branch_data: &Vec<String>, remote_identifier: &str) -> Vec<GitBranch> {
    let branch_list = raw_branch_data
        .iter()
        .map(|s| match s.starts_with(remote_identifier) {
            true => GitBranch {
                name: s.split((remote_identifier.to_string() + "/").as_str())
                    .nth(1)
                    .unwrap()
                    .to_string(),
                remote_prefix: Some(remote_identifier.to_string()),
            },
            false => GitBranch {
                name: s.clone(),
                remote_prefix: None,
            },
        })
        .collect();

    return branch_list;
}

pub fn run_git_command(args: &Vec<String>) -> ProcessOutput {
    let command_response = process::Command::new("git").args(args.clone()).output();
    let out = match command_response {
        Ok(v) => v,
        Err(e) => panic!("Error: {}", e),
    };

    let stdout = match String::from_utf8(out.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let stderr = match String::from_utf8(out.stderr) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let code = out.status
        .code()
        .expect(&format!("No error code in command: git {:?}", args));

    ProcessOutput {
        stdout,
        stderr,
        code,
    }
}

pub fn error_and_exit(error_message: String) {
    println!("{}", error_message);
    process::exit(1);
}
