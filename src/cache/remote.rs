use std::io::Read;

use regex::Regex;

use reqwest::Client;

use errors::*;
use set::Set;

const PUBLIC_SUFFIX_LIST_URLS: &'static [&'static str] = &[
    "https://publicsuffix.org/list/public_suffix_list.dat",
    "https://raw.githubusercontent.com/publicsuffix/list/master/public_suffix_list.dat",
];

const PUBLIC_SUFFIX_RE: &'static str = r"^(?P<suffix>[.*!]*\w[\S]*)";

pub fn get_tld_cache(private_domain: bool) -> Result<Set<String>> {
    debug!("Trying getting remote TLD data");
    let client = Client::new();
    let reg = Regex::new(PUBLIC_SUFFIX_RE).unwrap();

    for u in PUBLIC_SUFFIX_LIST_URLS {
        let mut resp = client.get(*u).send()?;
        let mut buf = String::new();
        let _ = resp.read_to_string(&mut buf)?;

        let buf = if !private_domain {
            buf.split("// ===BEGIN PRIVATE DOMAINS===").next().unwrap_or("")
        } else {
            &buf[..]
        };

        return Ok(buf
            .lines()
            .filter(|line| !line.starts_with("//"))
            .filter_map(|line| reg.captures(line.trim()).and_then(|cap| cap.name("suffix")))
            .map(|suffix| suffix.as_str().to_string())
            .collect());
    }
    unreachable!("no suffix list urls")
}
