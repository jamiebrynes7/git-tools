use std::process;
use crate::errors::Result;
use std::ffi::OsStr;

pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub status_code: i32,
}

impl ProcessOutput {
    pub fn get_error(&self) -> Result<()> {
        if self.status_code == 0 {
            return Ok(());
        }

        Err(self.stderr.clone().into())
    }

    pub fn success(&self) -> bool {
        self.status_code == 0
    }

    pub fn lines(&self) -> Vec<&str> {
        self.stdout.split("\n").collect()
    }
}

pub fn git<I, S>(args: I) -> Result<ProcessOutput> where I: IntoIterator<Item=S>, S: AsRef<OsStr> {
    let command = process::Command::new("git")
        .args(args)
        .output()?;

    let stdout = String::from_utf8(command.stdout)?;
    let stderr = String::from_utf8(command.stderr)?;
    let status_code = match command.status.code() {
        Some(value) => value,
        None => return Err("Git command did not have exit code!?".into())
    };

    Ok(ProcessOutput {
        stdout,
        stderr,
        status_code,
    })
}
