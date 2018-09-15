extern crate curl;

use curl::easy::{Easy, WriteError};
use std::env::{args, Args};
use std::io::{stdout, Write};

mod cache;

const PREFIX: &str = "https://www.gitignore.io/api/";

fn main() {
    let argv: Vec<String> = proc_args(args());
    let a_str: String = format(PREFIX, argv);
    let mut easy = Easy::new();
    easy.url(a_str.as_str()).unwrap();
    easy.write_function(write_fn).unwrap();
    easy.perform().unwrap();
}

fn proc_args(argv: Args) -> Vec<String> {
    let mut a: Vec<String> = argv.collect();
    a.remove(0);
    return a;
}

fn format(prefix: &str, args: Vec<String>) -> String {
    format!("{}{}", prefix, args.join(","))
}

fn write_fn(data: &[u8]) -> Result<usize, WriteError> {
    Ok(stdout().write(data).unwrap())
}
