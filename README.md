# tldextract [![Travis](https://img.shields.io/travis/doomsplayer/tldextract-rs.svg)](https://crates.io/crates/tldextract) [![Crates.io](https://img.shields.io/crates/d/tldextract.svg)](https://crates.io/crates/tldextract)

tldextract accurately separates the gTLD or ccTLD (generic or country code top-level domain)
from the registered domain and subdomains of a URL. For example,
say you want just the 'google' part of 'http://www.google.com'.

Everybody gets this wrong. Splitting on the '.' and taking the last 2 elements goes a long way
only if you're thinking of simple e.g. .com domains. Think parsing http://forums.bbc.co.uk
for example: the naive splitting method above will give you 'co' as the domain and 'uk' as the TLD,
instead of 'bbc' and 'co.uk' respectively.

tldextract on the other hand knows what all gTLDs and ccTLDs look like
by looking up the currently living ones according to the Public Suffix List.
So, given a URL, it knows its subdomain from its domain, and its domain from its country code.

Thanks to [john-kurkowski](https://github.com/john-kurkowski),
this project is mainly inspired (Ok, stolen) by his [work](https://github.com/john-kurkowski/tldextract) in python

[documentation](http://wooya.me/tldextract-rs/tldextract/index.html)