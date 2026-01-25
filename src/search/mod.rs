// Search module - all search operations
//
// This module provides different types of search operations on vector storage:
// - vector_search: Basic k-NN similarity search
// - filtered_search: k-NN with metadata filtering
// 
// Future search types:
// - range_search: Find all vectors within a distance threshold
// - batch_search: Search multiple queries at once
// - hybrid_search: Combine vector + keyword search
// - recommendation_search: Find similar to these, not like those

mod result;
mod vector;
mod filtered;
mod utils;

pub use result::SearchResult;
pub use vector::vector_search;
pub use filtered::filtered_search;

// Re-export for convenience
pub use crate::metrics::Metric;
pub use crate::query::Filter;
