extern crate clap;

use self::clap::{App, Arg, SubCommand};
use std::env;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, Error};
use std::path::{Path, PathBuf};

const long_desc: &str =
    "This is a command line app to generate gitignores. You can use it easily right
 along with command line git with the `gi` command. It uses the public API to 
 generate gitignores, and therefore requires internet connection. Requests can 
 sometimes be cached for offline usage.";

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
    .help("Choose a symbolic install directories. A symlink will be created there; PATH can then be linked to the symlink instead of this executable"))

    // Instal::Dir
    .arg(Arg::with_name("dir")
    .long("dir")
    .short("d")
    .help("Choose a directory in which to copy (install) this executable. `gi` will then try to link PATH to that executable instead of this one.")
    .takes_value(true)
    .required(false)
    .number_of_values(1))

    // Install::No Link
    .arg(Arg::with_name("nolink")
    .long("nolink")
    .short("nl")
    .takes_value(false)
    .required(false)
    .help("Turns off automatic linking. `gi` will no longer try and link PATH to an executable"))
    
    // Install::No DirWrap
    .arg(Arg::with_name("nodirwrap")
    .long("nodirwrap")
    .short("nd")
    .help("Turns off wrapping of executable in directories. Besides linking this executable, `gi` would wrap the executable into `gi/bin/gi` for safety.")
    ))
}

fn install(target_os: TargetOS) -> Option<io::Result<()>> {
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
    i_add_PATH_var(&format_path(&Path::new(&path)))
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

enum TargetOS {
    Win,
    Mac,
    Other,
}

/// Lower level function, asks helper function to append raw text to files.
fn i_add_PATH_var(var_text: &OsString) -> Result<(), Error> {
    append_to_file(Path::new("~/.bash_profile"), var_text).unwrap();
    append_to_file(Path::new("~/.profile"), var_text).unwrap();
    append_to_file(Path::new("~/.bashrc"), var_text).unwrap();
    Ok(())
}

fn format_path(dir_path: &Path) -> OsString {
    OsString::from(format!(
        "export PATH=\"{}:$PATH\"",
        dir_path.to_str().unwrap()
    ))
}
