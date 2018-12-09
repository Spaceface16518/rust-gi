#[macro_use]
extern crate log;
extern crate curl;
extern crate pretty_env_logger;

use curl::easy::{Easy, WriteError};
use out::{write, Output};
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

mod app;
mod out;

const PREFIX: &str = "https://www.gitignore.io/api/";

fn main() {
    pretty_env_logger::init();
    let (input, mut out) = proc_args();
    let a_str: String = format(PREFIX, input);
    let mut easy = Easy::new();
    easy.url(a_str.as_str()).unwrap();
    easy.write_function(move |data| write_fn(data, &mut out)).unwrap();
    easy.perform().unwrap();
    info!("Success");
}

fn proc_args() -> (Vec<String>, Output<File>) {
    trace!("Processing args...");
    let matches = app::app().get_matches();

    (
        matches
            .value_of("INPUT")
            .unwrap()
            .split(" ")
            .map(String::from)
            .collect::<Vec<String>>(),
        match matches.value_of("OUTPUT") {
            Some(name) => {
                match OpenOptions::new().create(true).write(true).open(name) {
                    Ok(handle) => Output::Other(handle),
                    Err(_) => {
                        error!("There was an error trying to use that location as an output; using standard output instead");
                        Output::Stdout
                    },
                }
            },
            None => Output::Stdout,
        },
    )
}

fn format(prefix: &str, args: Vec<String>) -> String {
    trace!("Formatting args...");
    format!("{}{}", prefix, args.join(","))
}

fn write_fn<O: Write>(
    data: &[u8],
    out: &mut Output<O>,
) -> Result<usize, WriteError> {
    trace!("Writing data");
    match write(out, data) {
        Ok(i) => Ok(i),
        Err(e) => panic!("Error in write function {:?}", e),
    }
}
