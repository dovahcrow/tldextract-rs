extern crate tldextract;

use tldextract::TldExtractor;
use tldextract::TldResult;
use tldextract::TldOption;

fn option() -> TldOption {
    TldOption::default()
}

#[test]
fn baidu() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("https://www.baidu.com").unwrap(),
               TldResult::new("www", "baidu", "com"));
}
#[test]
fn shuiguan() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("http://www.水管.com").unwrap(),
               TldResult::new("www", "水管", "com"));
}
#[test]
fn google() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("https://google.fr").unwrap(),
               TldResult::new(None, "google", "fr"));
}
#[test]
fn facebook() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("https://m.facebook.com").unwrap(),
               TldResult::new("m", "facebook", "com"));
}
#[test]
fn uestc() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("http://www.uestc.edu.cn").unwrap(),
               TldResult::new("www", "uestc", "edu.cn"));
}
#[test]
fn bbc_uk() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("http://forums.bbc.co.uk/").unwrap(),
               TldResult::new("forums", "bbc", "co.uk"));
}
#[test]
fn cnn() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("http://forums.news.cnn.com/").unwrap(),
               TldResult::new("forums.news", "cnn", "com"));
}
#[test]
fn worldbank() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("http://www.worldbank.org.kg/").unwrap(),
               TldResult::new("www", "worldbank", "org.kg"));
}
#[test]
fn localhost_ip() {
    let ext = TldExtractor::new(option());
    assert_eq!(ext.extract("http://127.0.0.1:8080/deployed/").unwrap(),
               TldResult::new(None, "127.0.0.1", None));
}