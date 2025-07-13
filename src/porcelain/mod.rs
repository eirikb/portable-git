use anyhow::Result;
use clap::{Parser, Subcommand};
use gix::bstr::BString;
use std::path::PathBuf;

pub mod commands;

#[derive(Debug, Parser)]
#[clap(
    name = "git",
    about = "A fast, cross-platform Git implementation",
    version
)]
#[clap(subcommand_required = true)]
#[clap(arg_required_else_help = true)]
pub struct Args {
    /// The repository to access
    #[clap(short = 'r', long, default_value = ".")]
    pub repository: PathBuf,

    /// Add these values to the configuration in the form of `key=value` or `key`
    #[clap(long, short = 'c', value_parser = crate::shared::AsBString)]
    pub config: Vec<BString>,

    /// Enable verbose output
    #[clap(long, short = 'v')]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clone a repository into a new directory
    #[clap(display_order = 1)]
    Clone {
        /// Repository to clone
        repository: String,
        /// Directory to clone into
        directory: Option<PathBuf>,
        /// Create a bare repository
        #[clap(long)]
        bare: bool,
        /// Number of commits to fetch
        #[clap(long)]
        depth: Option<u32>,
    },

    /// Create an empty Git repository or reinitialize an existing one
    #[clap(display_order = 2)]
    Init {
        /// Directory to create repository in
        #[clap(default_value = ".")]
        path: PathBuf,
        /// Create a bare repository
        #[clap(long)]
        bare: bool,
    },

    /// Add file contents to the index
    #[clap(display_order = 3)]
    Add {
        /// Files to add to the index
        pathspec: Vec<PathBuf>,
        /// Add all modified and deleted files
        #[clap(short = 'A', long)]
        all: bool,
        /// Add changes from all tracked and untracked files
        #[clap(short = 'a', long)]
        update: bool,
        /// Allow adding otherwise ignored files
        #[clap(short = 'f', long)]
        force: bool,
        /// Don't actually add the files, just show what would be done
        #[clap(short = 'n', long)]
        dry_run: bool,
    },

    /// Record changes to the repository
    #[clap(display_order = 4)]
    Commit {
        /// Use the given message as the commit message
        #[clap(short = 'm', long)]
        message: Option<String>,
        /// Automatically stage files that have been modified and deleted
        #[clap(short = 'a', long)]
        all: bool,
        /// Replace the tip of the current branch by creating a new commit
        #[clap(long)]
        amend: bool,
        /// Allow empty commits
        #[clap(long)]
        allow_empty: bool,
        /// Override the commit author
        #[clap(long)]
        author: Option<String>,
    },

    /// Show changes between commits, commit and working tree, etc
    #[clap(display_order = 5)]
    Diff {
        /// Files to compare
        pathspec: Vec<PathBuf>,
        /// Compare against staging area
        #[clap(long)]
        cached: bool,
        /// Generate diff with given number of context lines
        #[clap(short = 'U', long)]
        unified: Option<u32>,
    },

    /// Show commit logs
    #[clap(display_order = 6)]
    Log {
        /// Number of commits to show
        #[clap(short = 'n', long)]
        max_count: Option<usize>,
        /// Show commits in one line
        #[clap(long)]
        oneline: bool,
        /// Show commit graph
        #[clap(long)]
        graph: bool,
    },

    /// Show the working tree status
    #[clap(display_order = 7)]
    Status {
        /// Give the output in a short format
        #[clap(short = 's', long)]
        short: bool,
        /// Show untracked files
        #[clap(short = 'u', long)]
        untracked_files: bool,
    },

    /// Switch branches or restore working tree files
    #[clap(display_order = 8)]
    Checkout {
        /// The branch or commit to check out
        branch: Option<String>,
        /// Create a new branch
        #[clap(short = 'b', long)]
        new_branch: Option<String>,
        /// Force checkout (throw away local modifications)
        #[clap(short = 'f', long)]
        force: bool,
        /// Files to checkout
        #[clap(last = true)]
        paths: Vec<PathBuf>,
    },

    /// List, create, or delete branches
    #[clap(display_order = 9)]
    Branch {
        /// Branch name to create
        branch_name: Option<String>,
        /// List branches
        #[clap(short = 'l', long)]
        list: bool,
        /// Delete a branch
        #[clap(short = 'd', long)]
        delete: Option<String>,
        /// List remote-tracking branches
        #[clap(short = 'r', long)]
        remote: bool,
        /// Show sha1 and commit subject line for each head
        #[clap(short = 'v', long)]
        verbose: bool,
    },

    /// Download objects and refs from another repository
    #[clap(display_order = 10)]
    Fetch {
        /// Remote name or URL to fetch from
        remote: Option<String>,
        /// Refspecs to fetch
        #[clap(value_parser = crate::shared::AsBString)]
        refspecs: Vec<BString>,
        /// Show what would be done, without making any changes
        #[clap(long, short = 'n')]
        dry_run: bool,
        /// Show additional information
        #[clap(long, short = 'v')]
        verbose: bool,
        /// Fetch all remotes
        #[clap(long)]
        all: bool,
    },

    /// Update remote refs along with associated objects
    #[clap(display_order = 11)]
    Push {
        /// Repository to push to
        remote: Option<String>,
        /// Refspecs to push
        #[clap(value_parser = crate::shared::AsBString)]
        refspecs: Vec<BString>,
        /// Show what would be done, without making any changes
        #[clap(long, short = 'n')]
        dry_run: bool,
        /// Show additional information
        #[clap(long, short = 'v')]
        verbose: bool,
        /// Force update of remote refs
        #[clap(long, short = 'f')]
        force: bool,
        /// Set up upstream tracking
        #[clap(long, short = 'u')]
        set_upstream: bool,
    },

    /// Fetch from and integrate with another repository or a local branch
    #[clap(display_order = 12)]
    Pull {
        /// Repository to pull from
        remote: Option<String>,
        /// Branch to pull
        branch: Option<String>,
        /// Show what would be done, without making any changes
        #[clap(long, short = 'n')]
        dry_run: bool,
        /// Show additional information
        #[clap(long, short = 'v')]
        verbose: bool,
        /// Create a merge commit even when fast-forward is possible
        #[clap(long)]
        no_ff: bool,
        /// Only fast-forward merges are allowed
        #[clap(long)]
        ff_only: bool,
        /// Use rebase instead of merge
        #[clap(long, short = 'r')]
        rebase: bool,
    },

    /// Join two or more development histories together
    #[clap(display_order = 13)]
    Merge {
        /// Commits to merge into current branch
        commits: Vec<String>,
        /// Merge commit message
        #[clap(short = 'm', long)]
        message: Option<String>,
        /// Perform the merge but don't commit
        #[clap(long)]
        no_commit: bool,
        /// Fast-forward only
        #[clap(long)]
        ff_only: bool,
    },

    /// Reset current HEAD to the specified state
    #[clap(display_order = 14)]
    Reset {
        /// Commit to reset to
        commit: Option<String>,
        /// Reset index but not working tree (default)
        #[clap(long, conflicts_with_all = &["soft", "hard"])]
        mixed: bool,
        /// Only reset HEAD
        #[clap(long, conflicts_with_all = &["mixed", "hard"])]
        soft: bool,
        /// Reset HEAD, index, and working tree
        #[clap(long, conflicts_with_all = &["soft", "mixed"])]
        hard: bool,
    },

    /// Get and set repository or global options
    #[clap(display_order = 15)]
    Config {
        /// Config key to get or set
        key: Option<String>,
        /// Value to set (if not provided, will get the key)
        value: Option<String>,
        /// List all configuration entries
        #[clap(long, short = 'l')]
        list: bool,
        /// Show only global configuration
        #[clap(long)]
        global: bool,
        /// Show only local configuration
        #[clap(long)]
        local: bool,
    },

    /// Manage set of tracked repositories
    #[clap(display_order = 16)]
    Remote {
        /// Show remote url after name
        #[clap(short, long)]
        verbose: bool,
    },

    /// Access to low-level plumbing commands
    #[clap(display_order = 100, hide = true)]
    Plumbing {
        #[clap(subcommand)]
        command: PlumbingCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum PlumbingCommands {
    /// Show information about files in the index and the working tree
    LsFiles {
        /// Show cached files
        #[clap(long, short = 'c')]
        cached: bool,
        /// Show deleted files
        #[clap(long, short = 'd')]
        deleted: bool,
        /// Show modified files
        #[clap(long, short = 'm')]
        modified: bool,
        /// Show other files
        #[clap(long, short = 'o')]
        others: bool,
        /// Show staged files
        #[clap(long, short = 's')]
        stage: bool,
    },

    /// Provide content or type and size information for repository objects
    CatFile {
        /// The object to display
        object: String,
        /// Show object type
        #[clap(short = 't', long)]
        show_type: bool,
        /// Show object size
        #[clap(short = 's', long)]
        show_size: bool,
        /// Pretty-print the contents
        #[clap(short = 'p', long)]
        pretty_print: bool,
    },

    /// Verifies the connectivity and validity of objects in the database
    Fsck {
        /// Revspec to start checking from
        spec: Option<String>,
        /// Show detailed information
        #[clap(long, short = 'v')]
        verbose: bool,
    },

    /// All original gitoxide plumbing commands
    #[clap(external_subcommand)]
    External(Vec<String>),
}

pub fn main() -> Result<()> {
    let should_interrupt = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    #[allow(unsafe_code)]
    unsafe {
        gix::interrupt::init_handler(1, {
            let should_interrupt = std::sync::Arc::clone(&should_interrupt);
            move || should_interrupt.store(true, std::sync::atomic::Ordering::SeqCst)
        })?;
    }

    let args = Args::parse();

    match args.command {
        Commands::Init { path, bare } => commands::init::run(path, bare),
        Commands::Clone {
            repository,
            directory,
            bare,
            depth,
        } => commands::clone::run(repository, directory, bare, depth),
        Commands::Add {
            pathspec,
            all,
            update,
            force,
            dry_run,
        } => commands::add::run(&args.repository, pathspec, all, update, force, dry_run),
        Commands::Commit {
            message,
            all,
            amend,
            allow_empty,
            author,
        } => commands::commit::run(&args.repository, message, all, amend, allow_empty, author),
        Commands::Diff {
            pathspec,
            cached,
            unified: _unified,
        } => commands::diff::run(pathspec, cached),
        Commands::Log {
            max_count,
            oneline,
            graph,
        } => commands::log::run(max_count, oneline, graph),
        Commands::Status {
            short,
            untracked_files,
        } => commands::status::run(&args.repository, short, untracked_files),
        Commands::Checkout {
            branch,
            new_branch,
            force,
            paths,
        } => commands::checkout::run(&args.repository, branch, new_branch, force, paths),
        Commands::Branch {
            branch_name,
            list,
            delete,
            remote,
            verbose,
        } => commands::branch::run(&args.repository, branch_name, list, delete, remote, verbose),
        Commands::Fetch {
            remote,
            refspecs,
            dry_run,
            verbose,
            all,
        } => commands::fetch::run(&args.repository, remote, refspecs, dry_run, verbose, all),
        Commands::Push {
            remote,
            refspecs,
            dry_run,
            verbose,
            force,
            set_upstream,
        } => commands::push::run(
            &args.repository,
            remote,
            refspecs,
            dry_run,
            verbose,
            force,
            set_upstream,
        ),
        Commands::Pull {
            remote,
            branch,
            dry_run,
            verbose,
            no_ff,
            ff_only,
            rebase,
        } => commands::pull::run(
            &args.repository,
            remote,
            branch,
            dry_run,
            verbose,
            no_ff,
            ff_only,
            rebase,
        ),
        Commands::Merge {
            commits,
            message,
            no_commit,
            ff_only,
        } => commands::merge::run(&args.repository, commits, message, no_commit, ff_only),
        Commands::Reset {
            commit,
            mixed: _,
            soft,
            hard,
        } => {
            let mode = if soft {
                commands::reset::ResetMode::Soft
            } else if hard {
                commands::reset::ResetMode::Hard
            } else {
                commands::reset::ResetMode::Mixed
            };
            commands::reset::run(mode, commit)
        }
        Commands::Config {
            key,
            value,
            list: _list,
            global,
            local,
        } => commands::config::run(&args.repository, key, value, global, local),
        Commands::Remote { verbose } => commands::remote::run(&args.repository, verbose),
        Commands::Plumbing { command } => match command {
            PlumbingCommands::LsFiles {
                cached,
                deleted,
                modified,
                others,
                stage,
            } => {
                commands::ls_files::run(&args.repository, cached, deleted, modified, others, stage)
            }
            PlumbingCommands::CatFile {
                object,
                show_type,
                show_size,
                pretty_print,
            } => commands::cat_file::run(
                &args.repository,
                object,
                show_type,
                show_size,
                pretty_print,
            ),
            PlumbingCommands::Fsck { spec, verbose } => {
                commands::fsck::run(&args.repository, spec, verbose)
            }
            PlumbingCommands::External(_args) => crate::plumbing::main(),
        },
    }
}
