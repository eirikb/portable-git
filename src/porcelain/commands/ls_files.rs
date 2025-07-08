use anyhow::Result;
use gitoxide_core as core;
use std::path::PathBuf;

pub fn run(
    _repository: &PathBuf,
    cached: bool,
    deleted: bool,
    modified: bool,
    others: bool,
    stage: bool,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let mut out = std::io::stdout();
    let mut err = std::io::stderr();

    let show_cached = cached || (!deleted && !modified && !others);

    if show_cached {
        core::repository::index::entries(
            repo.clone(),
            vec![],
            &mut out,
            &mut err,
            core::repository::index::entries::Options {
                format: core::OutputFormat::Human,
                simple: true,
                attributes: None,
                recurse_submodules: false,
                statistics: false,
            },
        )?;
    }

    if deleted {
        println!("Deleted files listing not yet implemented");
    }
    if modified {
        println!("Modified files listing not yet implemented");
    }
    if others {
        println!("Untracked files listing not yet implemented");
    }
    if stage {
        println!("Stage information listing not yet implemented");
    }

    Ok(())
}
