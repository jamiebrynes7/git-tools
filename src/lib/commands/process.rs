use std::process;

pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: i32,
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

    let code = out
        .status
        .code()
        .expect(&format!("No error code in command: git {:?}", args));

    ProcessOutput {
        stdout,
        stderr,
        code,
    }
}
