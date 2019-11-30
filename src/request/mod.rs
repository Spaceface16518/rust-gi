use hash::FNV1aHash;
use std::iter::Sum;

mod hash;

#[derive(Debug, Default, PartialEq, Clone)]
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
    pub fn hash_parameters(&self) -> u64 {
        self.parameters
            .iter()
            .map(|s| u32::hash(s.as_bytes().iter()))
            .map(|i| i as u64)
            .sum::<u64>()
    }
}

impl Request<'_> {
    pub fn url_params(&self) -> String { self.parameters.join(",") }
}
