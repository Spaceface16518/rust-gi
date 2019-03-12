# rust-gi

[![Build Status](https://travis-ci.com/Spaceface16518/rust-gi.svg?branch=master)](https://travis-ci.com/Spaceface16518/rust-gi)

An command line application for generating gitignores, in written in pure rust using [hyper.rs](https://hyper.rs/) and the  [gitignore.io](https://www.gitignore.io/) API.

## Installation

Download a pre-built binary for your platform (if one is available) from the [Releases page](https://github.com/Spaceface16518/rust-gi/releases) of this repository.

You can also build it from source code. You will need Rust to build it, which you can install [here](https://www.rust-lang.org/tools/install). If this build is passing, this application has been tested on the stable, beta, and nightly channels on linux, macos, and windows. Check the badge at the [top of the readme](#rust-gi).

You can download the source code from the [Releases page](https://github.com/Spaceface16518/rust-gi/releases) (or clone the repository)

To run `rust-gi`, use

```shell
cargo run <PARAMETERS>
```

where `<PARAMETERS>` is a space separated list of the gitignore templates you want to use.

To just build, use

```shell
cargo build
```

or

```shell
cargo build --release
```

You can then copy the executable (named `gi`) to any folder on your `PATH` from the directory `target/debug/` or `target/release` depending on your build. You can then globally use the `gi` command (replace `cargo run` with `gi` in the above examples)
