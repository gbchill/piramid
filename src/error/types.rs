use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, PiramidError>;

#[derive(Error, Debug)]
pub enum PiramidError {
    // Storage errors
    #[error("Storage error: {0}")]
    Storage(#[from] super::storage::StorageError),

    // Index errors
    #[error("Index error: {0}")]
    Index(#[from] super::index::IndexError),

    // Server/API errors
    #[error("Server error: {0}")]
    Server(#[from] super::server::ServerError),

    // Embedding errors
    #[error("Embedding error: {0}")]
    Embedding(#[from] super::embedding::EmbeddingError),

    // IO errors
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    // Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    // Generic errors
    #[error("{0}")]
    Other(String),
}

impl PiramidError {
    pub fn other<S: Into<String>>(msg: S) -> Self {
        Self::Other(msg.into())
    }

    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Storage(e) => e.is_recoverable(),
            Self::Index(e) => e.is_recoverable(),
            Self::Server(e) => e.is_recoverable(),
            Self::Embedding(e) => e.is_recoverable(),
            Self::Io(_) => false,
            Self::Serialization(_) => false,
            Self::Other(_) => false,
        }
    }
}
