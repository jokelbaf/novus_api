use reqwest::header::InvalidHeaderValue;
use reqwest::StatusCode;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum NovusError {
    #[error("Request failed with status code {0}")]
    StatusError(StatusCode),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Header value error: {0}")]
    HeaderValueError(#[from] InvalidHeaderValue),

    #[error("Url parsing error: {0}")]
    UrlError(String),

    #[error("Can't refresh token without `refresh_token` set!")]
    RefreshTokenMissingError,

    #[error("The endpoint requires `token` to be set")]
    TokenMissingError,
}
