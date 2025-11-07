//! Error types for the client

use thiserror::Error;

/// Client error types
#[derive(Debug, Error)]
pub enum ClientError {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// gRPC transport error
    #[error("gRPC transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    /// gRPC status error
    #[error("gRPC status error: {0}")]
    Status(#[from] tonic::Status),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Invalid argument
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Already exists
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for client operations
pub type Result<T> = std::result::Result<T, ClientError>;

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::Serialization(err.to_string())
    }
}
