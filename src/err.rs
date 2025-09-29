use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error while accessing a file")]
    IOError(#[from] std::io::Error),

    #[error("failed to serialize vault to JSON")]
    SerializeError(serde_json::Error),

    #[error("failed to deserialize vault from JSON")]
    DeserializeError(serde_json::Error),

    #[error("vault not found")]
    VaultNotFound(String),

    #[error("permission denied (likely wrong password or corrupt data)")]
    PermissionDenied(String),
}
