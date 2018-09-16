use std::fs::{File, DirEntry};
use std::io::Error;
use std::path::Path;
use std::ffi::OsString;

pub const CACHE_NAME: &str = ".gi_cache";
pub const MAX_CACHE_SIZE: usize = 5;

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
}

            let entry = entry.unwrap();
            if entry.file_name() == OsString::from(CACHE_NAME) {
                found.push(entry);
            }
        }
return found;
}
