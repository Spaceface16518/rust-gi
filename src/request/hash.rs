use num_traits::WrappingMul;
use std::{ops::BitXor, str::FromStr};

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
