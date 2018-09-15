extern crate curl;

use std::env::args;
use std::io::{stdout, Write};
use curl::easy::Easy;

const PREFIX: &str = "https://www.gitignore.io/api/";

fn main() {
    let mut args: Vec<String> = args().collect();
    args.remove(0);
    let a_str: String = format!("{}{}", PREFIX, args.join(","));
    let mut easy = Easy::new();
    easy.url(a_str.as_str()).unwrap();
    easy.write_function(|data| Ok(stdout().write(data).unwrap()))
        .unwrap();
    easy.perform().unwrap();
}
