use failure::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum TldExtractError {
    #[fail(display = "no such host: '{}'", _0)]
    NoHostError(String),
}
