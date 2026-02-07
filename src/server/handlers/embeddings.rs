use axum::{extract::{Path, State}, response::Json};
use crate::{Metric, VectorEntry};
use crate::error::{Result, ServerError};
use super::super::{
    state::SharedState,
    types::*,
    sync::LockHelper,
    helpers::{json_to_metadata, metadata_to_json},
};

// Parse similarity metric from string
fn parse_metric(s: Option<String>) -> Metric {
    match s.as_deref() {
        Some("euclidean") => Metric::Euclidean,
        Some("dot") | Some("dot_product") => Metric::DotProduct,
        _ => Metric::Cosine,
    }
}

// POST /api/collections/:collection/embed - embed text and store
pub async fn embed_text(
    State(state): State<SharedState>,
    Path(collection): Path<String>,
    Json(req): Json<EmbedRequest>,
) -> Result<Json<EmbedResponse>> {
    state.get_or_create_collection(&collection)?;

    let embedder = state.embedder.as_ref()
        .ok_or(ServerError::ServiceUnavailable(super::super::helpers::EMBEDDING_NOT_CONFIGURED.to_string()))?;

    let response = embedder.embed(&req.text).await?;

    let metadata = json_to_metadata(req.metadata);
    let entry = VectorEntry::with_metadata(
        response.embedding.clone(),
        req.text,
        metadata,
    );

    let mut collections = state.collections.write_or_err()?;
    let storage = collections.get_mut(&collection)
        .ok_or(ServerError::NotFound(super::super::helpers::COLLECTION_NOT_FOUND.to_string()))?;

    let id = storage.store(entry)?;

    Ok(Json(EmbedResponse {
        id: id.to_string(),
        embedding: response.embedding,
        tokens: response.tokens,
    }))
}

// POST /api/collections/:collection/embed/batch - batch embed texts
pub async fn embed_batch(
    State(state): State<SharedState>,
    Path(collection): Path<String>,
    Json(req): Json<EmbedBatchRequest>,
) -> Result<Json<EmbedBatchResponse>> {
    if req.texts.is_empty() {
        return Err(ServerError::InvalidRequest("No texts provided".to_string()).into());
    }

    state.get_or_create_collection(&collection)?;

    let embedder = state.embedder.as_ref()
        .ok_or(ServerError::ServiceUnavailable(super::super::helpers::EMBEDDING_NOT_CONFIGURED.to_string()))?;

    let responses: Vec<crate::embeddings::EmbeddingResponse> = embedder.embed_batch(&req.texts).await?;

    let mut collections = state.collections.write_or_err()?;
    let storage = collections.get_mut(&collection)
        .ok_or(ServerError::NotFound(super::super::helpers::COLLECTION_NOT_FOUND.to_string()))?;

    let mut ids = Vec::with_capacity(responses.len());
    let mut total_tokens = 0u32;

    for (idx, response) in responses.into_iter().enumerate() {
        let metadata = if idx < req.metadata.len() {
            json_to_metadata(req.metadata[idx].clone())
        } else {
            crate::Metadata::new()
        };

        let entry = VectorEntry::with_metadata(
            response.embedding,
            req.texts[idx].clone(),
            metadata,
        );

        let id = storage.store(entry)?;

        ids.push(id.to_string());
        if let Some(tokens) = response.tokens {
            total_tokens += tokens;
        }
    }

    Ok(Json(EmbedBatchResponse {
        ids,
        total_tokens: if total_tokens > 0 { Some(total_tokens) } else { None },
    }))
}

// POST /api/collections/:collection/search/text - search by text query
pub async fn search_by_text(
    State(state): State<SharedState>,
    Path(collection): Path<String>,
    Json(req): Json<TextSearchRequest>,
) -> Result<Json<SearchResponse>> {
    state.get_or_create_collection(&collection)?;

    let embedder = state.embedder.as_ref()
        .ok_or(ServerError::ServiceUnavailable(super::super::helpers::EMBEDDING_NOT_CONFIGURED.to_string()))?;

    let response = embedder.embed(&req.query).await?;

    let metric = parse_metric(req.metric);

    let collections = state.collections.read_or_err()?;
    let storage = collections.get(&collection)
        .ok_or(ServerError::NotFound(super::super::helpers::COLLECTION_NOT_FOUND.to_string()))?;

    let results: Vec<SearchResultResponse> = storage
        .search(&response.embedding, req.k, metric)
        .into_iter()
        .map(|r| SearchResultResponse {
            id: r.id.to_string(),
            score: r.score,
            text: r.text,
            metadata: metadata_to_json(&r.metadata),
        })
        .collect();

    Ok(Json(SearchResponse { results }))
}
