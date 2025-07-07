# portable-git

A cross-platform, portable single binary Git implementation.

## Project Status: Proof of Concept

This project is currently a **proof of concept** that demonstrates how to create a standalone Git CLI tool.

### Current State

At present, this is essentially a 1:1 copy of the `gix` plumbing commands from
the [GitoxideLabs/gitoxide](https://github.com/GitoxideLabs/gitoxide) project, repackaged as a single binary. The core
Git functionality is unchanged from the original gitoxide implementation.

### Vision & Roadmap

Our goal is to gradually transform this foundation into a Git CLI that matches the familiar interface and behavior of
the original Git command-line tool. Starting with the robust, well-tested gitoxide codebase as our foundation, we plan
to:

1. **Phase 1** (Current): Direct integration of gitoxide's `gix` plumbing commands
2. **Phase 2**: Add porcelain commands with Git-compatible interfaces
3. **Phase 3**: Implement Git-style command aliases and behavior
4. **Phase 4**: Achieve full compatibility with standard Git workflows

### Why Start with Gitoxide?

[Gitoxide](https://github.com/GitoxideLabs/gitoxide) provides a solid, pure-Rust Git implementation that we can build
upon. Rather than reimplementing Git from scratch, we're leveraging this excellent foundation to create a portable,
single-binary Git tool that can eventually provide the familiar Git experience users expect.

## Usage

### Available Commands

```
The git underworld

Usage: git [OPTIONS] <COMMAND>

Commands:
  archive       Subcommands for creating worktree archives
  clean         Remove untracked files from the working tree
  commit-graph  Subcommands for interacting with commit-graph files
  odb           Interact with the object database
  fsck          Check for missing objects
  tree          Interact with tree objects
  commit        Interact with commit objects
  verify        Verify the integrity of the entire repository
  revision      Query and obtain information about revisions [aliases: rev, r]
  credential    A program just like `git credential`
  fetch         Fetch data from remotes and store it in the repository
  clone         Clone a repository into a new directory
  mailmap       Interact with the mailmap
  remote        Interact with the remote hosts [aliases: remotes]
  attributes    Interact with the attribute files like .gitattributes [aliases: attrs]
  exclude       Interact with the exclude files like .gitignore
  index         Interact with a worktree index like .git/index
  submodule     Interact with submodules
  cat           Show whatever object is at the given spec
  is-clean      Check for changes in the repository, treating this as an error
  is-changed    Check for changes in the repository, treating their absence as an error
  config-tree   Show which git configuration values are used or planned
  status        Compute repository status similar to `git status`
  config        Print all entries in a configuration file or access other sub-commands
  corpus        Run algorithms on a corpus of git repositories and store their results for later analysis
  merge-base    A command for calculating all merge-bases
  merge         Perform merges of various kinds
  env           Print paths relevant to the Git installation
  diff          Print all changes between two objects
  log           List all commits in a repository, optionally limited to those that change a given path
  worktree      Commands for handling worktrees
  free          Subcommands that need no Git repository to run [aliases: no-repo]
  blame         Blame lines in a file
  completions   Generate shell completions to stdout or a directory [aliases: generate-completions, shell-completions]
  help          Print this message or the help of the given subcommand(s)

Options:
  -r, --repository <REPOSITORY>
          The repository to access
          
          [default: .]

  -c, --config <CONFIG>
          Add these values to the configuration in the form of `key=value` or `key`.
          
          For example, if `key` is `core.abbrev`, set configuration like `[core] abbrev = key`, or `remote.origin.url = foo` to set `[remote "origin"] url = foo`.

  -t, --threads <THREADS>
          The amount of threads to use for some operations.
          
          If unset, or the value is 0, there is no limit and all logical cores can be used.

  -v, --verbose
          Display verbose messages and progress information

      --trace
          Display structured `tracing` output in a tree-like structure

      --no-verbose
          Turn off verbose message display for commands where these are shown by default

  -s, --strict
          Don't default malformed configuration flags, but show an error instead. Ignore IO errors as well.
          
          Note that some subcommands use strict mode by default.

  -f, --format <FORMAT>
          Determine the format to use when outputting statistics
          
          [default: human]
          [possible values: human, json]

      --object-hash <OBJECT_HASH>
          The object format to assume when reading files that don't inherently know about it, or when writing files
          
          [default: SHA1]
          [possible values: SHA1]

  -h, --help
          Print help (see a summary with '-h')
```

### Example Commands

- **Check repository status**: `git status`
- **View commit history**: `git log`
- **Compare changes**: `git diff`
- **Verify repository integrity**: `git verify`
- **Clone a repository**: `git clone <url>`
- **Get help for any command**: `git <command> --help`

## Attribution

This project incorporates significant code from [GitoxideLabs/gitoxide](https://github.com/GitoxideLabs/gitoxide). See
individual source files for detailed attribution.
