extern crate clap;

use self::clap::{App, Arg, SubCommand};
use std::{
    env,
    ffi::OsString,
    fs::{File, OpenOptions},
    io::{self, prelude::*, Error},
    path::{Path, PathBuf},
};

mod statics;
use self::statics::*;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("A command line app to generate gitignores")
        .long_about(long_desc)
        // Update //
        // .arg(Arg::with_name("update")
        // .takes_value(false)
        // .short("u")
        // .long("update")
        // .help("Installs the latest version of `gi`. Only updates with
        // internet connection.")) Output
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .required(false)
                .takes_value(true),
        )
        // Input //
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .index(1)
                .min_values(1)
                .multiple(true)
                .help("The parameters for gitignore generation"),
        )
}
