# portable-git

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://rustup.rs)
[![CI/CD](https://github.com/eirikb/portable-git/actions/workflows/ci-cd.yml/badge.svg?branch=main)](https://github.com/eirikb/portable-git/actions/workflows/ci-cd.yml)
[![GitHub release](https://img.shields.io/github/release/eirikb/portable-git.svg)](https://github.com/eirikb/portable-git/releases)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/eirikb/portable-git#license)

A cross-platform, portable single binary Git implementation.

## Run

The **easiest and recommended way** to use portable-git is through [gg](https://github.com/eirikb/gg), where it's
built-in as the `git` command. Run:

```bash
gg.cmd git
```

More detailed (Linux):

```bash
wget ggcmd.io/gg.cmd
sh gg.cmd git
```

More detailed (Windows):

```powershell
wget ggcmd.io -OutFile gg.cmd
.\gg.cmd git
```

This provides seamless integration and **Just Works™** out of the box.  
With support for updated.

## Project Status: Proof of Concept

This project is currently a **proof of concept** that demonstrates how to create a standalone Git CLI tool.

### Current State

At present, this is essentially a 1:1 copy of the `gix` plumbing commands from
the [GitoxideLabs/gitoxide](https://github.com/GitoxideLabs/gitoxide) project, repackaged as a single binary. The core
Git functionality is unchanged from the original gitoxide implementation.

## Usage

### Available Commands

```
A fast, cross-platform Git implementation

Usage: git [OPTIONS] <COMMAND>

Commands:
  clone     Clone a repository into a new directory
  init      Create an empty Git repository or reinitialize an existing one
  diff      Show changes between commits, commit and working tree, etc
  log       Show commit logs
  status    Show the working tree status
  fetch     Download objects and refs from another repository
  config    Get and set repository or global options
  ls-files  Show information about files in the index and the working tree
  cat-file  Provide content or type and size information for repository objects
  blame     Show what revision and author last modified each line of a file
  merge     Join two or more development histories together
  fsck      Verifies the connectivity and validity of objects in the database
  remote    Manage set of tracked repositories
  help      Print this message or the help of the given subcommand(s)

Options:
  -r, --repository <REPOSITORY>  The repository to access [default: .]
  -c, --config <CONFIG>          Add these values to the configuration in the form of `key=value` or `key`
  -v, --verbose                  Enable verbose output
  -h, --help                     Print help
  -V, --version                  Print version
```

## Attribution

This project incorporates significant code from [GitoxideLabs/gitoxide](https://github.com/GitoxideLabs/gitoxide). See
individual source files for detailed attribution.  
[Gitoxide](https://github.com/GitoxideLabs/gitoxide) provides a solid, pure-Rust Git implementation that we can build
upon. Rather than reimplementing Git from scratch, we're leveraging this excellent foundation to create a portable,
single-binary Git tool that can eventually provide the familiar Git experience users expect.
