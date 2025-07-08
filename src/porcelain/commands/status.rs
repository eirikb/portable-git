use anyhow::Result;
use gitoxide_core as core;
use gix::bstr::{BString, ByteSlice};
use std::path::PathBuf;

pub fn run(_repository: &PathBuf, short: bool, untracked_files: bool) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let head = repo.head()?;
    let is_unborn = head.is_unborn();
    let branch_name = if is_unborn {
        let config = repo.config_snapshot();
        let default_branch = config
            .string("init.defaultBranch")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "master".to_string());
        default_branch
    } else {
        head.referent_name()
            .and_then(|name| name.shorten().to_str().ok())
            .unwrap_or("HEAD")
            .to_string()
    };

    let mut out = Vec::new();
    let mut err = Vec::new();

    let pathspec: Vec<BString> = Vec::new();
    let status_result = core::repository::status::show(
        repo,
        pathspec,
        &mut out,
        &mut err,
        gix::progress::Discard,
        core::repository::status::Options {
            format: if short {
                core::repository::status::Format::PorcelainV2
            } else {
                core::repository::status::Format::Simplified
            },
            ignored: if untracked_files {
                Some(core::repository::status::Ignored::Collapsed)
            } else {
                None
            },
            output_format: core::OutputFormat::Human,
            statistics: false,
            thread_limit: None,
            allow_write: false,
            index_worktree_renames: None,
            submodules: Some(core::repository::status::Submodules::None),
        },
    );

    match status_result {
        Ok(_) => {
            let status_output = String::from_utf8_lossy(&out);

            if short {
                print!("{}", status_output);
            } else {
                println!("On branch {}", branch_name);

                if is_unborn {
                    println!("\nNo commits yet");
                }

                if status_output.trim().is_empty() {
                    if is_unborn {
                        println!(
                            "\nnothing to commit (create/copy files and use \"git add\" to track)"
                        );
                    } else {
                        println!("\nnothing to commit, working tree clean");
                    }
                } else {
                    println!();
                    print!("{}", status_output);
                }
            }
        }
        Err(e) => {
            eprintln!("error: {}", e);
        }
    }

    Ok(())
}
