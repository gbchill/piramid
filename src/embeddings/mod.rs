// This module provides a unified interface for different embedding providers,
// allowing users to generate embeddings from text without needing to handle
// the embeddings externally.

mod types;
pub mod providers;
pub mod cache;

pub use types::{Embedder, EmbeddingConfig, EmbeddingResponse, EmbeddingResult};
pub use providers::{EmbeddingProvider, create_embedder};
pub use cache::{CachedEmbedder, CacheStats};
pub use crate::error::embedding::EmbeddingError;

