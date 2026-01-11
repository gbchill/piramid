//! Similarity metrics - how we measure "similarity" between vectors
//!
//! This module provides different ways to compare vectors in high-dimensional space.
//! Each metric has different characteristics and use cases:
//! - Cosine: Best for text embeddings (direction matters, not magnitude)
//! - Euclidean: Physical distance in space (magnitude matters)
//! - Dot Product: Fast, good for normalized vectors

// each file is a module. `mod x` says "include x.rs"
mod cosine;
mod euclidean;
mod dot;

// `pub use` re-exports: users can do `metrics::cosine_similarity`
// instead of `metrics::cosine::cosine_similarity`
pub use cosine::cosine_similarity;
pub use euclidean::euclidean_distance;
pub use dot::dot_product;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimilarityMetric {
    #[default]  // used when you call SimilarityMetric::default()
    Cosine,
    Euclidean,
    DotProduct,
}

impl SimilarityMetric {
    pub fn calculate(&self, a: &[f32], b: &[f32]) -> f32 {
        //  `match` must handle ALL variants (exhaustive)
        // This is great - compiler catches if you add a new variant
        match self {
            SimilarityMetric::Cosine => cosine_similarity(a, b),
            SimilarityMetric::Euclidean => {
                let dist = euclidean_distance(a, b);
                1.0 / (1.0 + dist)  // flip: distance -> similarity
            }
            SimilarityMetric::DotProduct => dot_product(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_vectors() {
        let v = vec![1.0, 2.0, 3.0];
        
        // Identical vectors should have max similarity
        assert!((SimilarityMetric::Cosine.calculate(&v, &v) - 1.0).abs() < 1e-6);
        assert!((SimilarityMetric::Euclidean.calculate(&v, &v) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_orthogonal_vectors() {
        let v1 = vec![1.0, 0.0];
        let v2 = vec![0.0, 1.0];
        
        // Orthogonal vectors have 0 cosine similarity
        assert!(SimilarityMetric::Cosine.calculate(&v1, &v2).abs() < 1e-6);
    }
}
