// Core Collection storage structure
use memmap2::MmapMut;
use std::collections::HashMap;
use std::fs::File;
use uuid::Uuid;

use crate::error::Result;
use crate::index::VectorIndex;
use crate::storage::wal::Wal;
use crate::storage::persistence::EntryPointer;
use crate::storage::metadata::CollectionMetadata;

// Vector storage engine with memory-mapped files and pluggable indexing
pub struct Collection {
    pub(super) data_file: File,
    pub(super) mmap: Option<MmapMut>,
    pub(super) index: HashMap<Uuid, EntryPointer>,
    pub(super) vector_index: Box<dyn VectorIndex>,
    pub(super) config: crate::config::CollectionConfig,
    pub(super) metadata: CollectionMetadata,
    pub(super) path: String,
    pub(super) wal: Wal,
    pub(super) operation_count: usize,
}

impl Collection {
    pub(super) fn init_rayon_pool(config: &crate::config::ParallelismConfig) {
        let num_threads = config.num_threads();
        if num_threads > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
                .ok();
        }
    }

    pub(super) fn track_operation(&mut self) -> Result<()> {
        self.operation_count += 1;
        if self.config.wal.enabled && 
           self.operation_count >= self.config.wal.checkpoint_frequency {
            super::persistence::checkpoint(self)?;
            self.operation_count = 0;
        }
        Ok(())
    }
    
    pub fn metadata(&self) -> &CollectionMetadata {
        &self.metadata
    }

    pub fn count(&self) -> usize {
        self.index.len()
    }
    
    pub fn memory_usage_bytes(&self) -> usize {
        let mmap_size = self.mmap.as_ref().map(|m| m.len()).unwrap_or(0);
        let index_size = self.index.len() * std::mem::size_of::<(Uuid, EntryPointer)>();
        let vector_index_stats = self.vector_index.stats();
        
        mmap_size + index_size + vector_index_stats.memory_usage_bytes
    }

    pub fn vector_index(&self) -> &dyn VectorIndex {
        self.vector_index.as_ref()
    }

    pub fn get_all(&self) -> Vec<crate::storage::document::Document> {
        let mut all_entries = Vec::new();
        for (id, _) in &self.index {
            if let Some(entry) = super::operations::get(self, id) {
                all_entries.push(entry);
            }
        }
        all_entries
    }
}
