// Helper utilities for search operations

use crate::search::Hit;

// Sort search results by score (descending) and truncate to k
pub(crate) fn sort_and_truncate(results: &mut Vec<Hit>, k: usize) {
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score) // Sort by score (descending)
            .unwrap_or(std::cmp::Ordering::Equal) // Handle NaN cases by treating them as equal
    }); // Sort by score (descending)
    results.truncate(k);
}
