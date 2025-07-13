// COPIED FROM: https://github.com/Byron/gitoxide/blob/main/gitoxide-core/src/repository/cat.rs
use anyhow::{Context, Result};
use std::path::Path;

pub enum TreeMode {
    Pretty,
}

// COPIED FROM: https://github.com/Byron/gitoxide/blob/main/gitoxide-core/src/repository/cat.rs
pub fn display_object(
    _repo: &gix::Repository,
    spec: gix::revision::Spec<'_>,
    tree_mode: TreeMode,
    mut out: impl std::io::Write,
) -> anyhow::Result<()> {
    let id = spec
        .single()
        .context("rev-spec must resolve to a single object")?;
    let header = id.header()?;
    match header.kind() {
        gix::object::Kind::Tree => {
            if matches!(tree_mode, TreeMode::Pretty) {
                for entry in id.object()?.into_tree().iter() {
                    writeln!(out, "{}", entry?)?;
                }
            }
        }
        gix::object::Kind::Blob => {
            let object = id.object()?;
            out.write_all(&object.data)?;
        }
        gix::object::Kind::Tag => {
            let object = id.object()?;
            let tag = object.try_into_tag()?;
            writeln!(out, "object {}", tag.target_id()?)?;
            let decoded = tag.decode()?;
            writeln!(out, "type {:?}", decoded.target_kind)?;
            writeln!(out, "tag {}", decoded.name)?;
            if let Some(tagger) = decoded.tagger {
                writeln!(out, "tagger {} <{}>", tagger.name, tagger.email)?;
            }
            writeln!(out)?;
            if !decoded.message.is_empty() {
                writeln!(out, "{}", decoded.message)?;
            }
        }
        gix::object::Kind::Commit => {
            let object = id.object()?;
            let commit = object.try_into_commit()?;

            writeln!(out, "commit {}", id)?;

            let author = commit.author()?;
            writeln!(out, "Author: {} <{}>", author.name, author.email)?;

            let time = author.time;
            writeln!(out, "Date:   {}", time)?;

            writeln!(out)?;
            let message = commit.message()?;
            if let Some(body) = message.body() {
                for line in body.to_string().lines() {
                    writeln!(out, "    {}", line)?;
                }
            }
            writeln!(out)?;
        }
    }
    Ok(())
}

// COPIED FROM: https://github.com/Byron/gitoxide/blob/main/gitoxide-core/src/repository/cat.rs
pub fn cat(repo: gix::Repository, revspec: &str, out: impl std::io::Write) -> anyhow::Result<()> {
    let spec = repo.rev_parse(revspec)?;
    display_object(&repo, spec, TreeMode::Pretty, out)?;
    Ok(())
}

pub fn run(
    _repository: &Path,
    objects: Vec<String>,
    format: Option<String>,
    name_only: bool,
    oneline: bool,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let objects_to_show = if objects.is_empty() {
        vec!["HEAD".to_string()]
    } else {
        objects
    };

    for object_spec in objects_to_show {
        if oneline {
            // For oneline, try to get it as commit and show abbreviated
            match repo.rev_parse_single(object_spec.as_str()) {
                Ok(id) => {
                    if let Ok(object) = id.object() {
                        if let Ok(commit) = object.try_into_commit() {
                            let message = commit.message()?.summary();
                            println!("{} {}", id.to_hex_with_len(7), message);
                        } else {
                            println!("{}", id.to_hex_with_len(7));
                        }
                    }
                }
                Err(_) => {
                    eprintln!("fatal: bad revision '{}'", object_spec);
                }
            }
        } else {
            // Use the copied gitoxide-core cat function
            let mut out = std::io::stdout();
            match cat(repo.clone(), &object_spec, &mut out) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("fatal: bad revision '{}'", object_spec);
                }
            }
        }
    }

    if format.is_some() {
        println!("Note: Custom format not yet implemented");
    }

    if name_only {
        println!("Note: Name-only mode not yet implemented");
    }

    Ok(())
}
