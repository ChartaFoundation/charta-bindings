/// Error types for the Charta Rust SDK

use thiserror::Error;

/// Result type for Charta SDK operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the Charta SDK
#[derive(Error, Debug)]
pub enum Error {
    /// VM execution error
    #[error("VM error: {0}")]
    VM(#[from] charta_vm::error::VMError),

    /// IR loading error
    #[error("Failed to load IR: {0}")]
    IRLoad(String),

    /// File I/O error
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    /// JSON parsing error
    #[error("JSON error: {0}")]
    JSON(#[from] serde_json::Error),

    /// Signal/coil not found
    #[error("Signal/coil not found: {0}")]
    NotFound(String),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
