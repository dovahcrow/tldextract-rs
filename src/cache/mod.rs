mod local;
mod remote;
mod snapshot;

use ::set::Set;
use errors::*;

pub fn get_tld_cache<'a, O>(local_path: O, private_domains: bool) -> Set<String>
    where O: Into<Option<&'a str>>
{
    local::get_tld_cache(local_path)
        .or_else(|_| remote::get_tld_cache(private_domains))
        .or_else(|_| snapshot::get_tld_cache())
        .unwrap()
}

pub fn set_tld_cache<'a, O>(local_path: O, cache: &Set<String>) -> Result<()>
    where O: Into<Option<&'a str>>
{
    local::set_tld_cache(local_path, cache)
}