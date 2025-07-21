use anyhow::Result;
use gix::bstr::ByteSlice;
use std::path::Path;

/// Add a file to the index with proper metadata and blob creation
pub fn add_file_to_index(
    repo: &gix::Repository,
    file_path: &Path,
    index: &mut gix::index::File,
) -> Result<bool> {
    let work_dir = repo.workdir().unwrap();

    let relative_path = match file_path.strip_prefix(work_dir) {
        Ok(path) => path,
        Err(_) => {
            eprintln!("warning: '{}' is outside repository", file_path.display());
            return Ok(false);
        }
    };

    if relative_path.starts_with(".git") {
        return Ok(false);
    }

    let file_content = match std::fs::read(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("error: unable to read '{}': {}", file_path.display(), e);
            return Ok(false);
        }
    };

    let blob_id = repo.write_blob(&file_content)?;

    let metadata = std::fs::metadata(file_path)?;
    let stat = gix::index::entry::Stat {
        mtime: gix::index::entry::stat::Time {
            secs: metadata
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as u32,
            nsecs: 0,
        },
        ctime: gix::index::entry::stat::Time {
            secs: metadata
                .modified()?
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as u32,
            nsecs: 0,
        },
        dev: 0,
        ino: 0,
        uid: 0,
        gid: 0,
        size: metadata.len() as u32,
    };

    let mode = if metadata.is_file() {
        gix::index::entry::Mode::FILE
    } else if metadata.is_symlink() {
        gix::index::entry::Mode::SYMLINK
    } else {
        gix::index::entry::Mode::FILE
    };

    let path_bstr = gix::path::into_bstr(relative_path);

    index.dangerously_push_entry(
        stat,
        blob_id.detach(),
        gix::index::entry::Flags::empty(),
        mode,
        &path_bstr,
    );

    index.sort_entries();

    Ok(true)
}

/// Add all modified files to the index (for --update and commit -a)
pub fn add_modified_files_to_index(repo: &gix::Repository) -> Result<usize> {
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
            _ => {} // Skip untracked files for modifications-only mode
        }
    }

    if added_files > 0 {
        index.write(Default::default())?;
    }

    Ok(added_files)
}

/// Check if repository is in the initial commit state (no commits yet)
pub fn is_initial_commit(repo: &gix::Repository) -> Result<bool> {
    Ok(repo.head()?.is_unborn())
}

/// Get the current branch name
pub fn get_current_branch(repo: &gix::Repository) -> Result<String> {
    let head = repo.head()?;

    if head.is_unborn() {
        let config = repo.config_snapshot();
        let default_branch = config
            .string("init.defaultBranch")
            .map(|s| s.to_str_lossy().to_string())
            .unwrap_or_else(|| "master".to_string());
        Ok(default_branch)
    } else {
        Ok(head
            .referent_name()
            .and_then(|name| name.shorten().to_str().ok())
            .unwrap_or("HEAD")
            .to_string())
    }
}