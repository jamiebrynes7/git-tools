use std::process;

pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: i32
}

pub fn get_list_branches() -> Vec<String> {

    let list_branches_command = run_git_command(&vec!["branch".to_string(), "--list".to_string()]);
    if list_branches_command.code != 0 {
        error_and_exit(format!("git branch --list failed with:\n{}", list_branches_command.stderr))
    }

    let branches = list_branches_command.stdout.split("\n").collect::<Vec<&str>>();
    let cleaned_branches: Vec<String> = branches.iter()
        .map(|s| s.replace("*", ""))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    return cleaned_branches
}

pub fn run_git_command(args: &Vec<String>) -> ProcessOutput {
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

pub fn error_and_exit(error_message: String) {
    println!("{}", error_message);
    process::exit(1);
}