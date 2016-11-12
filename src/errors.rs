error_chain!{
    types {}
    links {}
    foreign_links {
         ::serde_json::Error, JsonError;
         ::hyper::Error, HttpError;
         ::url::ParseError, UrlParseError;
         ::std::io::Error, IoError;
    }
    errors {
         NoHostError(t: String) {
             description("no host error")
             display("no such host: '{}'", t)
         }
    }
}