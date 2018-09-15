extern crate curl;

use std::env::args;
use std::io::stdout;
use std::io::Write;

use curl::easy::Easy;

const PREFIX: &str = "https://www.gitignore.io/api/";

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    let a_str: String = format!("{}{}", PREFIX, args.join(","));
}
