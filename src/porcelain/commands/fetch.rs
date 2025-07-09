use anyhow::Result;
use gix::bstr::BString;
use std::path::Path;

#[cfg(feature = "gitoxide-core-blocking-client")]
use gitoxide_core as core;

pub fn run(
    _repository: &Path,
    remote: Option<String>,
    refspecs: Vec<BString>,
    dry_run: bool,
    verbose: bool,
    all: bool,
) -> Result<()> {
    #[cfg(not(feature = "gitoxide-core-blocking-client"))]
    {
        println!("fatal: fetch command requires 'gitoxide-core-blocking-client' feature");
        if let Some(r) = remote {
            println!("Remote: {}", r);
        }
        if !refspecs.is_empty() {
            println!("Refspecs: {:?}", refspecs);
        }
        if dry_run {
            println!("Dry run: true");
        }
        if verbose {
            println!("Verbose: true");
        }
        if all {
            println!("All: true");
        }
        return Ok(());
    }

    #[cfg(feature = "gitoxide-core-blocking-client")]
    {
        let repo = match gix::discover(".") {
            Ok(repo) => repo,
            Err(_) => {
                println!("fatal: not a git repository (or any of the parent directories): .git");
                return Ok(());
            }
        };

        if all {
            println!("fatal: --all flag not yet implemented");
            return Ok(());
        }

        let opts = core::repository::fetch::Options {
            format: core::OutputFormat::Human,
            dry_run,
            remote,
            handshake_info: verbose,
            negotiation_info: verbose,
            open_negotiation_graph: None,
            shallow: Default::default(),
            ref_specs: refspecs,
        };

        let mut out = Vec::new();
        let mut err = Vec::new();

        let result =
            core::repository::fetch(repo, gix::progress::Discard, &mut out, &mut err, opts);

        match result {
            Ok(_) => {
                let stdout_output = String::from_utf8_lossy(&out);
                if !stdout_output.is_empty() {
                    print!("{}", stdout_output);
                }

                let stderr_output = String::from_utf8_lossy(&err);
                if !stderr_output.is_empty() {
                    eprint!("{}", stderr_output);
                }
            }
            Err(e) => {
                println!("fatal: {}", e);
            }
        }

        Ok(())
    }
}
