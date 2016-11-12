#![doc(html_root_url = "http://wooya.me/tldextract-rs/tldextract/index.html")]
#![deny(missing_docs)]

//! # tldextract
//!
//! tldextract accurately separates the gTLD or ccTLD (generic or country code top-level domain)
//! from the registered domain and subdomains of a URL. For example,
//! say you want just the 'google' part of 'http://www.google.com'.
//!
//! Everybody gets this wrong. Splitting on the '.' and taking the last 2 elements goes a long way
//! only if you're thinking of simple e.g. .com domains. Think parsing http://forums.bbc.co.uk
//! for example: the naive splitting method above will give you 'co' as the domain and 'uk' as the TLD,
//! instead of 'bbc' and 'co.uk' respectively.
//!
//! tldextract on the other hand knows what all gTLDs and ccTLDs look like
//! by looking up the currently living ones according to the Public Suffix List.
//! So, given a URL, it knows its subdomain from its domain, and its domain from its country code.
//!
//! Thanks to [john-kurkowski](https://github.com/john-kurkowski),
//! this project is mainly inspired (Ok, stolen) by his [work](https://github.com/john-kurkowski/tldextract) in python

extern crate url;
#[macro_use]
extern crate error_chain;
extern crate idna;
extern crate serde_json;
// extern crate futures;
extern crate hyper;
// extern crate tokio_core;
extern crate regex;

#[allow(missing_docs)]
pub mod errors;
mod cache;
mod set;

use url::{Url, Host};

use idna::punycode;

use set::Set;
pub use errors::*;

/// The option for `TldExtractor`.
///
/// # Examples
///
/// ```
/// use tldextract::{TldExtractor, TldOption, TldResult};
///
/// let option = TldOption {
///    cache_path: Some(".tld_cache".to_string()),
///    private_domains: false,
///    update_local: false,
/// };
///
/// let ext = TldExtractor::new(option);
///
/// assert_eq!(ext.extract("https://m.facebook.com").unwrap(), TldResult::new("m", "facebook", "com"));
/// ```
#[derive(Default)]
pub struct TldOption {
    /// The path to file for storing tld cache
    pub cache_path: Option<String>,
    /// Whether to include private domains
    pub private_domains: bool,
    /// Should tldextract update local cache file if
    /// the cache is fetched from remote or from snapshot
    pub update_local: bool,
}

/// The tld extractor, see TldOption for more docs.
pub struct TldExtractor {
    tld_cache: Set<String>,
}

impl TldExtractor {
    /// Constructs a new `TldExtractor`.
    ///
    /// see TldOption for more docs.
    pub fn new(option: TldOption) -> TldExtractor {
        let cache_path = option.cache_path.as_ref().map(|s| &s[..]);
        let tld_cache = cache::get_tld_cache(cache_path.clone(), option.private_domains);
        if option.update_local {
            let _ = cache::set_tld_cache(cache_path, &tld_cache);
        }
        TldExtractor { tld_cache: tld_cache }
    }

    /// Extract (subdomain, domain, domain suffix) tuple from a given url
    pub fn extract(&self, url: &str) -> Result<TldResult> {
        let u = Url::parse(url)?;
        let host = u.host().ok_or(ErrorKind::NoHostError(url.into()))?;
        match host {
            Host::Domain(host) => self.extract_triple(host),
            Host::Ipv4(ip) => Ok(TldResult { domain: ip.to_string(), ..Default::default() }),
            Host::Ipv6(ip) => Ok(TldResult { domain: ip.to_string(), ..Default::default() }),
        }
    }

    fn extract_triple(&self, host: &str) -> Result<TldResult> {
        let segs: Vec<_> = host.split('.').collect();

        let mut suffix = None;
        let mut subdomain = None;
        let mut domain = None;
        for i in 0..segs.len() {
            let piece = segs[i..].join(".");
            let exception_piece = "!".to_string() + &piece;
            let wildcard_piece = "*.".to_string() + &segs[i + 1..].join(".");

            if let Some(_) = self.tld_cache.get(&exception_piece) {
                continue;
            }
            if let Some(_) = self.tld_cache.get(&piece).or(self.tld_cache.get(&wildcard_piece)) {
                suffix = Some(piece);

                domain = if segs[i - 1].starts_with("xn--") {
                    Some(punycode::decode_to_string(segs[i - 1].trim_left_matches("xn--"))
                        .ok_or("cannot decode punycode")?)
                } else {
                    Some(segs[i - 1].into())
                };

                subdomain = if segs[0..i - 1].len() == 0 {
                    None
                } else {
                    Some(segs[0..i - 1].join("."))
                };
                break;
            }
        }

        Ok(TldResult {
            suffix: suffix,
            subdomain: subdomain,
            domain: domain.expect("domain should not be empty"),
        })
    }
}

/// The Tld Result Type
///
/// E.g. "https://www.google.com" will be represent into
///
/// ```
/// use tldextract::TldResult;
///
/// TldResult { domain: "google".to_string(), subdomain: Some("www".to_string()), suffix: Some("com".to_string())};
/// ```
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TldResult {
    /// The "google" part of "www.google.com"
    pub domain: String,
    /// The "www" part of "www.google.com"
    pub subdomain: Option<String>,
    /// The "com" part of "www.google.com"
    pub suffix: Option<String>,
}

impl TldResult {
    /// Create a new TldResult
    ///
    /// # Examples
    ///
    /// ```
    /// use tldextract::TldResult;
    /// assert_eq!(TldResult::new("www", "google", "com"),
    ///   TldResult {
    ///     domain: "google".to_string(),
    ///     subdomain: Some("www".to_string()),
    ///     suffix: Some("com".to_string())
    ///   });
    /// ```
    pub fn new<'a, O, L>(subdomain: O, domain: &str, suffix: L) -> TldResult
        where O: Into<Option<&'a str>>,
              L: Into<Option<&'a str>>
    {
        TldResult {
            domain: domain.into(),
            subdomain: subdomain.into().map(|s| s.into()),
            suffix: suffix.into().map(|s| s.into()),
        }
    }
}