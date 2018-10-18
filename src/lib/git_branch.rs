use std;

// Internal
use commands::process::*;

#[derive(PartialEq, Eq, Hash)]
pub struct GitBranch {
    pub name: String,
    pub remote_prefix: Option<String>,
}

impl GitBranch {
    pub fn delete(&self, should_log: bool) -> Result<(), String> {
        match run_git_command(&vec![
            "branch".to_string(),
            "-D".to_string(),
            self.name.clone(),
        ]) {
            ProcessOutput { code, .. } if code == 0 => {
                if should_log {
                    println!("Deleting {}...", self.name);
                }
                Ok(())
            }
            ProcessOutput { stderr, .. } => Err(format!(
                "Failed to delete branch: {} with error: {}",
                self.name, stderr
            )),
        }
    }

    pub fn checkout(&self, should_log: bool) -> Result<(), String> {
        match run_git_command(&vec!["checkout".to_string(), self.name.clone()]) {
            ProcessOutput { code, .. } if code == 0 => {
                if should_log {
                    println!("Checking out {}...", self.name);
                }
                Ok(())
            }
            ProcessOutput { stderr, .. } => Err(format!(
                "Failed to checkout branch: {} with error: {}",
                self.name, stderr
            )),
        }
    }
}

impl std::fmt::Display for GitBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.remote_prefix {
            Some(ref prefix) => write!(f, "remotes/{}/{}", prefix, self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

impl std::clone::Clone for GitBranch {
    fn clone(&self) -> GitBranch {
        GitBranch {
            name: self.name.clone(),
            remote_prefix: self.remote_prefix.clone(),
        }
    }
}
