use git::*;
use structopt::StructOpt;

use std::io::{self, Write};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "git-clean-branches",
    about = "Clean remote and local branches."
)]
struct Arguments {
    /// Denotes whether to list remote branches.
    #[structopt(short = "r", long, default_value = "origin")]
    remote: String,
}

fn main() -> Result<()> {
    let args: Arguments = Arguments::from_args();

    let pruned_branches = list_pruned_branches(&args.remote)?;

    if pruned_branches.is_empty() {
        println!("Found no pruned remote branches!");
        return Ok(());
    }

    let local_branches = list_branches(false)?;

    let stale_local_branches = get_intersection(&pruned_branches, &local_branches);

    if stale_local_branches.is_empty() {
        println!("Found no local branches to delete!");
    }

    if get_user_confirmation(&stale_local_branches, &pruned_branches)? {
        delete_branches(&stale_local_branches, &args.remote)?;
    } else {
        println!("Aborting operation!");
    }

    Ok(())
}

fn get_intersection(remote_branches: &[GitBranch], local_branches: &[GitBranch]) -> Vec<GitBranch> {
    local_branches
        .iter()
        .cloned()
        .filter(|branch| {
            remote_branches
                .iter()
                .any(|remote_branch| branch.name == remote_branch.name)
        })
        .collect()
}

fn get_user_confirmation(
    stale_local_branches: &[GitBranch],
    remote_branches: &[GitBranch],
) -> Result<bool> {
    println!("This will delete the following local and remote branches:");

    for branch in stale_local_branches.iter().chain(remote_branches) {
        println!(" * {}", branch);
    }

    print!("\nEnter y to confirm: ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    Ok(match user_input.trim().to_lowercase().as_str() {
        "y" => true,
        _ => false,
    })
}

fn delete_branches(branches_to_delete: &[GitBranch], remote_name: &str) -> Result<()> {
    for branch in branches_to_delete {
        branch.delete()?;
    }

    prune_branches(remote_name)
}
