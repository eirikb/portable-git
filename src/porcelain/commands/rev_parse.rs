use anyhow::Result;
use std::path::Path;

pub fn run(
    _repository: &Path,
    revisions: Vec<String>,
    short: bool,
    _verify: bool,
    quiet: bool,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            if !quiet {
                println!("fatal: not a git repository (or any of the parent directories): .git");
            }
            return Ok(());
        }
    };

    if revisions.is_empty() {
        if !quiet {
            println!("fatal: bad revision 'HEAD'");
        }
        return Ok(());
    }

    for revision in revisions {
        match repo.rev_parse_single(revision.as_str()) {
            Ok(object) => {
                let id = object.detach();
                if short {
                    println!("{}", id.to_hex_with_len(7));
                } else {
                    println!("{}", id);
                }
            }
            Err(_) => {
                if !quiet {
                    eprintln!("fatal: bad revision '{}'", revision);
                }
            }
        }
    }

    Ok(())
}