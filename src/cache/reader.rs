use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

use crate::cache::{error::cache_miss, mapping::hash_to_path};
use std::io::{Error, Read};

pub struct CacheReader<'a> {
    cache_root: &'a Path,
}

impl<'a> CacheReader<'a> {
    /// Does not check if the cache root is valid
    pub const fn new(cache_root: &'a Path) -> Self {
        CacheReader {
            cache_root,
        }
    }

    pub fn try_read_item_lazy(
        &self,
        hash: u64,
    ) -> Result<CacheItemReader<impl AsRef<Path>>, io::Error> {
        self.try_get_item_path(hash).map(|path| CacheItemReader::lazy(path))
    }

    pub fn try_read_item_strict(
        &self,
        hash: u64,
    ) -> Result<CacheItemReader<impl AsRef<Path>>, io::Error> {
        self.try_get_item_path(hash)
            .and_then(|path| CacheItemReader::strict(path))
    }

    fn try_get_item_path(&self, hash: u64) -> Result<PathBuf, io::Error> {
        if self.cache_root.exists() {
            let item_relative_path = hash_to_path(hash);
            let item_path =
                self.cache_root.to_path_buf().join(item_relative_path);

            Ok(item_path)
        } else {
            Err(cache_miss())
        }
    }
}

pub struct CacheItemReader<P>(LazyCacheItemHandle<P>);

impl<P: AsRef<Path>> CacheItemReader<P> {
    pub fn lazy(path: P) -> Self {
        CacheItemReader(LazyCacheItemHandle::Lazy(path))
    }

    pub fn strict(path: P) -> io::Result<Self> {
        Ok(CacheItemReader(LazyCacheItemHandle::Open(File::open(path)?)))
    }
}

impl<P: AsRef<Path>> Read for CacheItemReader<P> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        match &mut self.0 {
            LazyCacheItemHandle::Open(file) => file.read(buf),
            LazyCacheItemHandle::Lazy(ref p) => {
                let mut file = File::open(p)?;
                self.0 = LazyCacheItemHandle::Open(file);
                self.0.file_mut().unwrap().read(buf)
            },
        }
    }
}

enum LazyCacheItemHandle<P> {
    Open(File),
    Lazy(P),
}

impl<P: AsRef<Path>> LazyCacheItemHandle<P> {
    /// Only returns `None` if `self` is set to the `Lazy` variant. If `self` is
    /// guaranteed to be the `Open` variant, an `unwrap` will not panic.
    fn file_mut(&mut self) -> Option<&mut File> {
        match self {
            Self::Lazy(_) => None,
            Self::Open(file) => Some(file),
        }
    }
}
