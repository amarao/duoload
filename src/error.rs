use std::io;
use thiserror::Error;
use reqwest::header::InvalidHeaderValue;

#[derive(Error, Debug)]
pub enum DuoloadError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error: {0}")]
    Api(String),

    #[error("Invalid header value: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),

    #[error("Authentication error: {0}")]
    Auth(#[from] crate::duocards::auth::AuthError),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, DuoloadError>;
