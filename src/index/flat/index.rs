// Flat (brute force) index implementation
// O(N) search - compares query against all vectors
// Best for: small collections (<10k vectors), zero build time, 100% recall

use uuid::Uuid;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::config::FlatConfig;
use crate::index::traits::{VectorIndex, IndexStats, IndexDetails, IndexType};

// Flat index - simple brute force search
// Stores nothing except config (vectors are in main storage)
#[derive(Clone, Serialize, Deserialize)]
pub struct FlatIndex {
    config: FlatConfig, // Configuration for the flat index, including distance metric and execution mode
    vector_ids: Vec<Uuid>,  // Track which vectors we've seen
}

// Implement methods for FlatIndex
impl FlatIndex {
    pub fn new(config: FlatConfig) -> Self {
        FlatIndex {
            config,
            vector_ids: Vec::new(),
        }
    }
}
// Implement the VectorIndex trait for FlatIndex. This includes methods for inserting vectors, searching for nearest neighbors, removing vectors, and getting index statistics. The insert method simply tracks the IDs of the vectors without building any indexing structure. The search method performs a brute force search by calculating the distance from the query to every vector in the collection and returning the top k results based on the configured metric. The remove method removes a vector ID from the tracking list, and the stats method returns information about the index such as total vectors and memory usage.
impl VectorIndex for FlatIndex {
    fn insert(&mut self, id: Uuid, _vector: &[f32], _vectors: &HashMap<Uuid, Vec<f32>>) {
        // Just track the ID - no indexing structure needed
        if !self.vector_ids.contains(&id) {
            self.vector_ids.push(id);
        }
    }
    
    // Search for nearest neighbors to the query vector. This method calculates the distance from the query to every vector in the collection using the configured metric, sorts the results by similarity score, and returns the top k IDs. The quality parameter is ignored for flat index since it's always exhaustive. The filter and metadata parameters are also ignored in this simple implementation, but they could be used in a more advanced version to filter results based on metadata or other criteria.
    fn search(
        &self,
        query: &[f32],
        k: usize,
        vectors: &HashMap<Uuid, Vec<f32>>,
        _quality: crate::config::SearchConfig,
        _filter: Option<&crate::search::query::Filter>,
        _metadatas: &HashMap<Uuid, crate::metadata::Metadata>,
    ) -> Vec<Uuid> {
        // Flat index is always exhaustive - quality parameter is ignored
        // Brute force: calculate distance to every vector
        let mut distances: Vec<(Uuid, f32)> = self.vector_ids
            .iter()
            .filter_map(|id| {
                vectors.get(id).map(|vec| {
                    let score = self.config.metric.calculate(query, vec, self.config.mode);
                    (*id, score)
                })
            })
            .collect();
        
        // Sort by score (descending for similarity)
        distances.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Return top k IDs
        distances.iter()
            .take(k)
            .map(|(id, _)| *id)
            .collect()
    }
    
    fn remove(&mut self, id: &Uuid) {
        self.vector_ids.retain(|vid| vid != id);
    }
    
    fn stats(&self) -> IndexStats {
        IndexStats {
            index_type: IndexType::Flat,
            total_vectors: self.vector_ids.len(),
            memory_usage_bytes: self.vector_ids.len() * std::mem::size_of::<Uuid>(),
            details: IndexDetails::Flat,
        }
    }
    
    fn index_type(&self) -> IndexType {
        IndexType::Flat
    }
}
