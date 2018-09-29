extern crate clap;

use self::clap::{App, Arg, SubCommand};
use std::env;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, Error};
use std::path::{Path, PathBuf};
use std::os::unix::fs::symlink;
use std::os::windows::fs::symlink_file;

mod statics;
use self::statics::*;

pub fn default_app<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .version(env!("CARGO_PKG_VERSION"))
    .about("A command line app to generate gitignores")
    .long_about(long_desc)

    // Update
    // .arg(Arg::with_name("update")
    // .takes_value(false)
    // .short("u")
    // .long("update")
    // .help("Installs the latest version of `gi`. Only updates with internet connection."))

    // Output
    .arg(Arg::with_name("output")
    .long("output")
    .short("o")
    .required(false)
    .takes_value(true))

    // Input
    .arg(Arg::with_name("input")
    .long("input")
    .short("i")
    .required(true)
    .takes_value(true)
    .min_values(1)
    .use_delimiter(true))

    // Install
    .subcommand(SubCommand::with_name("install")

    // Install::Symdir
    .arg(Arg::with_name("symdir")
    .long("symdir")
    .short("s")
    .required(false)
    .takes_value(true)
    .min_values(1)
    .help(SYMDIR_HELP))

    // Instal::Dir
    .arg(Arg::with_name("dir")
    .long("dir")
    .short("d")
    .help(DIR_HELP)
    .takes_value(true)
    .required(false)
    .number_of_values(1))

    // Install::No Link
    .arg(Arg::with_name("nolink")
    .long("nolink")
    .short("nl")
    .takes_value(false)
    .required(false)
    .help(NOLINK_HELP))
    
    // Install::No DirWrap
    .arg(Arg::with_name("nodirwrap")
    .long("nodirwrap")
    .short("nd")
    .help(NODIRWRAP_HELP)
    ))
}

// Dir Wrap

/// Outputs the executable to the place specified by options. Linking to PATH must ne handled externally.
// pub fn output(dir: &Path, wrap: bool, symdir: Vec<&Path>) -> io::Result<()> {

// }

// fn out_syms(wrap: bool, symdir: Vec<&Path>) -> io::Result<()> {

// }

#[cfg(target_os = "windows")]
fn i_out_syms(src: &Path, dst: &Path) -> io::Result<()> {
    symlink_file(src.to_str().unwrap(), dst.to_str().unwrap())
}

#[cfg(not(target_os = "windows"))]
fn i_out_syms(src: &Path, dst: &Path) -> io:: Result<()> {
    symlink(src.to_str().unwrap(), dst.to_str().unwrap())
}


// Install

/// Install functionality, handles the program side of installation. Needs a 
/// handler to be pretty for a user
fn install() -> Option<io::Result<()>> {
    if cfg!(target_os = "windows") {
        return None;
    } else {
        let var_text: PathBuf = PathBuf::from(
            env::current_dir().expect("Unable to get current directory from environment"),
        );
        Some(add_path_var(var_text.as_path()))
    }
}

/// Higher level function, formats the path into the raw text and passes it on
fn add_path_var(path: &Path) -> io::Result<()> {
    i_add_path_var(&format_path(&Path::new(&path)))
}

/// Opens a file in append mode using `io::OpenOptions`
fn a_open(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).append(true).open(path)
}

/// Appends an `OsString` to a file specified by `path`.
fn append_to_file(path: &Path, data: &OsString) -> io::Result<()> {
    let mut file = a_open(path).expect("Unable to open file (in append mode)");
    file.write_all(
        &data
            .to_str()
            .expect("Could not convert data to a `&str`")
            .as_bytes(),
    ).expect("Unable to write to file");
    Ok(())
}

/// Lower level function, asks helper function to append raw text to files.
fn i_add_path_var(var_text: &OsString) -> Result<(), Error> {
    for &i in PROFILES.iter() {
        append_to_file(Path::new(i), var_text).unwrap();
    }
    Ok(())
}

/// Formats path for entry into profiles
fn format_path(dir_path: &Path) -> OsString {
    OsString::from(format!(
        "export PATH=\"{}:$PATH\"",
        dir_path.to_str().unwrap()
    ))
}
