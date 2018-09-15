use std::fs::{File, DirEntry};
use std::io::Error;
use std::path::Path;
use std::ffi::OsString;

pub const CACHE_NAME: &str = ".gi_cache";

pub fn init(path: &Path) -> Result<File, Error> {
    if !path.is_dir() {
        init(path.parent().unwrap()) // Take parent instead of excepting?
    } else {
        Ok(File::create(path.join(Path::new(CACHE_NAME)).as_path()).unwrap())
    }
}

pub fn check(path: &Path) -> Option<Vec<DirEntry>> {
    if !path.is_dir() {
        check(path.parent().unwrap())
    } else {
        let mut found: Vec<DirEntry> = Vec::new();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == OsString::from(CACHE_NAME) {
                found.push(entry);
            }
        }
        if found.is_empty() {
            None
        } else {
            Some(found)
        }
    }
}
