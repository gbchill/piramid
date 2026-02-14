use crate::metrics::Metric;
use crate::search::Hit;
use crate::storage::Collection;

pub fn search(
    collection: &Collection,
    query: &[f32],
    k: usize,
    metric: Metric,
    mut params: crate::search::SearchParams,
) -> Vec<Hit> {
    if matches!(params.mode, crate::config::ExecutionMode::Auto) {
        params.mode = collection.config().execution;
    }
    if params.filter_overfetch_override.is_none() {
        params.filter_overfetch_override = Some(collection.config.search.filter_overfetch);
    }
    crate::search::search_collection(collection, query, k, metric, params)
}

pub fn search_batch(
    collection: &Collection,
    queries: &[Vec<f32>],
    k: usize,
    metric: Metric,
) -> Vec<Vec<Hit>> {
    let params = crate::search::SearchParams {
        mode: collection.config().execution,
        filter: None,
        filter_overfetch_override: None,
        search_config_override: None,
    };
    crate::search::search_batch_collection(collection, queries, k, metric, params)
}
