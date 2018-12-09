extern crate hyper;
extern crate log;

use hyper::Uri;
use log::trace;
use hyper::http::HttpTryFrom;

pub fn uri_from(args: Vec<String>) -> Uri {
    trace!("Creating URI from provided arguments");
    Uri::try_from(format_uri(format_args(args))).expect("Could not generate URI")
}

fn format_args(args: Vec<String>) -> String {
    args.join(",")
}

fn format_uri(formatted: String) -> String {
    format!("https://www.gitignore.io/api/{}", formatted)
}

#[cfg(test)]
mod uri_tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_format_args() {
        let args = vec!["arg1", "arg2", "arg3"].into_iter().map(String::from).collect::<Vec<String>>();
        let formatted = format_args(args);
        assert_eq!(formatted, "arg1,arg2,arg3".to_string());
    }

    #[test]
    fn test_uri_from() {
        let uri = uri_from(vec!["arg1", "arg2", "arg3"].into_iter().map(String::from).collect::<Vec<String>>());
        assert_eq!(uri, Uri::from_str("https://www.gitignore.io/api/arg1,arg2,arg3").unwrap());
    }

    #[test]
    fn test_format_uri() {
        let formatted = format_uri("postfix".to_string());
        assert_eq!(formatted, "https://www.gitignore.io/api/postfix".to_string());
    }
}