use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, TldExtractError>;

#[derive(Debug, Error)]
pub enum TldExtractError {
    #[error("no such host: '{0}'")]
    NoHostError(String),

    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[cfg(feature = "remote")]
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
