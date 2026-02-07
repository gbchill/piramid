use axum::{http::StatusCode, response::Json};
use super::super::{state::SharedState, types::HealthResponse};
use axum::extract::State;

// GET /api/health - simple liveness check
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}

// GET /api/health/embeddings - check if embedding service is available
pub async fn health_embeddings(State(state): State<SharedState>) -> StatusCode {
    match state.embedder {
        Some(_) => StatusCode::OK,
        None => StatusCode::SERVICE_UNAVAILABLE,
    }
}
