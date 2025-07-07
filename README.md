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

## Attribution

This project incorporates significant code from [GitoxideLabs/gitoxide](https://github.com/GitoxideLabs/gitoxide). See
individual source files for detailed attribution.
