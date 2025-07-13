// COPIED FROM: https://github.com/Byron/gitoxide/blob/main/gitoxide-core/src/repository/commit.rs
use anyhow::{Context, Result};
use std::path::Path;

pub mod describe {
    #[derive(Debug, Clone)]
    pub struct Options {
        pub all_tags: bool,
        pub all_refs: bool,
        pub first_parent: bool,
        pub always: bool,
        pub statistics: bool,
        pub max_candidates: usize,
        pub long_format: bool,
        pub dirty_suffix: Option<String>,
    }

    impl Default for Options {
        fn default() -> Self {
            Self {
                all_tags: false,
                all_refs: false,
                first_parent: false,
                always: false,
                statistics: false,
                max_candidates: 10,
                long_format: false,
                dirty_suffix: None,
            }
        }
    }
}

// COPIED FROM: https://github.com/Byron/gitoxide/blob/main/gitoxide-core/src/repository/commit.rs
pub fn describe(
    mut repo: gix::Repository,
    rev_spec: Option<&str>,
    mut out: impl std::io::Write,
    mut err: impl std::io::Write,
    describe::Options {
        all_tags,
        all_refs,
        first_parent,
        always,
        statistics,
        max_candidates,
        long_format,
        dirty_suffix,
    }: describe::Options,
) -> Result<()> {
    repo.object_cache_size_if_unset(4 * 1024 * 1024);
    let commit = match rev_spec {
        Some(spec) => repo.rev_parse_single(spec)?.object()?.try_into_commit()?,
        None => repo.head_commit()?,
    };
    use gix::commit::describe::SelectRef::*;
    let select_ref = if all_refs {
        AllRefs
    } else if all_tags {
        AllTags
    } else {
        Default::default()
    };
    let resolution = commit
        .describe()
        .names(select_ref)
        .traverse_first_parent(first_parent)
        .id_as_fallback(always)
        .max_candidates(max_candidates)
        .try_resolve()?
        .with_context(|| {
            format!(
                "Did not find a single candidate ref for naming id '{}'",
                commit.id
            )
        })?;

    if statistics {
        writeln!(err, "traversed {} commits", resolution.outcome.commits_seen)?;
    }

    let mut describe_id = resolution.format_with_dirty_suffix(dirty_suffix)?;
    describe_id.long(long_format);

    writeln!(out, "{describe_id}")?;
    Ok(())
}

pub fn run(
    _repository: &Path,
    commit: Option<String>,
    tags: bool,
    all: bool,
    always: bool,
    long: bool,
    exact_match: bool,
    dirty: Option<String>,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let commit_spec = commit.as_deref();

    let mut out = std::io::stdout();
    let mut err = std::io::stderr();

    // Use the copied gitoxide-core describe function
    let result = describe(
        repo,
        commit_spec,
        &mut out,
        &mut err,
        describe::Options {
            all_tags: tags,
            all_refs: all,
            first_parent: false,
            always,
            statistics: false,
            max_candidates: 10,
            long_format: long,
            dirty_suffix: dirty,
        },
    );

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if exact_match {
                // In exact match mode, failure should be silent
                return Ok(());
            }
            eprintln!("error: {}", e);
            Ok(())
        }
    }
}
