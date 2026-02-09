// Collection search operations
use std::collections::HashMap;
use uuid::Uuid;

use crate::metrics::Metric;
use crate::search::Hit;
use super::storage::Collection;

pub fn search(storage: &Collection, query: &[f32], k: usize, metric: Metric) -> Vec<Hit> {
    search_with_mode(storage, query, k, metric, storage.config.execution)
}

pub fn search_with_mode(
    storage: &Collection, 
    query: &[f32], 
    k: usize, 
    metric: Metric, 
    mode: crate::config::ExecutionMode
) -> Vec<Hit> {
    let mut vectors: HashMap<Uuid, Vec<f32>> = HashMap::new();
    for (id, _) in &storage.index {
        if let Some(entry) = super::operations::get(storage, id) {
            vectors.insert(*id, entry.get_vector());
        }
    }
    
    let neighbor_ids = storage.vector_index.search(
        query, 
        k, 
        &vectors, 
        crate::config::SearchConfig::default()
    );
    
    let mut results = Vec::new();
    for id in neighbor_ids {
        if let Some(entry) = super::operations::get(storage, &id) {
            let vec = entry.get_vector();
            let score = metric.calculate(query, &vec, mode);
            results.push(Hit {
                id,
                score,
                text: entry.text,
                vector: vec,
                metadata: entry.metadata.clone(),
            });
        }
    }
    results
}

pub fn search_batch(
    storage: &Collection, 
    queries: &[Vec<f32>], 
    k: usize, 
    metric: Metric
) -> Vec<Vec<Hit>> {
    if storage.config.parallelism.parallel_search {
        use rayon::prelude::*;
        queries
            .par_iter()
            .map(|query| search(storage, query, k, metric))
            .collect()
    } else {
        queries
            .iter()
            .map(|query| search(storage, query, k, metric))
            .collect()
    }
}

pub fn search_with_filter(
    storage: &Collection,
    query: &[f32],
    k: usize,
    metric: Metric,
    filter: Option<&crate::search::query::Filter>,
) -> Vec<Hit> {
    let mode = storage.config.execution;
    match filter {
        Some(f) => crate::search::filtered_search(storage, query, k, metric, f, mode),
        None => search(storage, query, k, metric),
    }
}
