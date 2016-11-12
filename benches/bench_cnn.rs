#![feature(test)]

extern crate test;
extern crate tldextract;

use test::Bencher;
use tldextract::TldExtractor;
use tldextract::TldOption;

fn option() -> TldOption {
    TldOption::default()
}

#[bench]
fn bench_cnn(b: &mut Bencher) {
    let ext = TldExtractor::new(option());
    b.iter(|| ext.extract("http://forums.news.cnn.com/").unwrap())
}