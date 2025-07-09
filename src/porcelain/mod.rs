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

    /// Show changes between commits, commit and working tree, etc
    #[clap(display_order = 4)]
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
    #[clap(display_order = 5)]
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
    #[clap(display_order = 6)]
    Status {
        /// Give the output in a short format
        #[clap(short = 's', long)]
        short: bool,
        /// Show untracked files
        #[clap(short = 'u', long)]
        untracked_files: bool,
    },

    /// Download objects and refs from another repository
    #[clap(display_order = 9)]
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

    /// Get and set repository or global options
    #[clap(display_order = 10)]
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

    /// Show information about files in the index and the working tree
    #[clap(display_order = 11)]
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
    #[clap(display_order = 12)]
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

    /// Show what revision and author last modified each line of a file
    #[clap(display_order = 13)]
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

    /// Join two or more development histories together  
    #[clap(display_order = 14)]
    Merge {
        /// Commits to merge
        commits: Vec<String>,
        /// Commit message for the merge
        #[clap(short = 'm', long)]
        message: Option<String>,
        /// Perform the merge but don't commit
        #[clap(long)]
        no_commit: bool,
        /// Fast-forward only
        #[clap(long)]
        ff_only: bool,
    },

    /// Verifies the connectivity and validity of objects in the database
    #[clap(display_order = 15)]
    Fsck {
        /// Revspec to start checking from
        spec: Option<String>,
        /// Show detailed information
        #[clap(long, short = 'v')]
        verbose: bool,
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
    /// All original gitoxide plumbing commands
    #[clap(external_subcommand)]
    External(Vec<String>),
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Init { path, bare } => commands::init::run(path, bare),
        Commands::Status {
            short,
            untracked_files,
        } => commands::status::run(&args.repository, short, untracked_files),
        Commands::Clone {
            repository,
            directory,
            bare,
            depth,
        } => commands::clone::run(repository, directory, bare, depth),
        Commands::Log {
            max_count,
            oneline,
            graph,
        } => commands::log::run(max_count, oneline, graph),
        Commands::Diff {
            pathspec,
            cached,
            unified: _unified,
        } => commands::diff::run(pathspec, cached),
        Commands::Fetch {
            remote,
            refspecs,
            dry_run,
            verbose,
            all,
        } => commands::fetch::run(&args.repository, remote, refspecs, dry_run, verbose, all),
        Commands::Config {
            key,
            value,
            list: _list,
            global,
            local,
        } => commands::config::run(&args.repository, key, value, global, local),
        Commands::LsFiles {
            cached,
            deleted,
            modified,
            others,
            stage,
        } => commands::ls_files::run(&args.repository, cached, deleted, modified, others, stage),
        Commands::CatFile {
            object,
            show_type,
            show_size,
            pretty_print,
        } => commands::cat_file::run(&args.repository, object, show_type, show_size, pretty_print),
        Commands::Blame {
            file,
            statistics,
            ranges,
        } => commands::blame::run(&args.repository, file, statistics, ranges),
        Commands::Merge {
            commits,
            message,
            no_commit,
            ff_only,
        } => commands::merge::run(&args.repository, commits, message, no_commit, ff_only),
        Commands::Fsck { spec, verbose } => commands::fsck::run(&args.repository, spec, verbose),
        Commands::Remote { verbose } => commands::remote::run(&args.repository, verbose),
        Commands::Plumbing { command } => match command {
            PlumbingCommands::External(_args) => crate::plumbing::main(),
        },
    }
}
