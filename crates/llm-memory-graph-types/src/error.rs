//! Error types for LLM Memory Graph

use thiserror::Error;

/// Main error type for LLM Memory Graph
#[derive(Debug, Error)]
pub enum Error {
    /// Node not found
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    /// Edge not found
    #[error("Edge not found: {0}")]
    EdgeNotFound(String),

    /// Session not found
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    /// Template not found
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    /// Agent not found
    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    /// Node already exists
    #[error("Node already exists: {0}")]
    NodeAlreadyExists(String),

    /// Edge already exists
    #[error("Edge already exists: {0}")]
    EdgeAlreadyExists(String),

    /// Invalid node type
    #[error("Invalid node type: {0}")]
    InvalidNodeType(String),

    /// Invalid edge type
    #[error("Invalid edge type: {0}")]
    InvalidEdgeType(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Storage error (alias)
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Runtime error
    #[error("Runtime error: {0}")]
    RuntimeError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),

    /// Timeout error
    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    /// Plugin error
    #[error("Plugin error: {0}")]
    PluginError(String),

    /// Integration error
    #[error("Integration error: {0}")]
    IntegrationError(String),

    /// gRPC error
    #[error("gRPC error: {0}")]
    GrpcError(String),

    /// Query error
    #[error("Query error: {0}")]
    QueryError(String),

    /// Migration error
    #[error("Migration error: {0}")]
    MigrationError(String),

    /// Graph traversal error
    #[error("Graph traversal error: {0}")]
    TraversalError(String),

    /// Prometheus metrics error
    #[error("Prometheus error: {0}")]
    PrometheusError(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::Other(err.to_string())
    }
}

#[cfg(feature = "storage")]
impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Error::StorageError(err.to_string())
    }
}

#[cfg(feature = "storage")]
impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

#[cfg(feature = "storage")]
impl From<rmp_serde::encode::Error> for Error {
    fn from(err: rmp_serde::encode::Error) -> Self {
        Error::SerializationError(err.to_string())
    }
}

#[cfg(feature = "storage")]
impl From<rmp_serde::decode::Error> for Error {
    fn from(err: rmp_serde::decode::Error) -> Self {
        Error::DeserializationError(err.to_string())
    }
}

#[cfg(feature = "metrics")]
impl From<prometheus::Error> for Error {
    fn from(err: prometheus::Error) -> Self {
        Error::PrometheusError(err.to_string())
    }
}

/// Result type for LLM Memory Graph operations
pub type Result<T> = std::result::Result<T, Error>;
