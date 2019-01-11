# rust-gi
An command line application for generating gitignores, in rust using [hyper.rs](https://hyper.rs/) and [gitignore.io](https://www.gitignore.io/)

## Installation

Download a pre-built binary for your platform (if one is available) from the [Releases page](https://github.com/Spaceface16518/rust-gi/releases) of this repository.

You can also build it from source code. It is preferable to have `rustup` to manage `rustc` version, but this application has been tested and found compatible with `rustc` version(s)

 - 1.31.1
 - 1.31.0
 - 1.30.1
 - 1.30.0
 - 1.29.2
 - 1.29.1
 - 1.29.0
 - 1.28.0
 - 1.27.2
 - 1.27.1
 - 1.27.0

assuming you are running a unix system.

You will need [`rustup`](https://rustup.rs/) and [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html#installation) to build this package from source. You can click on their links for installation pages.

You can then download the source code from the [Releases page](https://github.com/Spaceface16518/rust-gi/releases). To run, use

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

You can then copy the executable (named `gi`) to any folder on your `PATH` from the directory `target/[debug|release]/` depending on your build. You can then globally use the `gi` command.
