use anyhow::Result;
use std::path::PathBuf;

pub fn run(
    _repository: &PathBuf,
    commits: Vec<String>,
    message: Option<String>,
    no_commit: bool,
    ff_only: bool,
) -> Result<()> {
    let _repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    if commits.is_empty() {
        println!("fatal: no commits specified to merge");
        return Ok(());
    }

    println!("fatal: merge functionality not yet fully implemented");
    println!("Would merge: {:?}", commits);
    if let Some(msg) = message {
        println!("With message: {}", msg);
    }
    if no_commit {
        println!("With --no-commit flag");
    }
    if ff_only {
        println!("With --ff-only flag");
    }

    Ok(())
}
