// Collection persistence operations
use crate::error::Result;
use crate::storage::persistence::{save_index as save_idx, save_vector_index as save_vec_idx, save_metadata as save_meta};
use super::storage::Collection;

pub fn save_index(storage: &Collection) -> Result<()> {
    save_idx(&storage.path, &storage.index)
}

pub fn save_vector_index(storage: &Collection) -> Result<()> {
    save_vec_idx(&storage.path, storage.vector_index.as_ref())
}

pub fn save_metadata(storage: &Collection) -> Result<()> {
    save_meta(&storage.path, &storage.metadata)
}

pub fn checkpoint(storage: &mut Collection) -> Result<()> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    storage.persistence.wal.checkpoint(timestamp)?;
    storage.persistence.record_checkpoint(timestamp);
    save_index(storage)?;
    save_vector_index(storage)?;
    save_metadata(storage)?;
    storage.persistence.wal.truncate()?;
    
    Ok(())
}

pub fn flush(storage: &mut Collection) -> Result<()> {
    storage.persistence.wal.flush()?;
    Ok(())
}
