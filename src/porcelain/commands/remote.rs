use anyhow::Result;
use std::path::Path;

pub fn run(_repository: &Path, command: crate::porcelain::RemoteCommands) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    match command {
        crate::porcelain::RemoteCommands::List => {
            let config = repo.config_snapshot();

            for section in config.sections() {
                if let Ok(section_name) = std::str::from_utf8(section.header().name()) {
                    if section_name == "remote" {
                        if let Some(remote_name) = section.header().subsection_name() {
                            if let Ok(name_str) = std::str::from_utf8(remote_name) {
                                println!("{}", name_str);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
