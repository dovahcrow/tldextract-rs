mod local;
#[cfg(feature = "remote")]
mod remote;
mod snapshot;

use regex::Regex;
use std::collections::HashSet;

use crate::errors::Result;

const PUBLIC_SUFFIX_RE: &str = r"^(?P<suffix>[.*!]*\w[\S]*)";

#[cfg(feature = "remote")]
pub fn get_tld_cache<'a, O, T>(
    cache_path: O,
    local_file_path: T,
    private_domains: bool,
) -> HashSet<String>
where
    O: Into<Option<&'a str>>,
    T: Into<Option<String>>,
{
    local::get_tld_cache(cache_path)
        .or_else(|_| local::get_tld_from_local_file(local_file_path, private_domains))
        .or_else(|_| remote::get_tld_cache(private_domains))
        .or_else(|_| snapshot::get_tld_cache())
        .unwrap()
}

#[cfg(not(feature = "remote"))]
pub fn get_tld_cache<'a, O, T>(
    cache_path: O,
    local_file_path: T,
    private_domains: bool,
) -> HashSet<String>
where
    O: Into<Option<&'a str>>,
    T: Into<Option<String>>,
{
    local::get_tld_cache(cache_path)
        .or_else(|_| local::get_tld_from_local_file(local_file_path, private_domains))
        .or_else(|_| snapshot::get_tld_cache())
        .unwrap()
}

pub fn set_tld_cache<'a, O>(local_path: O, cache: &HashSet<String>) -> Result<()>
where
    O: Into<Option<&'a str>>,
{
    local::set_tld_cache(local_path, cache)
}

fn parse_public_suffix_list(list: &str, private_domain: bool) -> Result<HashSet<String>> {
    let reg = Regex::new(PUBLIC_SUFFIX_RE).unwrap();

    let list = if !private_domain {
        list.split("// ===BEGIN PRIVATE DOMAINS===")
            .next()
            .unwrap_or("")
    } else {
        list
    };

    Ok(list
        .lines()
        .filter(|line| !line.starts_with("//"))
        .filter_map(|line| reg.captures(line.trim()).and_then(|cap| cap.name("suffix")))
        .map(|suffix| suffix.as_str().to_string())
        .collect())
}
