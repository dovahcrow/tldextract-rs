extern crate env_logger;
extern crate tldextract;

use std::env;

use tldextract::{TldExtractor, TldOption};

fn option() -> TldOption {
    TldOption::default()
}

fn main() {
    env::set_var("RUST_LOG", "tldextract=debug");
    env_logger::init();
    let ext = TldExtractor::new(option());
    let tld = ext.extract("http://forums.news.cnn.com/").unwrap();
    println!("TLD for 'http://forums.news.cnn.com/' is '{:?}'", tld);
    let tld = ext.extract("forums.news.cnn.com").unwrap();
    println!("TLD for 'forums.news.cnn.com' is '{:?}'", tld);
}
