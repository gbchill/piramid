// HNSW types and configuration

use serde::{Serialize, Deserialize};
use crate::metrics::Metric;

// HNSW (Hierarchical Navigable Small World) index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnswConfig{
    pub m: usize,  // max number of connections per node
    pub m_max: usize,  // max connections for layer 0 (typically 2*M)
    pub ef_construction: usize,  // size of the dynamic list for the construction phase
    pub ml: f32,  // layer multiplier: 1/ln(M)
    pub metric: Metric,  // similarity metric
}

impl Default for HnswConfig {
    fn default() -> Self {
        let m = 16;
        HnswConfig {
            m,
            m_max: m * 2,
            ef_construction: 200,
            ml: 1.0 / (m as f32).ln(),
            metric: Metric::Cosine,
        }
    }
}

#[derive(Debug)]
pub struct HnswStats {
    pub total_nodes: usize,
    pub max_layer: isize,
    pub layer_sizes: Vec<usize>,
    pub avg_connections: f32,
}
