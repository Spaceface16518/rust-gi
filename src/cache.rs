use std::fs::{File, DirEntry};
use std::io::Error;
use std::path::Path;
use std::ffi::OsString;

pub const CACHE_NAME: &str = ".gi_cache";
pub const MAX_CACHE_SIZE: usize = 5;

pub fn init(path: &Path) -> Result<File, Error> {
    if !path.is_dir() {
        Ok(i_init(path.parent().unwrap())) // Take parent instead of excepting?
    } else {
        Ok(i_init(path))
    }
}

fn i_init(path: &Path) -> File {
    File::create(path.join(Path::new(CACHE_NAME)).as_path()).unwrap()
}

pub fn check(path: &Path) -> Option<Vec<DirEntry>> {
    if !path.is_dir() {
        Some(i_check(path.parent().unwrap()))
    } else {
        let f = i_check(path);
        if f.is_empty() {
            None
        } else {
            Some(f)
        }
    }
}

fn i_check(path: &Path) -> Vec<DirEntry> {
let mut found: Vec<DirEntry> = Vec::new();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == OsString::from(CACHE_NAME) {
                found.push(entry);
            }
        }
return found;
}
