use std::{mem::size_of, path::PathBuf};

/// Returns a path based on a hash value, relative to the cache root.
pub fn hash_to_path(hash: u64) -> PathBuf {
    // TODO: make the formatting stuff less clunky
    (0..size_of::<u64>() as u64)
        .into_iter()
        .map(|shift| shift * 8)
        .map(|shift| hash >> shift & 0xFF)
        .map(|chunk| format!("{:x}", chunk))
        .rev()
        .collect()
}

#[cfg(test)]
#[test]
fn test_hash_to_path() {
    let hash: u64 = 0xcbf29ce484222325; // this is just the offset basis of a 64-bit fnv hash
    let path = hash_to_path(hash);
    assert_eq!(path, PathBuf::from("cb/f2/9c/e4/84/22/23/25"))
}
