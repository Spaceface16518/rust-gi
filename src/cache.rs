use std::fs::File;
use std::io::Error;
use std::path::Path;

pub const CACHE_NAME: &str = ".gi_cache";

pub fn init(path: &Path) -> Result<File, Error> {
    if !path.is_dir() {
        init(path.parent().unwrap()) // Take parent instead of excepting?
    } else {
        Ok(File::create(path.join(Path::new(CACHE_NAME)).as_path()).unwrap())
    }
}
