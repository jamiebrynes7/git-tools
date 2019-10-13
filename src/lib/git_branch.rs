use std;

// Internal
use crate::errors::Result;
use crate::commands::process::*;

#[derive(PartialEq, Eq, Hash)]
pub struct GitBranch {
    pub name: String,
    pub remote_prefix: Option<String>,
}

impl GitBranch {
    // Expects input in the form of: "feature/my-feature"
    pub fn from_local<S: Into<String>>(name: S) -> Self {
        GitBranch {
            name: name.into(),
            remote_prefix: None
        }
    }

    // Expects input in the form of: "remotes/{origin_name}/feature/my-feature
    pub fn from_remote<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();
        let mut parts = name.split("/").skip(1);

        let remote_name = match parts.next() {
            Some(value) => value,
            None => return Err("Invalid remote branch format".into()),
        };
        let branch_name = parts.map(|s| s.to_string()).collect::<Vec<String>>().join("/");

        Ok(GitBranch {
            name: branch_name,
            remote_prefix: Some(remote_name.to_string())
        })
    }

    pub fn from_prune_ref<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();
        let mut parts = name.split("/");

        let remote_name = match parts.next() {
            Some(value) => value,
            None => return Err("Invalid pruned branch reference format".into()),
        };
        let branch_name = parts.map(|s| s.to_string()).collect::<Vec<String>>().join("/");

        Ok(GitBranch {
            name: branch_name,
            remote_prefix: Some(remote_name.to_string())
        })
    }

    pub fn delete(&self, should_log: bool) -> Result<()> {
        let process = git(vec!["branch", "-D", &self.name])?;
        if !process.success() {
            return Err(format!("Failed to delete branch {} with error {}", self.name, process.stderr).into());
        }

        println!("Deleting {} ... ", self.name);
        Ok(())
    }

    pub fn checkout(&self, should_log: bool) -> Result<()> {
        let process = git(vec!["checkout", &self.name])?;
        if !process.success() {
            return Err(format!("Failed to checkout branch {} with error {}", self.name, process.stderr).into());
        }

        println!("Checking out {} ...", self.name);
        Ok(())
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
