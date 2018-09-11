# tldExtract [![Build Status](https://travis-ci.org/dovahcrow/tldextract-rs.png?branch=master)](https://travis-ci.org/dovahcrow/tldextract-rs) [![Crates.io](https://img.shields.io/crates/d/tldextract.svg)](https://crates.io/crates/tldextract)
A rust implementation of tldExtract. tldExtract accurately extracts TLD, including gTLD(generic top-level domain) and ccTLD ( country code top-level domain)
from the domain and subdomains of a URL. For example,
it extracts 'google' from 'http://www.google.com'.

Splitting the url with '.' and taking the last 2 elements does not work except for simple examples like .com domains. This does not work for complicated domains like http://forums.bbc.co.uk . The naive splitting method above will give you 'co' as the domain and 'uk' as the TLD,
instead of 'bbc' and 'co.uk' respectively.

While tldExtract knows what all gTLDs and ccTLDs look like
by looking up the currently living ones according to the Public Suffix List.
So, tleExtract knows the subdomain and its domain from its country code.

Thanks to [john-kurkowski](https://github.com/john-kurkowski),
this project is mainly inspired by his [work](https://github.com/john-kurkowski/tldextract) in python

[documentation](https://docs.rs/tldextract/0.5.1/tldextract/)
