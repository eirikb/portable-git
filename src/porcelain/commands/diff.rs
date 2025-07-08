use anyhow::Result;
use gitoxide_core as core;
use std::path::PathBuf;

pub fn run(pathspec: Vec<PathBuf>, cached: bool) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let mut out = std::io::stdout();

    if cached {
        let head_tree = match repo.head_tree_id() {
            Ok(tree_id) => format!("{}", tree_id),
            Err(_) => {
                println!("fatal: ambiguous argument 'HEAD': unknown revision or path not in the working tree");
                return Ok(());
            }
        };

        // TODO: Hmmm... this is a bit of a hack, we should ideally use the index tree
        let empty_tree = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

        core::repository::diff::tree(repo, &mut out, head_tree.into(), empty_tree.into())?;
    } else {
        if pathspec.is_empty() {
            println!("Working tree diff not yet fully implemented");
            println!("Use 'git status' to see changed files");
        } else {
            println!("Working tree diff for specific paths not yet implemented");
            for path in &pathspec {
                println!("Would show diff for: {}", path.display());
            }
        }
    }

    Ok(())
}
