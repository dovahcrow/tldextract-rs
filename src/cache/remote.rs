use crate::errors::Result;
use futures::TryFutureExt;
use log::debug;
use std::collections::HashSet;
use tokio::runtime::Builder;

const PUBLIC_SUFFIX_LIST_URLS: &'static [&'static str] = &[
    "https://publicsuffix.org/list/public_suffix_list.dat",
    "https://raw.githubusercontent.com/publicsuffix/list/master/public_suffix_list.dat",
];

pub fn get_tld_cache(private_domain: bool) -> Result<HashSet<String>> {
    debug!("Trying getting remote TLD data");

    let rt = Builder::new_current_thread().enable_all().build()?;

    for u in PUBLIC_SUFFIX_LIST_URLS {
        let respfut = reqwest::get(*u);
        let contentfut = respfut.and_then(|resp| resp.bytes());
        let content = rt.block_on(contentfut)?;

        let buf = String::from_utf8_lossy(&*content);
        return super::parse_public_suffix_list(&buf, private_domain);
    }
    unreachable!("no suffix list urls")
}
