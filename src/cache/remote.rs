use std::collections::HashSet;

use futures::{Future, Stream};
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use regex::Regex;
use tokio::runtime::current_thread::Runtime;

use errors::Result;

const PUBLIC_SUFFIX_LIST_URLS: &'static [&'static str] = &[
    "https://publicsuffix.org/list/public_suffix_list.dat",
    "https://raw.githubusercontent.com/publicsuffix/list/master/public_suffix_list.dat",
];

const PUBLIC_SUFFIX_RE: &'static str = r"^(?P<suffix>[.*!]*\w[\S]*)";

pub fn get_tld_cache(private_domain: bool) -> Result<HashSet<String>> {
    debug!("Trying getting remote TLD data");
    let https = HttpsConnector::new(1).unwrap();
    let client = Client::builder().build::<_, Body>(https);

    let reg = Regex::new(PUBLIC_SUFFIX_RE).unwrap();

    let mut rt = Runtime::new()?;

    for u in PUBLIC_SUFFIX_LIST_URLS {
        let respfut = client.get(u.parse().unwrap());
        let contentfut = respfut.and_then(|resp| resp.into_body().concat2());
        let content = rt.block_on(contentfut)?;
        let buf = String::from_utf8_lossy(&content);

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
