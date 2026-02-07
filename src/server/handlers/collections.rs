use axum::{extract::{Path, State}, response::Json};
use crate::error::Result;
use super::super::{
    state::SharedState,
    types::*,
    sync::LockHelper,
};

// GET /api/collections - list all loaded collections
pub async fn list_collections(State(state): State<SharedState>) -> Result<Json<CollectionsResponse>> {
    let collections = state.collections.read_or_err()?;
    
    let infos: Vec<CollectionInfo> = collections
        .iter()
        .map(|(name, storage)| CollectionInfo {
            name: name.clone(),
            count: storage.count(),
        })
        .collect();
    
    Ok(Json(CollectionsResponse { collections: infos }))
}

// POST /api/collections - create a new collection
pub async fn create_collection(
    State(state): State<SharedState>,
    Json(req): Json<CreateCollectionRequest>,
) -> Result<Json<CollectionInfo>> {
    state.get_or_create_collection(&req.name)?;
    
    let collections = state.collections.read_or_err()?;
    let count = collections.get(&req.name).map(|s| s.count()).unwrap_or(0);
    
    Ok(Json(CollectionInfo { name: req.name, count }))
}

// GET /api/collections/:name - get info about one collection
pub async fn get_collection(
    State(state): State<SharedState>,
    Path(name): Path<String>,
) -> Result<Json<CollectionInfo>> {
    state.get_or_create_collection(&name)?;
    
    let collections = state.collections.read_or_err()?;
    let count = collections.get(&name).map(|s| s.count()).unwrap_or(0);
    
    Ok(Json(CollectionInfo { name, count }))
}

// DELETE /api/collections/:name - remove a collection
pub async fn delete_collection(
    State(state): State<SharedState>,
    Path(name): Path<String>,
) -> Result<Json<DeleteResponse>> {
    let mut collections = state.collections.write_or_err()?;
    let existed = collections.remove(&name).is_some();
    
    if existed {
        let path = format!("{}/{}.db", state.data_dir, name);
        std::fs::remove_file(&path).ok();
    }
    
    Ok(Json(DeleteResponse { deleted: existed }))
}

// GET /api/collections/:name/count - just the count
pub async fn collection_count(
    State(state): State<SharedState>,
    Path(collection): Path<String>,
) -> Result<Json<CountResponse>> {
    state.get_or_create_collection(&collection)?;
    
    let collections = state.collections.read_or_err()?;
    let count = collections.get(&collection).map(|s| s.count()).unwrap_or(0);
    
    Ok(Json(CountResponse { count }))
}
