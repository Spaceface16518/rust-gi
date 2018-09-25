extern crate clap;

use self::clap::{App, Arg, ArgMatches, SubCommand};

const long_desc: &str =
"This is a command line app to generate gitignores. You can use it easily right
 along with command line git with the `gi` command. It uses the public API to 
 generate gitignores, and therefore requires internet connection. Requests can 
 sometimes be cached for offline usage.";

pub fn n_app<'a, 'b>() -> App<'a, 'b> {
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
    .arg(Arg::with_name("install")
    .long("install")
    .short("l")
    .help("Globally installs (or reinstalls) this binary as a global binary.")
    .required(false)
    .takes_value(false))
}
