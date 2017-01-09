error_chain!{
    types {}
    links {}
    foreign_links {
        JsonError(::serde_json::Error);
        HttpError(::hyper::Error);
        UrlParseError(::url::ParseError);
        IoError(::std::io::Error);
    }
    errors {
        NoHostError(t: String) {
            description("no host error")
            display("no such host: '{}'", t)
        }
        EmptyDomainError {
            description("domain should not be empty")
            display("domain should not be empty")
        }
    }
}
