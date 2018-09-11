mod local;
mod remote;
mod snapshot;

use std::collections::HashSet;

use errors::Result;

pub fn get_tld_cache<'a, O>(local_path: O, private_domains: bool) -> HashSet<String>
where
    O: Into<Option<&'a str>>,
{
    local::get_tld_cache(local_path)
        .or_else(|_| remote::get_tld_cache(private_domains))
        .or_else(|_| snapshot::get_tld_cache())
        .unwrap()
}

pub fn set_tld_cache<'a, O>(local_path: O, cache: &HashSet<String>) -> Result<()>
where
    O: Into<Option<&'a str>>,
{
    local::set_tld_cache(local_path, cache)
}
