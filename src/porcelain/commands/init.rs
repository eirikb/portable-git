use anyhow::Result;
use gix::create;
use std::path::PathBuf;

pub fn run(path: PathBuf, bare: bool) -> Result<()> {
    if bare {
        create::into(&path, create::Kind::Bare, create::Options::default())?
    } else {
        create::into(
            &path,
            create::Kind::WithWorktree,
            create::Options::default(),
        )?
    };

    if bare {
        println!("Initialized empty Git repository in {}/", path.display());
    } else {
        println!(
            "Initialized empty Git repository in {}/.git/",
            path.display()
        );
    }

    Ok(())
}
