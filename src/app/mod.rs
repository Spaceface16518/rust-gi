extern crate clap;

use self::clap::{App, Arg};

mod statics;
use self::statics::*;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("A command line app to generate gitignores")
        .long_about(LONG_DESC)
        // Update //
        // .arg(Arg::with_name("update")
        // .takes_value(false)
        // .short("u")
        // .long("update")
        // .help("Installs the latest version of `gi`. Only updates with
        // internet connection.")) Output
        .arg(
            Arg::with_name("OUTPUT")
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
                .help("The parameters for gitignore generation"),
        )
}
