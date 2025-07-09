use anyhow::Result;
use std::path::Path;

pub fn run(_repository: &Path, verbose: bool) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let config = repo.config_snapshot();

    for section in config.sections() {
        if let Ok(section_name) = std::str::from_utf8(section.header().name()) {
            if section_name == "remote" {
                if let Some(remote_name) = section.header().subsection_name() {
                    if let Ok(name_str) = std::str::from_utf8(remote_name) {
                        if verbose {
                            let fetch_key = format!("remote.{}.url", name_str);
                            let push_key = format!("remote.{}.pushurl", name_str);

                            if let Some(fetch_url) = config.string(&fetch_key) {
                                println!("{}\t{} (fetch)", name_str, fetch_url);
                            }

                            let push_url = config
                                .string(&push_key)
                                .or_else(|| config.string(&fetch_key));

                            if let Some(push_url) = push_url {
                                println!("{}\t{} (push)", name_str, push_url);
                            }
                        } else {
                            println!("{}", name_str);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
