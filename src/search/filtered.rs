// Filtered search - vector similarity search with metadata filtering

use std::collections::HashMap;
use uuid::Uuid;

use crate::storage::VectorStorage;
use crate::metrics::Metric;
use crate::query::Filter;
use crate::search::SearchResult;

// Perform filtered vector similarity search
// 
// Combines vector similarity search with metadata filtering.
// Note: Filtering is applied post-search, so we search for more candidates
// (k * 10) to ensure we get k results after filtering.
// 
// # Arguments
// * `storage` - The vector storage to search in
// * `query` - Query vector
// * `k` - Number of results to return (after filtering)
// * `metric` - Distance/similarity metric to use
// * `filter` - Metadata filter to apply
// 
// # Returns
// Vector of k most similar results matching the filter, sorted by score
pub fn filtered_search(
    storage: &VectorStorage,
    query: &[f32],
    k: usize,
    metric: Metric,
    filter: &Filter,
) -> Vec<SearchResult> {
    // Create vector map for index
    let vector_map: HashMap<Uuid, Vec<f32>> = storage
        .get_vectors()
        .iter()
        .map(|(id, entry)| (*id, entry.vector.clone()))
        .collect();

    // Search for more candidates to compensate for filtered-out results
    let search_k = k * 10;
    let ef = (search_k * 2).max(50);
    
    let result_ids = storage.index().search(query, search_k, ef, &vector_map);

    // Convert IDs to SearchResults and apply filter
    let mut results: Vec<SearchResult> = result_ids
        .into_iter()
        .filter_map(|id| {
            storage.get(&id).and_then(|entry| {
                // Apply filter
                if !filter.matches(&entry.metadata) {
                    return None;
                }
                
                let score = metric.calculate(query, &entry.vector);
                Some(SearchResult::new(
                    entry.id,
                    score,
                    entry.text.clone(),
                    entry.vector.clone(),
                    entry.metadata.clone(),
                ))
            })
        })
        .collect();

    // Sort by score
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(k);
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::VectorEntry;
    use crate::metadata;

    #[test]
    fn test_filtered_search() {
        let _ = std::fs::remove_file("test_filtered_search.db");
        let mut storage = VectorStorage::open("test_filtered_search.db").unwrap();

        // Insert vectors with metadata
        let e1 = VectorEntry::with_metadata(
            vec![1.0, 0.0, 0.0],
            "rust doc".to_string(),
            metadata::metadata([("lang", "rust".into())])
        );
        let e2 = VectorEntry::with_metadata(
            vec![0.9, 0.1, 0.0],
            "python doc".to_string(),
            metadata::metadata([("lang", "python".into())])
        );
        
        storage.store(e1).unwrap();
        storage.store(e2).unwrap();

        // Search with filter
        let filter = Filter::new().eq("lang", "rust");
        let query = vec![1.0, 0.0, 0.0];
        let results = filtered_search(&storage, &query, 5, Metric::Cosine, &filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].text, "rust doc");

        std::fs::remove_file("test_filtered_search.db").unwrap();
    }
}
