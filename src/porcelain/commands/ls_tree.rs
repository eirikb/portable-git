// COPIED FROM: https://github.com/Byron/gitoxide/blob/main/gitoxide-core/src/repository/tree.rs
use anyhow::Result;
use gix::Tree;
use std::io::{self, BufWriter};
use std::path::Path;

// Helper function to resolve treeish to tree object
fn treeish_to_tree<'repo>(treeish: Option<&str>, repo: &'repo gix::Repository) -> anyhow::Result<Tree<'repo>> {
    match treeish {
        Some(rev) => {
            let obj = repo.rev_parse_single(rev)?.object()?;
            Ok(obj.try_into_tree()?)
        }
        None => {
            let head = repo.head()?.try_into_referent().ok_or_else(|| anyhow::anyhow!("No HEAD reference"))?.peel_to_id_in_place()?;
            let commit = head.object()?.try_into_commit()?;
            Ok(commit.tree()?)
        }
    }
}

// Helper function to format tree entries
fn format_entry(
    out: &mut impl io::Write,
    entry: &gix::objs::tree::EntryRef<'_>,
    filename: &gix::bstr::BStr,
    size: Option<Result<u64, gix::object::find::Error>>,
) -> anyhow::Result<()> {
    use gix::bstr::ByteSlice;
    
    writeln!(
        out,
        "{:0>6o} {:?} {}\t{}{}",
        entry.mode,
        entry.mode.kind(),
        entry.oid,
        filename.as_bstr(),
        if let Some(size) = size {
            match size {
                Ok(s) => format!(" ({})", s),
                Err(_) => " (?)".to_string(),
            }
        } else {
            String::new()
        }
    )?;
    Ok(())
}

mod entries {
    use std::collections::VecDeque;
    use gix::{
        bstr::{BStr, BString, ByteSlice, ByteVec},
        objs::tree::EntryRef,
        traverse::tree::visit::Action,
    };
    use crate::porcelain::commands::ls_tree::format_entry;

    #[derive(Default)]
    pub struct Statistics {
        pub num_trees: usize,
        pub num_links: usize,
        pub num_blobs: usize,
        pub num_blobs_exec: usize,
        pub num_submodules: usize,
        pub num_bytes: u64,
    }

    pub struct Traverse<'repo, 'a> {
        pub stats: Statistics,
        repo: Option<&'repo gix::Repository>,
        out: Option<&'a mut dyn std::io::Write>,
        path: BString,
        path_deque: VecDeque<BString>,
    }

    impl<'repo, 'a> Traverse<'repo, 'a> {
        pub fn new(repo: Option<&'repo gix::Repository>, out: Option<&'a mut dyn std::io::Write>) -> Self {
            Traverse {
                stats: Default::default(),
                repo,
                out,
                path: BString::default(),
                path_deque: VecDeque::new(),
            }
        }

        fn pop_element(&mut self) {
            if let Some(pos) = self.path.rfind_byte(b'/') {
                self.path.resize(pos, 0);
            } else {
                self.path.clear();
            }
        }

        fn push_element(&mut self, name: &BStr) {
            if !self.path.is_empty() {
                self.path.push(b'/');
            }
            self.path.push_str(name);
        }
    }

    impl<'repo> gix::traverse::tree::Visit for Traverse<'repo, '_> {
        fn pop_front_tracked_path_and_set_current(&mut self) {
            if let Some(path) = self.path_deque.pop_front() {
                self.path = path;
            }
        }

        fn push_back_tracked_path_component(&mut self, component: &BStr) {
            self.push_element(component);
            self.path_deque.push_back(self.path.clone());
        }

        fn pop_back_tracked_path_and_set_current(&mut self) {
            if let Some(path) = self.path_deque.pop_back() {
                self.path = path;
            }
        }

        fn push_path_component(&mut self, component: &BStr) {
            self.push_element(component);
        }

        fn pop_path_component(&mut self) {
            self.pop_element();
        }

        fn visit_tree(&mut self, entry: &EntryRef<'_>) -> Action {
            self.stats.num_trees += 1;
            if let Some(out) = self.out.as_mut() {
                let _ = format_entry(out, entry, self.path.as_bstr(), None);
            }
            Action::Continue
        }

        fn visit_nontree(&mut self, entry: &EntryRef<'_>) -> Action {
            use gix::object::tree::EntryKind::*;
            match entry.mode.kind() {
                Tree => unreachable!("BUG"),
                Blob => self.stats.num_blobs += 1,
                BlobExecutable => self.stats.num_blobs_exec += 1,
                Link => self.stats.num_links += 1,
                Commit => self.stats.num_submodules += 1,
            }

            let size = self.repo.and_then(|repo| {
                repo.find_object(entry.oid)
                    .map(|obj| obj.data.len() as u64)
                    .ok()
            });

            if let Some(size) = size {
                self.stats.num_bytes += size;
            }

            if let Some(out) = self.out.as_mut() {
                let _ = format_entry(
                    out,
                    entry,
                    self.path.as_bstr(),
                    size.map(Ok)
                );
            }
            Action::Continue
        }
    }
}

// COPIED FROM: https://github.com/Byron/gitoxide/blob/main/gitoxide-core/src/repository/tree.rs
pub fn entries(
    repo: gix::Repository,
    treeish: Option<&str>,
    recursive: bool,
    extended: bool,
    mut out: impl io::Write,
) -> anyhow::Result<()> {
    let tree = treeish_to_tree(treeish, &repo)?;

    if recursive {
        let mut write = BufWriter::new(out);
        let mut delegate = entries::Traverse::new(extended.then_some(&repo), Some(&mut write));
        tree.traverse().depthfirst(&mut delegate)?;
    } else {
        for entry in tree.iter() {
            let entry = entry?;
            format_entry(
                &mut out,
                &entry.inner,
                entry.inner.filename,
                None, // Extended size info not implemented for now
            )?;
        }
    }

    Ok(())
}

pub fn run(
    _repository: &Path,
    tree_ish: Option<String>,
    _name_only: bool,
    recursive: bool,
    long_format: bool,
) -> Result<()> {
    let repo = match gix::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("fatal: not a git repository (or any of the parent directories): .git");
            return Ok(());
        }
    };

    let tree_ish_str = tree_ish.as_deref();
    
    let mut out = std::io::stdout();

    // Use the copied gitoxide-core entries function
    entries(repo, tree_ish_str, recursive, long_format, &mut out)?;

    Ok(())
}