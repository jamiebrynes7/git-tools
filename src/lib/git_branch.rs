use std;

// Internal
use commands::process::*;

pub trait BranchOperations {
    fn delete(&self, should_log: bool) -> Result<(), String>;
    fn checkout(&self, should_log: bool) -> Result<(), String>;
}

#[derive(PartialEq, Eq, Hash)]
pub struct GitBranch {
    pub name: String,
    pub remote_prefix: Option<String>,
}

impl BranchOperations for GitBranch {
    fn delete(&self, should_log: bool) -> Result<(), String> {
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

    fn checkout(&self, should_log: bool) -> Result<(), String> {
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

pub(crate) fn parse_raw_branch_data(
    raw_branch_data: &Vec<String>,
    remote_identifier: &str,
) -> Vec<GitBranch> {
    let branch_list = raw_branch_data
        .iter()
        .map(|s| match s.starts_with(remote_identifier) {
            true => GitBranch {
                name: s
                    .split((remote_identifier.to_string() + "/").as_str())
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
