use std::fmt::{self, write};

#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    SerializeError,
    DeserializeError,
    VaultNotFound,
    PermissionDenied,
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IOError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_err: serde_json::Error) -> Self {
        AppError::DeserializeError
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IOError(e) => write!(f, "I/O error: {}", e),
            AppError::SerializeError     => write!(f, "serialization error"),
            AppError::DeserializeError   => write!(f, "deserialization error"),
            AppError::VaultNotFound      => write!(f, "vault not found"),
            AppError::PermissionDenied   => write!(f, ""),
        }
    }
}

impl std::error::Error for AppError {}
