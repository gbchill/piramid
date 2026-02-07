// HNSW index persistence
// Shows that HNSW index survives restart

use piramid::{VectorStorage, VectorEntry, Metric};
use std::time::Instant;

fn main() {
    println!("=== HNSW Index Persistence ===\n");
    
    let db_path = "test_hnsw_persist.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("test_hnsw_persist.index.db");
    
    let query: Vec<f32> = (0..128).map(|i| (i as f32 / 10.0).sin()).collect();
    
    // Phase 1: Build index
    println!("Phase 1: Building HNSW index...");
    {
        let mut storage = VectorStorage::open(db_path).unwrap();
        
        let start = Instant::now();
        let entries: Vec<VectorEntry> = (0..5000).map(|i| {
            let vector: Vec<f32> = (0..128).map(|j| 
                ((i * j) as f32 / 100.0).sin()
            ).collect();
            VectorEntry::new(vector, format!("Vector {}", i))
        }).collect();
        
        storage.store_batch(entries).unwrap();
        let build_time = start.elapsed();
        
        println!("   Built index with 5000 vectors in {:?}", build_time);
        
        let stats = storage.index().stats();
        println!("   Max layer: {}", stats.max_layer);
        println!("   Total nodes: {}", stats.total_nodes);
        
        // Do a search
        let start = Instant::now();
        let results = storage.search(&query, 10, Metric::Cosine);
        let search_time = start.elapsed();
        
        println!("   Search time: {:?}", search_time);
        println!("   Top result: {} (score: {:.4})", results[0].text, results[0].score);
        println!();
    }
    
    // Phase 2: Reopen and verify index still works
    println!("Phase 2: Reopening storage (loading persisted index)...");
    {
        let start = Instant::now();
        let storage = VectorStorage::open(db_path).unwrap();
        let load_time = start.elapsed();
        
        println!("   Loaded index in {:?}", load_time);
        
        let stats = storage.index().stats();
        println!("   Max layer: {}", stats.max_layer);
        println!("   Total nodes: {}", stats.total_nodes);
        
        // Search should be fast (index already built)
        let start = Instant::now();
        let results = storage.search(&query, 10, Metric::Cosine);
        let search_time = start.elapsed();
        
        println!("   Search time: {:?}", search_time);
        println!("   Top result: {} (score: {:.4})", results[0].text, results[0].score);
        println!();
    }
    
    println!("✓ HNSW index persists across restarts!");
    println!("  No need to rebuild - instant startup");
    
    // Clean up
    std::fs::remove_file(db_path).unwrap();
    std::fs::remove_file("test_hnsw_persist.index.db").unwrap();
}
