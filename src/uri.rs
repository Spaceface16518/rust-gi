use hyper::{http::HttpTryFrom, Uri};
use log::trace;
use std::ops::Deref;

const PREFIX: &str = "https://www.gitignore.io/api/";

pub fn uri_from(args: Vec<String>) -> Uri {
    trace!("Creating URI from provided arguments");
    args.into_uri()
}

#[inline]
fn format_args<T: Deref<Target = [String]>>(args: T) -> String {
    args.deref().join(",")
}

#[inline]
fn format_uri(formatted: String) -> String {
    format!("{}{}", PREFIX, formatted)
}

pub trait IntoUri {
    fn into_uri(self) -> Uri;
}

impl<T: Deref<Target = [String]>> IntoUri for T {
    #[inline]
    fn into_uri(self) -> Uri {
        Uri::try_from(format_uri(format_args(self)))
            .expect("Could not generate URI")
    }
}

#[cfg(test)]
mod uri_tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn test_format_args() {
        let args = vec!["arg1", "arg2", "arg3"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        let formatted = format_args(args);
        assert_eq!(formatted, "arg1,arg2,arg3".to_string());
    }

    #[test]
    fn test_uri_from() {
        let uri = uri_from(
            vec!["arg1", "arg2", "arg3"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
        );
        assert_eq!(
            uri,
            Uri::from_str("https://www.gitignore.io/api/arg1,arg2,arg3")
                .unwrap()
        );
    }

    #[test]
    fn test_format_uri() {
        let formatted = format_uri("postfix".to_string());
        assert_eq!(
            formatted,
            "https://www.gitignore.io/api/postfix".to_string()
        );
    }
}
