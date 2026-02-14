use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use parking_lot::RwLock;
use dashmap::DashMap;
use tokio::runtime::Handle;

use crate::Collection;
use crate::storage::collection::CollectionOpenOptions;
use crate::embeddings::Embedder;
use crate::metrics::LatencyTracker;
use crate::error::{Result, ServerError};
use crate::config::AppConfig;
use std::sync::atomic::AtomicU64;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RebuildState {
    Running,
    Completed,
    Failed,
}

#[derive(Clone)]
pub struct RebuildJobStatus {
    pub status: RebuildState, // Current status of the rebuild job (Running, Completed, Failed)
    pub started_at: u64, // Timestamp when the rebuild job started (in seconds since UNIX epoch)
    pub finished_at: Option<u64>, // Optional timestamp when the rebuild job finished (in seconds since UNIX epoch)
    pub error: Option<String>, // Optional error message if the rebuild job failed
    pub elapsed_ms: Option<u128>, // Optional elapsed time for the rebuild job in milliseconds
}

// Shared application state
// Each collection is an independent Collection with its own file.
// DashMap allows concurrent access to different collections without blocking.
// Holds config + optional embedder so handlers can access without reloading.
pub struct AppState {
    pub collections: DashMap<String, Arc<RwLock<Collection>>>, // Map of collection name to its storage handle. Wrapped in Arc<RwLock> for shared mutable access across threads.
    pub data_dir: String, // Base directory for collection files, e.g. "./data"
    pub embedder: Option<Arc<dyn Embedder>>, // Optional embedder, if configured. Wrapped in Arc for shared ownership.
    pub shutting_down: Arc<AtomicBool>, // Flag to indicate server is shutting down, used to reject new requests gracefully
    pub latency_tracker: Arc<DashMap<String, LatencyTracker>>,  // Per-collection latency tracking
    pub app_config: Arc<RwLock<AppConfig>>, // Global config accessible to handlers, protected by RwLock for dynamic updates
    pub slow_query_ms: u128, // Threshold for logging slow queries in ms
    pub rebuild_jobs: Arc<DashMap<String, RebuildJobStatus>>, // Track index rebuild jobs by collection name
    pub config_last_reload: Arc<AtomicU64>, // Timestamp of last config reload for cache invalidation
}

impl AppState {
    pub fn new(data_dir: &str, app_config: AppConfig, slow_query_ms: u128) -> Self {
        std::fs::create_dir_all(data_dir).ok();
        
        Self {
            collections: DashMap::new(),
            data_dir: data_dir.to_string(),
            embedder: None,
            shutting_down: Arc::new(AtomicBool::new(false)),
            latency_tracker: Arc::new(DashMap::new()),
            app_config: Arc::new(RwLock::new(app_config)),
            slow_query_ms,
            rebuild_jobs: Arc::new(DashMap::new()),
            // Initialize to current time; updated on each config reload
            config_last_reload: Arc::new(AtomicU64::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )),
        }
    }

    pub fn with_embedder(data_dir: &str, app_config: AppConfig, slow_query_ms: u128, embedder: Arc<dyn Embedder>) -> Self {
        std::fs::create_dir_all(data_dir).ok();
        
        Self {
            collections: DashMap::new(),
            data_dir: data_dir.to_string(),
            embedder: Some(embedder),
            shutting_down: Arc::new(AtomicBool::new(false)),
            latency_tracker: Arc::new(DashMap::new()),
            app_config: Arc::new(RwLock::new(app_config)),
            slow_query_ms,
            rebuild_jobs: Arc::new(DashMap::new()),
            config_last_reload: Arc::new(AtomicU64::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )),
        }
    }

    // Lazily load or create a collection
    pub fn get_or_create_collection(&self, name: &str) -> Result<()> {

        if self.shutting_down.load(Ordering::Relaxed) {
            return Err(ServerError::ServiceUnavailable("Server is shutting down".into()).into());
        }

        if !self.collections.contains_key(name) {
            let path = format!("{}/{}.db", self.data_dir, name);
            let cfg = { self.app_config.read().clone() };
            let storage = Collection::open_with_options(
                &path,
                CollectionOpenOptions::from(cfg.to_collection_config()),
            )?;
            let handle = Arc::new(RwLock::new(storage));
            self.collections.insert(name.to_string(), handle.clone());
            
            // Create latency tracker for this collection
            self.latency_tracker.insert(name.to_string(), LatencyTracker::new());

            // Warm caches in the background to avoid first-request latency.
            let warm_handle = handle.clone();
            if let Ok(rt) = Handle::try_current() {
                rt.spawn_blocking(move || {
                    let guard = warm_handle.read();
                    guard.warm_page_cache();
                });
            }
        }
        
        Ok(())
    }

    pub fn checkpoint_all(&self) -> Result<()> {
        for mut entry in self.collections.iter_mut() {
            let storage = entry.value_mut();
            let mut storage_guard = storage.write();
            storage_guard.checkpoint()?;
            storage_guard.flush()?;
        }
        Ok(())
    }

    pub fn reload_config(&self) -> Result<AppConfig> {
        let new_cfg = crate::config::loader::load_app_config();
        {
            let mut guard = self.app_config.write();
            *guard = new_cfg.clone();
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.config_last_reload.store(now, Ordering::Relaxed);
        Ok(new_cfg)
    }

    pub fn current_config(&self) -> AppConfig {
        self.app_config.read().clone()
    }
    
    pub fn initiate_shutdown(&self) {
        self.shutting_down.store(true, Ordering::Relaxed);
    }
}

pub type SharedState = Arc<AppState>;
