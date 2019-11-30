use std::io::{Error, ErrorKind};

pub fn cache_miss() -> Error { Error::from(ErrorKind::NotFound) }
