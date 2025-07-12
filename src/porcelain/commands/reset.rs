use anyhow::Result;
use gix::bstr::ByteSlice;

#[derive(Debug, Clone, Copy)]
pub enum ResetMode {
    Soft,
    Mixed,
    Hard,
}

pub fn run(mode: ResetMode, commit: Option<String>) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let target = commit.as_deref().unwrap_or("HEAD");

    let commit_id = match repo.rev_parse_single(target) {
        Ok(spec) => spec.detach(),
        Err(_) => {
            println!(
                "fatal: ambiguous argument '{}': unknown revision or path not in the working tree.",
                target
            );
            return Ok(());
        }
    };

    let mut head = match repo.head_ref() {
        Ok(Some(head)) => head,
        Ok(None) => {
            println!("fatal: Failed to get HEAD reference");
            return Ok(());
        }
        Err(e) => {
            println!("fatal: {}", e);
            return Ok(());
        }
    };

    match mode {
        ResetMode::Soft => {
            head.set_target_id(commit_id, "reset: moving to HEAD")?;
            println!(
                "HEAD is now at {} {}",
                commit_id.to_hex_with_len(7),
                get_commit_summary(&repo, &commit_id)?
            );
        }
        ResetMode::Mixed => {
            head.set_target_id(commit_id, "reset: moving to HEAD")?;

            let tree_id = repo.find_commit(commit_id)?.tree_id()?;
            reset_index(&repo, tree_id.into())?;

            println!(
                "HEAD is now at {} {}",
                commit_id.to_hex_with_len(7),
                get_commit_summary(&repo, &commit_id)?
            );
        }
        ResetMode::Hard => {
            head.set_target_id(commit_id, "reset: moving to HEAD")?;

            let tree_id = repo.find_commit(commit_id)?.tree_id()?;
            reset_index(&repo, tree_id.into())?;

            reset_worktree(&repo, tree_id.into())?;

            println!(
                "HEAD is now at {} {}",
                commit_id.to_hex_with_len(7),
                get_commit_summary(&repo, &commit_id)?
            );
        }
    }

    Ok(())
}

fn get_commit_summary(repo: &gix::Repository, commit_id: &gix::ObjectId) -> Result<String> {
    let commit = repo.find_commit(*commit_id)?;
    let message = commit.message_raw_sloppy();
    let first_line = message.lines().next().unwrap_or(b"").to_str_lossy();
    Ok(first_line.to_string())
}

fn reset_index(repo: &gix::Repository, tree_id: gix::ObjectId) -> Result<()> {
    let mut index = repo.index_from_tree(&tree_id)?;

    index.write(Default::default())?;

    Ok(())
}

fn reset_worktree(repo: &gix::Repository, tree_id: gix::ObjectId) -> Result<()> {
    let _tree = repo.find_tree(tree_id)?;

    let worktree = repo
        .workdir()
        .ok_or_else(|| anyhow::anyhow!("No worktree found"))?;

    let mut index = repo.index_from_tree(&tree_id)?;

    let opts = gix::worktree::state::checkout::Options {
        overwrite_existing: true,
        ..Default::default()
    };

    gix::worktree::state::checkout(
        &mut index,
        worktree,
        repo.objects.clone(),
        &gix::progress::Discard,
        &gix::progress::Discard,
        &std::sync::atomic::AtomicBool::new(false),
        opts,
    )?;

    Ok(())
}
