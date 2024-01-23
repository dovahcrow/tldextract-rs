extern crate tldextract;
use tldextract::TldOption;
use tldextract::TldResult;

#[test]
fn baidu() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("https://www.baidu.com").unwrap(),
        TldResult::new("www", "baidu", "com")
    );
}
#[test]
fn shuiguan() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.水管.com").unwrap(),
        TldResult::new("www", "水管", "com")
    );
}
#[test]
fn google() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("https://google.fr").unwrap(),
        TldResult::new(None, "google", "fr")
    );
}
#[test]
fn facebook() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("https://m.facebook.com").unwrap(),
        TldResult::new("m", "facebook", "com")
    );
}
#[test]
fn uestc() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.uestc.edu.cn").unwrap(),
        TldResult::new("www", "uestc", "edu.cn")
    );
}
#[test]
fn bbc_uk() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://forums.bbc.co.uk/").unwrap(),
        TldResult::new("forums", "bbc", "co.uk")
    );
}
#[test]
fn cnn() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://forums.news.cnn.com/").unwrap(),
        TldResult::new("forums.news", "cnn", "com")
    );
}
#[test]
fn worldbank() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.worldbank.org.kg/").unwrap(),
        TldResult::new("www", "worldbank", "org.kg")
    );
}
#[test]
fn localhost_ip() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://127.0.0.1:8080/deployed/").unwrap(),
        TldResult::new(None, "127.0.0.1", None)
    );
}

#[test]
fn american() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.google.com").unwrap(),
        TldResult::new("www", "google", "com")
    );
}

#[test]
fn british() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.theregister.co.uk").unwrap(),
        TldResult::new("www", "theregister", "co.uk")
    );
}

#[test]
fn no_subdomain() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://gmail.com").unwrap(),
        TldResult::new(None, "gmail", "com")
    );
}

#[test]
fn nested_subdomain() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://media.forums.theregister.co.uk")
            .unwrap(),
        TldResult::new("media.forums", "theregister", "co.uk")
    );
}

#[test]
fn odd_but_possible() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.www.com").unwrap(),
        TldResult::new("www", "www", "com")
    );
    assert_eq!(
        ext.extract("http://www.com").unwrap(),
        TldResult::new(None, "www", "com")
    );
}

#[test]
fn local_host() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://internalunlikelyhostname/").unwrap(),
        TldResult::new(None, "internalunlikelyhostname", None)
    );
    assert_eq!(
        ext.extract("http://internalunlikelyhostname.bizarre")
            .unwrap(),
        TldResult::new("internalunlikelyhostname", "bizarre", None)
    );
}

#[test]
fn qualified_local_host() {
    let ext = TldOption::default().build();

    assert_eq!(
        ext.extract("http://internalunlikelyhostname.info/")
            .unwrap(),
        TldResult::new(None, "internalunlikelyhostname", "info")
    );
    assert_eq!(
        ext.extract("http://internalunlikelyhostname.information/")
            .unwrap(),
        TldResult::new("internalunlikelyhostname", "information", None)
    );
}

#[test]
fn ip() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://216.22.0.192/").unwrap(),
        TldResult::new(None, "216.22.0.192", None)
    );
    assert_eq!(
        ext.extract("http://216.22.project.coop/").unwrap(),
        TldResult::new("216.22", "project", "coop")
    );
}

#[test]
fn punycode() {
    let ext = TldOption::default().build();

    assert_eq!(
        ext.extract("http://xn--h1alffa9f.xn--p1ai").unwrap(),
        TldResult::new(None, "россия", "рф")
    );
}

#[test]
fn punycode2() {
    let ext = TldOption::default().build();

    assert_eq!(
        ext.extract("xn--tub-1m9d15sfkkhsifsbqygyujjrw602gk4li5q.google.com")
            .unwrap(),
        TldResult::new("亲您好异常订单退款链接tub", "google", "com")
    );
}

#[test]
fn invalid_punycode() {
    let ext = TldOption::default().build();

    // Entries that might generate UnicodeError exception
    // This subdomain generates UnicodeError 'IDNA does not round-trip'
    ext.extract("http://xn--tub-1m9d15sfkkhsifsbqygyujjrw602gk4li5qqk98aca0w.google.com")
        .unwrap_err();

    // This subdomain generates UnicodeError 'incomplete punicode string'
    ext.extract("http://xn--tub-1m9d15sfkkhsifsbqygyujjrw60.google.com")
        .unwrap_err();
}

#[test]
fn invalid_puny_with_puny() {
    let ext = TldOption::default().build();
    ext.extract("http://xn--zckzap6140b352by.blog.so-net.xn--wcvs22d.hk")
        .unwrap_err();
}

#[test]
fn puny_with_non_puny() {
    let ext = TldOption::default().build();
    ext.extract("http://xn--zckzap6140b352by.blog.so-net.教育.hk")
        .unwrap_err();
}

#[test]
fn idna_2008() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://xn--gieen46ers-73a.de").unwrap(),
        TldResult::new(None, "gießen46ers", "de")
    );
}

#[test]
fn scheme() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("https://mail.google.com/mail").unwrap(),
        TldResult::new("mail", "google", "com")
    );
    assert_eq!(
        ext.extract("ssh://mail.google.com/mail").unwrap(),
        TldResult::new("mail", "google", "com")
    );
}

#[test]
fn port() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("git+ssh://www.github.com:8443/").unwrap(),
        TldResult::new("www", "github", "com")
    );
}

#[test]
fn username() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("ftp://johndoe:5cr1p7k1dd13@1337.warez.com:2501")
            .unwrap(),
        TldResult::new("1337", "warez", "com")
    );
}

#[test]
fn query_fragment() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://google.com?q=cats").unwrap(),
        TldResult::new(None, "google", "com")
    );
    assert_eq!(
        ext.extract("http://google.com#Welcome").unwrap(),
        TldResult::new(None, "google", "com")
    );
    assert_eq!(
        ext.extract("http://google.com/#Welcome").unwrap(),
        TldResult::new(None, "google", "com")
    );
    assert_eq!(
        ext.extract("http://google.com/s#Welcome").unwrap(),
        TldResult::new(None, "google", "com")
    );
    assert_eq!(
        ext.extract("http://google.com/s?q=cats#Welcome").unwrap(),
        TldResult::new(None, "google", "com")
    );
}

#[test]
fn regex_order() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.parliament.uk").unwrap(),
        TldResult::new("www", "parliament", "uk")
    );
    assert_eq!(
        ext.extract("http://www.parliament.co.uk").unwrap(),
        TldResult::new("www", "parliament", "co.uk")
    );
}

#[test]
fn unhandled_by_iana() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.cgs.act.edu.au/").unwrap(),
        TldResult::new("www", "cgs", "act.edu.au")
    );
    assert_eq!(
        ext.extract("http://www.google.com.au/").unwrap(),
        TldResult::new("www", "google", "com.au")
    );
}

#[test]
fn ld_is_a_website_too() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.metp.net.cn").unwrap(),
        TldResult::new("www", "metp", "net.cn")
    );
    // assert_eq!(ext.extract("http://www.net.cn").unwrap(),
    //            TldResult::new("www", "net", "cn"));
    // This is unhandled by the
    // PSL. Or is it?
}

#[test]
fn dns_root_label() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://www.example.com./").unwrap(),
        TldResult::new("www", "example", "com")
    );
}

#[test]
fn private_domains() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("http://waiterrant.blogspot.com").unwrap(),
        TldResult::new("waiterrant", "blogspot", "com")
    );
}

#[test]
fn whole_url_is_a_suffix() {
    let ext = TldOption::default().build();
    assert_eq!(
        ext.extract("https://shingo.aomori.jp").unwrap(),
        TldResult::new(None, None, "shingo.aomori.jp")
    );
}

#[test]
fn public_suffix_list_custom_local_file() {
    let file_path: std::path::PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "public_suffix_list-custom_local_file.dat",
    ]
    .iter()
    .collect();
    let ext = TldOption::default()
        .local_public_suffix_file(file_path.display().to_string().as_str())
        .private_domains(true)
        .build();
    assert_eq!(
        ext.extract("www.dovahcrow.tldextract").unwrap(),
        TldResult::new("www", "dovahcrow", "tldextract")
    );
}
