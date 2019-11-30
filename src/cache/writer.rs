use std::{fs, fs::File, io, io::BufWriter, path::Path};

use crate::cache::mapping::hash_to_path;
use std::fs::OpenOptions;

pub struct CacheWriter {
    writer: BufWriter<File>,
}

impl CacheWriter {
    pub fn new(param_hash: u64, cache_root: &Path) -> io::Result<Self> {
        let path = cache_root.to_path_buf().join(hash_to_path(param_hash));
        fs::create_dir_all(&path);
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path.join(".gitignore"))?;
        Ok(CacheWriter {
            writer: BufWriter::new(file),
        })
    }

    fn ensure_parent_dir(path: &Path) {
        // TODO: actually handle error case?
        let _ = fs::create_dir_all(path);
    }
}

// Delegate Write implementations
impl io::Write for CacheWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> { self.writer.flush() }
}
