use std;
use std::error::Error;
use std::ffi::OsStr;
use std::process;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn list_pruned_branches<S: AsRef<str>>(remote: S) -> Result<Vec<GitBranch>> {
    let process = git(vec!["remote", "prune", remote.as_ref(), "--dry-run"])?;
    process.get_error()?;

    process
        .lines()
        .iter()
        .filter(|s| s.contains('*'))
        .map(|s| s.split(']').nth(1).unwrap().trim().to_string())
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            if s.starts_with(remote.as_ref()) {
                return Some(GitBranch::from_prune_ref(s));
            }

            None
        })
        .collect::<Result<Vec<GitBranch>>>()
}

pub fn prune_branches<S: AsRef<str>>(remote: S) -> Result<()> {
    git(vec!["remote", "prune", remote.as_ref()])?.get_error()?;
    Ok(())
}

pub fn list_branches(show_remotes: bool) -> Result<Vec<GitBranch>> {
    let list_branches_command = if show_remotes {
        git(vec!["branch", "--all"])?
    } else {
        git(vec!["branch", "--list"])?
    };

    list_branches_command.get_error()?;

    list_branches_command
        .lines()
        .iter()
        .map(|s| s.replace('*', "").trim().to_string())
        .filter(|s| !s.is_empty() && !s.contains("->"))
        .map(|s| {
            if s.starts_with("remotes") {
                Ok(GitBranch::from_remote(s)?)
            } else {
                Ok(GitBranch::from_local(s))
            }
        })
        .collect::<Result<Vec<GitBranch>>>()
}

fn git<I, S>(args: I) -> Result<ProcessOutput>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let command = process::Command::new("git").args(args).output()?;

    let stdout = String::from_utf8(command.stdout)?;
    let stderr = String::from_utf8(command.stderr)?;
    let status_code = match command.status.code() {
        Some(value) => value,
        None => return Err("Git command did not have exit code!?".into()),
    };

    Ok(ProcessOutput {
        stdout,
        stderr,
        status_code,
    })
}
struct ProcessOutput {
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
        self.stdout.split('\n').collect()
    }
}

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
            remote_prefix: None,
        }
    }

    // Expects input in the form of: "remotes/{origin_name}/feature/my-feature
    pub fn from_remote<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();
        let mut parts = name.split('/').skip(1);

        let remote_name = match parts.next() {
            Some(value) => value,
            None => return Err("Invalid remote branch format".into()),
        };
        let branch_name = parts
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("/");

        Ok(GitBranch {
            name: branch_name,
            remote_prefix: Some(remote_name.to_string()),
        })
    }

    pub fn from_prune_ref<S: Into<String>>(name: S) -> Result<Self> {
        let name = name.into();
        let mut parts = name.split('/');

        let remote_name = match parts.next() {
            Some(value) => value,
            None => return Err("Invalid pruned branch reference format".into()),
        };
        let branch_name = parts
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("/");

        Ok(GitBranch {
            name: branch_name,
            remote_prefix: Some(remote_name.to_string()),
        })
    }

    pub fn delete(&self) -> Result<()> {
        let process = git(vec!["branch", "-D", &self.name])?;
        if !process.success() {
            return Err(format!(
                "Failed to delete branch {} with error {}",
                self.name, process.stderr
            )
            .into());
        }

        println!("Deleting {} ... ", self.name);
        Ok(())
    }

    pub fn checkout(&self) -> Result<()> {
        let process = git(vec!["checkout", &self.name])?;
        if !process.success() {
            return Err(format!(
                "Failed to checkout branch {} with error {}",
                self.name, process.stderr
            )
            .into());
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
