use self::uri::IntoUri;
use crate::{
    cache::{reader::CacheReader, writer::CacheWriter},
    request::Request,
};
use reqwest::get;
use std::{
    env::args,
    error::Error,
    io,
    io::{stdout, Write},
    mem,
};

mod cache;
mod request;
mod uri;

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    // Get the parameters from command line arguments
    let args = args().skip(1).collect::<Vec<String>>();
    let params = Request::new(&args);
    let param_hash = params.hash_parameters();

    // attempt to grab from cache
    const CACHE_NAME: &str = "gi";
    // TODO: better error handling on this
    let cache_root = cachedir::CacheDirConfig::new(CACHE_NAME)
        .user_cache(true)
        .get_cache_dir()?
        .to_path_buf();
    let mut cache_reader = CacheReader::new(&cache_root);
    if let Ok(ref mut item_reader) =
        cache_reader.try_read_item_strict(param_hash)
    {
        io::copy(item_reader, &mut stdout()).map(mem::drop)?;
    } else {
        let url = "https://gitignore.io/api/".to_owned() + &params.url_params();
        let body = reqwest::get(&url)?.text()?;
        let buf = body.as_bytes();
        stdout().write_all(buf)?;
        let mut cache_writer = CacheWriter::new(param_hash, &cache_root)?;
        cache_writer.write_all(buf)?;
    }
    Ok(())
}
