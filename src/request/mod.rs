use crate::request::hash::RequestHash;
use hash::FNV1aHash;
use std::iter::Sum;

mod hash;

pub struct Request<'a> {
    parameters: &'a [String],
}

impl<'a> Request<'a> {
    pub const fn new(parameters: &'a [String]) -> Self {
        Request {
            parameters,
        }
    }
}

impl Request<'_> {
    /// This is a really important function, despite it being out of the way.
    /// The way a request is hashed is
    ///
    /// 1. The hash of each parameters is computed.
    /// 2. The new hashes are summed together.
    ///
    /// Because the hashes are summed, **the order of the parameters doesn't
    /// matter**.
    pub fn hash_parameters<T: FNV1aHash + Sum>(&self) -> RequestHash<T> {
        self.parameters
            .iter()
            .map(|s| T::hash(s.as_bytes().iter()))
            .sum::<T>()
            .into()
    }
}

impl Request<'_> {
    pub fn url_params(&self) -> String { self.parameters.join(",") }
}
