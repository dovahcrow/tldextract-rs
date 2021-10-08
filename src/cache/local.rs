use crate::errors::Result;
use log::debug;
use serde_json::{from_reader, to_vec};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn get_tld_cache<'a, O>(cache_path: O) -> Result<HashSet<String>>
where
    O: Into<Option<&'a str>>,
{
    debug!("Trying using local cached TLD data");
    let cache_path = cache_path.into().unwrap_or(".tld_cache");
    let f = File::open(cache_path)?;
    Ok(from_reader(f)?)
}

pub fn set_tld_cache<'a, O>(cache_path: O, tld_cache: &HashSet<String>) -> Result<()>
where
    O: Into<Option<&'a str>>,
{
    let cache_path = cache_path.into().unwrap_or(".tld_cache");
    let data = to_vec(tld_cache).expect("cannot serialize tld cache");
    let mut f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(cache_path)?;
    f.write_all(&data)?;
    Ok(())
}
