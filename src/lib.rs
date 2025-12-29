//stores text embeddings

pub mod config;
pub mod storage;
pub mod error;

pub use config::Config;
pub use error::{PiramidError, Result};
pub use storage::{VectorEntry, VectorStorage};
