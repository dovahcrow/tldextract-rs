use std::fs::{File, OpenOptions};
use std::io::Write;

use serde_json::{to_vec, from_reader};

use ::set::Set;
use ::errors::*;

pub fn get_tld_cache<'a, O>(cache_path: O) -> Result<Set<String>>
    where O: Into<Option<&'a str>>
{
    let cache_path = cache_path.into().unwrap_or(".tld_cache");
    let f = File::open(cache_path)?;
    Ok(from_reader(f)?)
}

pub fn set_tld_cache<'a, O>(cache_path: O, tld_cache: &Set<String>) -> Result<()>
    where O: Into<Option<&'a str>>
{
    let cache_path = cache_path.into().unwrap_or(".tld_cache");
    let data = to_vec(tld_cache).expect("cannot serialize tld cache");
    let mut f = OpenOptions::new().truncate(true).write(true).create(true).open(cache_path)?;
    f.write_all(&data)?;
    Ok(())
}