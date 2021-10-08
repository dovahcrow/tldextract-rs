use crate::errors::Result;
use futures::TryFutureExt;
use log::debug;
use regex::Regex;
use std::collections::HashSet;
use tokio::runtime::Builder;

const PUBLIC_SUFFIX_LIST_URLS: &'static [&'static str] = &[
    "https://publicsuffix.org/list/public_suffix_list.dat",
    "https://raw.githubusercontent.com/publicsuffix/list/master/public_suffix_list.dat",
];

const PUBLIC_SUFFIX_RE: &'static str = r"^(?P<suffix>[.*!]*\w[\S]*)";

pub fn get_tld_cache(private_domain: bool) -> Result<HashSet<String>> {
    debug!("Trying getting remote TLD data");

    let reg = Regex::new(PUBLIC_SUFFIX_RE).unwrap();

    let rt = Builder::new_current_thread().enable_all().build()?;

    for u in PUBLIC_SUFFIX_LIST_URLS {
        let respfut = reqwest::get(*u);
        let contentfut = respfut.and_then(|resp| resp.bytes());
        let content = rt.block_on(contentfut)?;

        let buf = String::from_utf8_lossy(&*content);

        let buf = if !private_domain {
            buf.split("// ===BEGIN PRIVATE DOMAINS===")
                .next()
                .unwrap_or("")
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
