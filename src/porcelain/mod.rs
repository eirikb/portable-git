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
        /// Clone all submodules recursively
        #[clap(long)]
        recurse_submodules: bool,
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
        /// Add all modified and new files
        #[clap(short = 'A', long)]
        all: bool,
        /// Add only modified files (not new files)
        #[clap(short = 'u', long)]
        update: bool,
    },

    /// Record changes to the repository
    #[clap(display_order = 4)]
    Commit {
        /// Commit message
        #[clap(short = 'm', long)]
        message: Option<String>,
        /// Add all modified files before committing
        #[clap(short = 'a', long)]
        all: bool,
        /// Allow empty commits
        #[clap(long)]
        allow_empty: bool,
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

    /// Download objects and refs from another repository
    #[clap(display_order = 8)]
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

    /// Join two or more development histories together
    #[clap(display_order = 9)]
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
    #[clap(display_order = 10)]
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
    #[clap(display_order = 11)]
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
    #[clap(display_order = 12)]
    Remote {
        /// Show remote url after name
        #[clap(short, long)]
        verbose: bool,
    },

    /// Show various types of objects
    #[clap(display_order = 13)]
    Show {
        /// Objects to show
        objects: Vec<String>,
        /// Use custom format
        #[clap(long)]
        format: Option<String>,
        /// Show only names
        #[clap(long)]
        name_only: bool,
        /// Show each commit on a single line
        #[clap(long)]
        oneline: bool,
    },

    /// Give an object a human readable name based on an available ref
    #[clap(display_order = 14)]
    Describe {
        /// Committish object names to describe
        commit: Option<String>,
        /// Consider lightweight tags
        #[clap(long)]
        tags: bool,
        /// Consider all refs, not just tags
        #[clap(long)]
        all: bool,
        /// Show hash with short name
        #[clap(long)]
        always: bool,
        /// Always use long format
        #[clap(long)]
        long: bool,
        /// Only output exact matches
        #[clap(long)]
        exact_match: bool,
        /// Append dirty suffix
        #[clap(long)]
        dirty: Option<String>,
    },

    /// Show what revision and author last modified each line of a file
    #[clap(display_order = 15)]
    Blame {
        /// The file to annotate
        file: String,
        /// Show statistics
        #[clap(long, short = 's')]
        statistics: bool,
        /// Only blame lines in the given range
        #[clap(short = 'L', value_parser = crate::shared::AsRange, action = clap::ArgAction::Append)]
        ranges: Vec<std::ops::RangeInclusive<u32>>,
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
            recurse_submodules,
        } => commands::clone::run(repository, directory, bare, depth, recurse_submodules),
        Commands::Add {
            pathspec,
            all,
            update,
        } => commands::add::run(&args.repository, pathspec, all, update),
        Commands::Commit {
            message,
            all,
            allow_empty,
        } => commands::commit::run(&args.repository, message, all, allow_empty),
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
        Commands::Fetch {
            remote,
            refspecs,
            dry_run,
            verbose,
            all,
        } => commands::fetch::run(&args.repository, remote, refspecs, dry_run, verbose, all),
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
        Commands::Show {
            objects,
            format,
            name_only,
            oneline,
        } => commands::show::run(&args.repository, objects, format, name_only, oneline),
        Commands::Describe {
            commit,
            tags,
            all,
            always,
            long,
            exact_match,
            dirty,
        } => commands::describe::run(
            &args.repository,
            commit,
            tags,
            all,
            always,
            long,
            exact_match,
            dirty,
        ),
        Commands::Blame {
            file,
            statistics,
            ranges,
        } => commands::blame::run(&args.repository, file, statistics, ranges),
        Commands::Plumbing { command } => match command {
            PlumbingCommands::External(_args) => crate::plumbing::main(),
        },
    }
}
