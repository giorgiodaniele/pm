use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    SerializeError(serde_json::Error),
    DeserializeError(serde_json::Error),
    VaultNotFound(String),          // include path
    PermissionDenied(String),       // explain decryption/auth issues
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IOError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        // Default to deserialize error
        AppError::DeserializeError(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IOError(e) =>
                write!(f, "I/O error while accessing a file: {}", e),
            AppError::SerializeError(e) =>
                write!(f, "failed to serialize vault to JSON: {}", e),
            AppError::DeserializeError(e) =>
                write!(f, "failed to deserialize vault from JSON: {}", e),
            AppError::VaultNotFound(path) =>
                write!(f, "vault not found at {}", path),
            AppError::PermissionDenied(msg) =>
                write!(f, "permission denied (likely wrong password or corrupt data): {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
