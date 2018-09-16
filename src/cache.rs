use std::collections::hash_map::DefaultHasher;
use std::ffi::{OsStr, OsString};
use std::fs::{create_dir, read_dir, remove_dir, write, DirEntry, ReadDir};
use std::hash::Hasher;
use std::io::Result;
use std::path::{Path, PathBuf};

pub const CACHE_NAME: &str = ".gi_cache";
pub const MAX_CACHE_SIZE: usize = 5;
pub const CACHE_ENTRY_SUFFIX: &str = ".gitignore";

pub fn init(path: &Path) -> Result<()> {
    if !path.is_dir() {
        Ok(i_init(path.parent().unwrap())) // Take parent instead of excepting?
    } else {
        Ok(i_init(path))
    }
}

fn i_init(path: &Path) {
    create_dir(path).unwrap()
}

pub fn preexists(path: &Path) -> bool {
    i_check(path)
}

fn i_check(path: &Path) -> bool {
    for entry in path.read_dir().unwrap() {
        let entry: DirEntry = entry.unwrap();
        if entry.metadata().unwrap().is_dir() {
            if entry.file_name() == OsString::from(CACHE_NAME) {
                return true;
            }
        }
    }
    return false;
}

pub struct Cache {
    inner: OsStr,
}

impl Cache {
    pub fn new<S>(path: &S) -> &Cache
    where
        S: AsRef<OsStr> + ?Sized,
    {
        // unsafe { &*(path.as_ref() as *const OsStr as *const Cache) } // Borrowed from `Path`, don't know what it does

        Cache {}
    }

    pub fn index(&self) -> Result<&[u64]> {
        let mut sto: Vec<u64> = Vec::new();
        for entry in self.get_cache().unwrap() {
            let mut hasher = DefaultHasher::new();
            entry
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .bytes()
                .for_each(|byte| {
                    hasher.write_u8(byte);
                });
            sto.push(hasher.finish());
        }
        Ok(&sto[..]) // Returning `Result` necessary here?
    }

    pub fn get_cache(&self) -> Result<ReadDir> {
        read_dir(Path::new(&self.inner))
    }

    pub fn size(&self) -> Result<usize> {
        // Inaccurate because of cache contamination
        Ok(self.get_cache().unwrap().count())
    }

    pub fn safe_size(&self) -> Result<usize> {
        Ok(self
            .get_cache()
            .unwrap()
            .filter(|entry| entry.unwrap().path().extension().unwrap() == CACHE_ENTRY_SUFFIX)
            .count())
    }

    pub fn san_size(&self) -> Result<usize> {
        Ok(self.sanitize().unwrap().get_cache().unwrap().count())
    }

    pub fn sanitize(&self) -> Result<&Cache> {
        for entry in self.get_cache().unwrap() {
            let entry = entry.unwrap();
            if entry.path().extension().unwrap() != CACHE_ENTRY_SUFFIX {
                self.raw_remove(entry.file_name().to_str().unwrap().to_string());
            }
        }
        Ok(self)
    }

    pub fn push(&self, name: &String, contents: &[u8]) -> Result<&Cache> {
        let path = PathBuf::from(self.as_path()).push(name);
        self.raw_add(name, contents).unwrap();
        Ok(self)
    }

    pub fn remove(&self, name: String) -> Result<&Cache> {
        let mut s = name.clone();
        s.push_str(CACHE_ENTRY_SUFFIX);
        self.raw_remove(s).unwrap();
        Ok(self)
    }

    fn raw_remove(&self, name: String) -> Result<()> {
        remove_dir(self.as_path())
    }

    pub fn as_path(&self) -> &Path {
        Path::new(&self.inner)
    }

    fn raw_add(&self, name: &String, file_contents: &[u8]) -> Result<()> {
        let mut path = PathBuf::from(self.as_path());
        path.push(name);
        write(path, file_contents)
    }
}
