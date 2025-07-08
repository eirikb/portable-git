use anyhow::Result;
use gitoxide_core as core;
use std::path::Path;

pub fn run(
    _repository: &Path,
    object: String,
    _show_type: bool,
    _show_size: bool,
    _pretty_print: bool,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let mut out = Vec::new();

    let result = core::repository::cat(repo, &object, &mut out);

    match result {
        Ok(_) => {
            let stdout_output = String::from_utf8_lossy(&out);
            if !stdout_output.is_empty() {
                print!("{}", stdout_output);
            }
        }
        Err(e) => {
            println!("fatal: {}", e);
        }
    }

    Ok(())
}
