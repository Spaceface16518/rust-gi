extern crate clap;

use self::clap::{App, Arg, ArgMatches, SubCommand};
use std::env;
use std::ffi::{OsStr, OsString};
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
    .arg(Arg::with_name("PROFILE")
    .required(true)
    .index(1)))
}

pub fn install(profile: &Path) -> Result<(), Error> {
    let var_text = PathBuf::from(env::current_dir().expect("Unable to get current directory from environment"));

    add_PATH_var(profile, OsString::from(var_text.as_os_str()))
}

fn add_PATH_var(target: &Path, path: OsString) -> Result<(), Error> {
    i_add_PATH_var(target, format_path(&Path::new(&path))
}

fn a_open(path: &Path) -> io::Result<File> {
    OpenOptions::new().write(true).append(true).open(path)
}

fn append_to_file(path: &Path, data: &[u8]) -> io::Result<()> {
    let mut file = a_open(path).unwrap();
    file.write_all(data).expect("Unable to write to file");
    Ok(())
}

#[cfg(target_os = "windows")]
fn i_add_PATH_var(target: &Path, var_text: OsString) -> Result<(), Error> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn i_add_PATH_var(target: &Path, var_text: OsString) -> Result<(), Error> {
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn i_add_PATH_var(target: &Path, var_text: OsString) -> Result<(), Error> {
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn format_path(dir_path: &Path) -> OsString {
    OsString::from(format!(
        "export PATH=\"{}:$PATH\"",
        dir_path.to_str().unwrap()
    ))
}

#[cfg(target_os = "windows")]
fn format_path(dir_path: &Path) -> OsString {
    OsString::from(format!(
        "set PATH=\"{};%PATH%\"",
        dir_path.to_str().unwrap()
    ))
}
