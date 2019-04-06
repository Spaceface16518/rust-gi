use self::uri::IntoUri;
use reqwest::get;
use std::env::args;

mod uri;

fn main() {
    pretty_env_logger::init();

    // Get the URI from command line arguments
    let uri = args().skip(1).collect::<Vec<String>>().into_uri();

    let body = get(&uri).unwrap().text().unwrap();

    println!("{}", body);
}
