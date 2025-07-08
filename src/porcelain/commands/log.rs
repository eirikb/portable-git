use anyhow::Result;
use gitoxide_core as core;

pub fn run(max_count: Option<usize>, oneline: bool, graph: bool) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let mut out = std::io::stdout();
    let _ = std::io::stderr();

    let spec = std::ffi::OsString::from("HEAD");

    let format = if oneline || graph {
        core::OutputFormat::Human
    } else {
        core::OutputFormat::Human
    };

    if oneline || graph {
        let head = match repo.head_commit() {
            Ok(commit) => commit,
            Err(_) => {
                println!("fatal: your current branch does not have any commits yet");
                return Ok(());
            }
        };

        let mut count = 0;
        let limit = max_count.unwrap_or(usize::MAX);

        let mut current = Some(head);
        while let Some(commit) = current {
            if count >= limit {
                break;
            }

            let hash = commit.id();
            let short_hash = format!("{:.7}", hash);
            let message = commit.message()?.title.to_string();

            if oneline {
                println!("{} {}", short_hash, message);
            } else if graph {
                println!("* {} {}", short_hash, message);
            }

            let mut parent_ids = commit.parent_ids();
            current = if let Some(parent_id) = parent_ids.next() {
                Some(repo.find_commit(parent_id)?)
            } else {
                None
            };
            count += 1;
        }
    } else {
        core::repository::revision::list(
            repo,
            gix::progress::Discard,
            &mut out,
            core::repository::revision::list::Context {
                limit: max_count,
                spec,
                format,
                long_hashes: false,
                text: core::repository::revision::list::Format::Text,
            },
        )?;
    }

    Ok(())
}
