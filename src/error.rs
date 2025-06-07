use thiserror::Error;

#[derive(Debug, Error)]
pub enum DuoloadError {
    #[error("Invalid or expired cookie")]
    InvalidCookie,
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Anki package error: {0}")]
    AnkiPackage(String),
    
    #[error("Data parsing error: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("Operation error: {0}")]
    Operation(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, DuoloadError>;
