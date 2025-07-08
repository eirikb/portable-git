use anyhow::Result;
use gitoxide_core as core;
use std::path::PathBuf;

pub fn run(_repository: &PathBuf, spec: Option<String>, verbose: bool) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let mut out = std::io::stdout();

    if verbose {
        println!("Checking object database...");
    }

    core::repository::fsck(repo, spec, &mut out)?;

    if verbose {
        println!("Fsck completed successfully");
    }

    Ok(())
}
