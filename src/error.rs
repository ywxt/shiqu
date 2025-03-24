use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    HttpError(Box<dyn std::error::Error + Send + Sync>),
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error {
        Error::HttpError(Box::new(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::HttpError(Box::new(err))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
