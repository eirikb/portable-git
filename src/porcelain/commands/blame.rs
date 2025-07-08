use anyhow::Result;
use gitoxide_core as core;
use std::path::PathBuf;

pub fn run(
    _repository: &PathBuf,
    file: String,
    statistics: bool,
    ranges: Vec<std::ops::RangeInclusive<u32>>,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let mut out = std::io::stdout();
    let mut err = std::io::stderr();

    let diff_algorithm = repo.diff_algorithm()?;

    let file_path = std::ffi::OsString::from(&file);

    core::repository::blame::blame_file(
        repo,
        &file_path,
        gix::blame::Options {
            diff_algorithm,
            range: gix::blame::BlameRanges::from_ranges(ranges),
            since: None,
            rewrites: Some(gix::diff::Rewrites::default()),
            debug_track_path: false,
        },
        &mut out,
        if statistics { Some(&mut err) } else { None },
    )?;

    Ok(())
}
