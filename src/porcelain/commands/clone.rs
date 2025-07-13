use anyhow::Result;
use std::path::PathBuf;

#[cfg(feature = "gitoxide-core-blocking-client")]
use gitoxide_core as core;

pub fn run(
    repository: String,
    directory: Option<PathBuf>,
    bare: bool,
    depth: Option<u32>,
) -> Result<()> {
    #[cfg(not(feature = "gitoxide-core-blocking-client"))]
    {
        println!("fatal: clone command requires 'gitoxide-core-blocking-client' feature");
        println!("Repository: {}", repository);
        if let Some(dir) = directory {
            println!("Directory: {}", dir.display());
        }
        if bare {
            println!("Bare: true");
        }
        if let Some(d) = depth {
            println!("Depth: {}", d);
        }
        return Ok(());
    }

    #[cfg(feature = "gitoxide-core-blocking-client")]
    {
        let target_dir = directory.unwrap_or_else(|| {
            let name = repository.split('/').last().unwrap_or("repo");
            let name = name.strip_suffix(".git").unwrap_or(name);
            PathBuf::from(name)
        });

        if target_dir.exists() {
            println!(
                "fatal: destination path '{}' already exists and is not an empty directory.",
                target_dir.display()
            );
            return Ok(());
        }

        // TODO: Add proper shallow/depth support by using the correct Shallow enum
        if depth.is_some() {
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

        println!("Cloning into '{}'...", target_dir.display());

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
                    Some(target_dir.clone()),
                    config,
                    progress,
                    &mut std::io::sink(),
                    &mut std::io::sink(),
                    opts,
                )
                .map_err(|e| anyhow::anyhow!("clone failed: {}", e))
            },
        )
    }
}
