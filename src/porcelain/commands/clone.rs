use anyhow::Result;
use gitoxide_core as core;
use std::path::PathBuf;

pub fn run(
    repository: String,
    directory: Option<PathBuf>,
    bare: bool,
    depth: Option<u32>,
    recurse_submodules: bool,
) -> Result<()> {
    run_inner(repository, directory, bare, depth, recurse_submodules, 10)
}

fn run_inner(
    repository: String,
    directory: Option<PathBuf>,
    bare: bool,
    depth: Option<u32>,
    recurse_submodules: bool,
    ttl: usize,
) -> Result<()> {
    if ttl == 0 {
        println!("Warning: Maximum submodule depth reached, skipping further submodules");
        return Ok(());
    }

    let target_dir = directory.unwrap_or_else(|| {
        let name = repository.split('/').last().unwrap_or("repo");
        let name = name.strip_suffix(".git").unwrap_or(name);
        PathBuf::from(name)
    });

    let is_submodule = ttl < 10;

    if target_dir.exists() {
        if is_submodule {
            let git_dir = target_dir.join(".git");
            if git_dir.exists() {
                println!("Submodule already cloned, skipping");
                return Ok(());
            } else {
                std::fs::remove_dir_all(&target_dir)
                    .map_err(|e| anyhow::anyhow!("Failed to remove empty directory: {}", e))?;
            }
        } else {
            println!(
                "fatal: destination path '{}' already exists and is not an empty directory.",
                target_dir.display()
            );
            return Ok(());
        }
    }

    // TODO: Add proper shallow/depth support by using the correct Shallow enum
    if depth.is_some() && !is_submodule {
        println!("fatal: --depth option not yet implemented");
        return Ok(());
    }

    let opts = core::repository::clone::Options {
        format: core::OutputFormat::Human,
        bare,
        handshake_info: false,
        no_tags: false,
        ref_name: None,
        shallow: Default::default(),
    };

    let config: Vec<gix::bstr::BString> = vec![];

    if !is_submodule {
        println!("Cloning into '{}'...", target_dir.display());
    }

    if is_submodule {
        // Had issues with fancy progress bars in submodules
        use gix::progress::Discard;
        let mut progress = Discard;

        core::repository::clone(
            repository,
            Some(target_dir.clone()),
            config,
            &mut progress,
            &mut std::io::sink(),
            &mut std::io::sink(),
            opts,
        )
        .map_err(|e| anyhow::anyhow!("clone failed: {}", e))?;
    } else {
        let target_dir_clone = target_dir.clone();
        crate::shared::pretty::prepare_and_run(
            "clone",
            false,
            true,
            false,
            false,
            None,
            move |progress, _out, _err| {
                core::repository::clone(
                    repository,
                    Some(target_dir_clone),
                    config,
                    progress,
                    &mut std::io::sink(),
                    &mut std::io::sink(),
                    opts,
                )
                .map_err(|e| anyhow::anyhow!("clone failed: {}", e))
            },
        )?;
    }

    if recurse_submodules {
        let repo = gix::discover(&target_dir)
            .map_err(|e| anyhow::anyhow!("Failed to open repository: {}", e))?;

        let Some(submodules) = repo
            .submodules()
            .map_err(|e| anyhow::anyhow!("Failed to read submodules: {}", e))?
        else {
            return Ok(());
        };

        let submodules: Vec<_> = submodules.collect();

        if !submodules.is_empty() {
            println!("Cloning submodules...");

            for submodule in submodules {
                let path = submodule
                    .path()
                    .map_err(|e| anyhow::anyhow!("Failed to get submodule path: {}", e))?
                    .to_string();
                let url = submodule
                    .url()
                    .map_err(|e| anyhow::anyhow!("Failed to get submodule URL: {}", e))?
                    .to_string();

                let submodule_dir = target_dir.join(&path);
                println!("Cloning submodule '{}' from '{}'...", path, url);

                run_inner(url, Some(submodule_dir), false, None, true, ttl - 1)?;
            }
        }
    }

    Ok(())
}
