use num_traits::WrappingMul;
use std::{
    ops::{BitOr, BitXor},
    str::FromStr,
};

/// An opaque type representing the hash of a `Request`'s parameters.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct RequestHash<T = u64>(T);

impl<T> RequestHash<T> {
    const fn new(n: T) -> Self { RequestHash(n) }
}

impl<T> From<T> for RequestHash<T> {
    fn from(n: T) -> Self { RequestHash::new(n) }
}

impl<T: FromStr> FromStr for RequestHash<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(RequestHash::new)
    }
}

pub trait FNV1aHash:
    From<u8> + WrappingMul + BitXor<Self, Output = Self>
{
    const OFFSET_BASIS: Self;
    const FNV_PRIME: Self;

    fn hash<'a>(params: impl Iterator<Item = &'a u8> + 'a) -> Self {
        params.fold(Self::OFFSET_BASIS, |acc, byte| {
            // fnv1a hashing algorithm
            (acc ^ Self::from(*byte)).wrapping_mul(&Self::FNV_PRIME)
        })
    }
}

impl FNV1aHash for u32 {
    const FNV_PRIME: Self = 16777619;
    const OFFSET_BASIS: Self = 0x811c9dc5;
}

impl FNV1aHash for u64 {
    const FNV_PRIME: Self = 1099511628211;
    const OFFSET_BASIS: Self = 0xcbf29ce484222325;
}
