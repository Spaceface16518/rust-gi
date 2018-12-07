#[macro_use]
extern crate log;
extern crate curl;
extern crate pretty_env_logger;

use curl::easy::{Easy, WriteError};
use std::{
    env::{args, Args},
    io::{stdout, Write},
};

//mod app;

const PREFIX: &str = "https://www.gitignore.io/api/";

fn main() {
    pretty_env_logger::init();
    let argv: Vec<String> = proc_args(args());
    let a_str: String = format(PREFIX, argv);
    let mut easy = Easy::new();
    easy.url(a_str.as_str()).unwrap();
    easy.write_function(|data| write_fn(data, stdout())).unwrap();
    easy.perform().unwrap();
    info!("Success");
}

fn proc_args(argv: Args) -> Vec<String> {
    trace!("Processing args...");
    let mut a: Vec<String> = argv.collect();
    a.remove(0);
    return a
}

fn format(prefix: &str, args: Vec<String>) -> String {
    trace!("Formatting args...");
    format!("{}{}", prefix, args.join(","))
}

fn write_fn<O: Write>(data: &[u8], mut out: O) -> Result<usize, WriteError> {
    trace!("Writing data");
    Ok(out.write(data).unwrap())
}
