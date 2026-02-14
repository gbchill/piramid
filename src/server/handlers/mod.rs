// Handler modules organized by functionality
pub mod health;
pub mod collections;
pub mod vectors;
pub mod embeddings;
pub mod config;

// Re-export all handlers
pub use health::*;
pub use collections::*;
pub use vectors::*;
pub use embeddings::*;
pub use config::*;
