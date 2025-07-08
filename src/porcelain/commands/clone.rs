use anyhow::Result;
use gitoxide_core as core;
use gix::bstr::BString;
use std::path::PathBuf;

pub fn run(
    repository: String,
    directory: Option<PathBuf>,
    bare: bool,
    depth: Option<u32>,
) -> Result<()> {
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

    let config: Vec<BString> = vec![];

    println!("Cloning into '{}'...", target_dir.display());

    let mut out = Vec::new();
    let mut err = Vec::new();

    let result = core::repository::clone(
        repository,
        Some(target_dir.clone()),
        config,
        gix::progress::Discard,
        &mut out,
        &mut err,
        opts,
    );

    match result {
        Ok(_) => {
            let error_output = String::from_utf8_lossy(&err);
            if !error_output.is_empty() {
                for line in error_output.lines() {
                    if line.contains("error") || line.contains("fatal") || line.contains("warning")
                    {
                        eprintln!("{}", line);
                    }
                }
            }
        }
        Err(e) => {
            println!("fatal: {}", e);
        }
    }

    Ok(())
}
