extern crate curl;

use std::env::args;
use std::io::stdout;
use std::io::Write;

use curl::easy::Easy;

const PREFIX: &str = "https://www.gitignore.io/api/";

fn main() {
    println!("Hello, world!");
}
