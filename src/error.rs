use reqwest::header::InvalidHeaderValue;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeckIdError {
    #[error("Invalid base64 encoding: {0}")]
    InvalidBase64(String),

    #[error("Invalid deck ID format: {0}")]
    InvalidFormat(String),

    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),

    #[error("UUID is not version 4: {0}")]
    NotUuidV4(String),
}

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

    #[error("Deck ID error: {0}")]
    DeckId(#[from] DeckIdError),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),

    #[error("Anki output is only supported for file output")]
    AnkiOutputNotSupported,
}

pub type Result<T> = std::result::Result<T, DuoloadError>;
