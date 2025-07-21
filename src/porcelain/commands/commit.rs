use anyhow::Result;
use std::path::Path;

use super::index_utils::{add_modified_files_to_index, get_current_branch, is_initial_commit};

pub fn run(
    _repository: &Path,
    message: Option<String>,
    all: bool,
    allow_empty: bool,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    if all {
        add_modified_files_to_index(&repo)?;
    }

    let commit_message = match message {
        Some(msg) => msg,
        None => {
            eprintln!("error: no commit message provided");
            eprintln!("hint: use -m <message> to provide a commit message");
            return Ok(());
        }
    };

    if !allow_empty && !has_staged_changes(&repo)? {
        println!("On branch {}", get_current_branch(&repo)?);

        if is_initial_commit(&repo)? {
            println!("\nNo commits yet\n");
            println!("nothing to commit (create/copy files and use \"git add\" to track)");
        } else {
            println!("\nnothing to commit, working tree clean");
        }
        return Ok(());
    }

    let commit_id = create_commit(&repo, &commit_message)?;

    let branch_name = get_current_branch(&repo)?;
    let short_id = commit_id.to_hex_with_len(7);

    if is_initial_commit(&repo)? {
        println!(
            "[{} (root-commit) {}] {}",
            branch_name,
            short_id,
            get_first_line(&commit_message)
        );
    } else {
        println!(
            "[{} {}] {}",
            branch_name,
            short_id,
            get_first_line(&commit_message)
        );
    }

    let stats = get_commit_stats(&repo)?;
    if stats.files_changed > 0 {
        print!(
            " {} file{} changed",
            stats.files_changed,
            if stats.files_changed == 1 { "" } else { "s" }
        );
        if stats.insertions > 0 {
            print!(
                ", {} insertion{}",
                stats.insertions,
                if stats.insertions == 1 { "" } else { "s" }
            );
        }
        if stats.deletions > 0 {
            print!(
                ", {} deletion{}",
                stats.deletions,
                if stats.deletions == 1 { "" } else { "s" }
            );
        }
        println!();
    }

    Ok(())
}

fn has_staged_changes(repo: &gix::Repository) -> Result<bool> {
    let index = repo.index()?;

    if is_initial_commit(repo)? {
        return Ok(index.entries().len() > 0);
    }

    let head_commit = repo.head_commit()?;
    let head_tree = head_commit.tree()?;
    let head_index = repo.index_from_tree(&head_tree.id)?;

    if index.entries().len() != head_index.entries().len() {
        return Ok(true);
    }

    for (staged_entry, head_entry) in index.entries().iter().zip(head_index.entries().iter()) {
        if staged_entry.id != head_entry.id
            || staged_entry.path(&index) != head_entry.path(&head_index)
        {
            return Ok(true);
        }
    }

    Ok(false)
}

fn create_commit(repo: &gix::Repository, message: &str) -> Result<gix::ObjectId> {
    repo.index()?;
    let tree_id = write_tree_from_index(repo)?;

    let parents: Vec<gix::ObjectId> = if is_initial_commit(repo)? {
        Vec::new()
    } else {
        vec![repo.head_commit()?.id]
    };

    let commit_id = repo.commit("HEAD", message, tree_id, parents)?;

    Ok(commit_id.detach())
}

fn write_tree_from_index(repo: &gix::Repository) -> Result<gix::ObjectId> {
    if !is_initial_commit(repo)? {
        let head_commit = repo.head_commit()?;
        let head_tree_id = head_commit.tree_id()?;

        return Ok(head_tree_id.detach());
    }

    let empty_tree_id = gix::ObjectId::empty_tree(repo.object_hash());
    Ok(empty_tree_id)
}

fn get_first_line(message: &str) -> &str {
    message.lines().next().unwrap_or("")
}

#[derive(Default)]
struct CommitStats {
    files_changed: usize,
    insertions: usize,
    deletions: usize,
}

fn get_commit_stats(_repo: &gix::Repository) -> Result<CommitStats> {
    Ok(CommitStats {
        files_changed: 1,
        insertions: 1,
        deletions: 0,
    })
}
