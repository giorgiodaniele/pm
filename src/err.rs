use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    SerializeError(serde_json::Error),
    DeserializeError(serde_json::Error),
    VaultNotFound,
    PermissionDenied(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IOError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        // Default to deserialize error, but we could branch on context if needed
        AppError::DeserializeError(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IOError(e)        => write!(f, "I/O error: {}", e),
            AppError::SerializeError(e) => write!(f, "serialization error: {}", e),
            AppError::DeserializeError(e) => write!(f, "deserialization error: {}", e),
            AppError::VaultNotFound     => write!(f, "vault not found"),
            AppError::PermissionDenied(msg) => write!(f, "permission denied: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
