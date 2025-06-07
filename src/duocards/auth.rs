use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid or expired cookie")]
    InvalidCookie,
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
}
