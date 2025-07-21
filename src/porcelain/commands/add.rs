use anyhow::Result;
use std::path::{Path, PathBuf};

use super::index_utils::{add_file_to_index, add_modified_files_to_index};

pub fn run(_repository: &Path, pathspec: Vec<PathBuf>, all: bool, update: bool) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    if all {
        add_all_files(&repo)?;
    } else if update {
        add_modified_files_to_index(&repo)?;
    } else if pathspec.is_empty() {
        println!("Nothing specified, nothing added.");
        println!("hint: Maybe you wanted to say 'git add .'?");
        println!("hint: Turn this message off by running");
        println!("hint: \"git config advice.addEmptyPathspec false\"");
        return Ok(());
    } else {
        add_pathspec_files(&repo, pathspec)?;
    }

    Ok(())
}

fn add_all_files(repo: &gix::Repository) -> Result<()> {
    let mut index = repo.index_or_load_from_head_or_empty()?.into_owned();
    let mut added_files = 0;

    let status_iter = repo
        .status(gix::progress::Discard)?
        .into_index_worktree_iter(Vec::new())?;

    for item in status_iter {
        match item? {
            gix::status::index_worktree::Item::Modification { rela_path, .. } => {
                let full_path = repo
                    .workdir()
                    .unwrap()
                    .join(gix::path::from_bstr(&rela_path));
                if add_file_to_index(repo, &full_path, &mut index)? {
                    added_files += 1;
                }
            }
            gix::status::index_worktree::Item::DirectoryContents { entry, .. } => {
                let full_path = repo
                    .workdir()
                    .unwrap()
                    .join(gix::path::from_bstr(&entry.rela_path));
                if add_file_to_index(repo, &full_path, &mut index)? {
                    added_files += 1;
                }
            }
            gix::status::index_worktree::Item::Rewrite { dirwalk_entry, .. } => {
                let full_path = repo
                    .workdir()
                    .unwrap()
                    .join(gix::path::from_bstr(&dirwalk_entry.rela_path));
                if add_file_to_index(repo, &full_path, &mut index)? {
                    added_files += 1;
                }
            }
        }
    }

    if added_files > 0 {
        index.write(Default::default())?;
    }

    Ok(())
}

fn add_pathspec_files(repo: &gix::Repository, pathspecs: Vec<PathBuf>) -> Result<()> {
    let mut index = repo.index_or_load_from_head_or_empty()?.into_owned();
    let mut added_files = 0;

    let work_dir = repo.workdir().unwrap();

    for pathspec in pathspecs {
        let full_path = if pathspec.is_absolute() {
            pathspec.clone()
        } else {
            work_dir.join(&pathspec)
        };

        if full_path.exists() {
            if full_path.is_file() {
                if add_file_to_index(repo, &full_path, &mut index)? {
                    added_files += 1;
                }
            } else if full_path.is_dir() {
                for entry in walkdir::WalkDir::new(&full_path)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                {
                    if add_file_to_index(repo, entry.path(), &mut index)? {
                        added_files += 1;
                    }
                }
            }
        } else {
            eprintln!("pathspec '{}' did not match any files", pathspec.display());
        }
    }

    if added_files > 0 {
        index.write(Default::default())?;
    }

    Ok(())
}
