// HNSW index configuration and behavior
// Demonstrates how HNSW parameters affect performance

use piramid::{VectorStorage, VectorEntry, HnswConfig, Metric};
use std::time::Instant;

fn main() {
    println!("=== HNSW Index Configuration ===\n");
    
    // Create dataset
    println!("Generating test dataset (1000 vectors, 128 dims)...");
    let entries: Vec<VectorEntry> = (0..1000).map(|i| {
        let vector: Vec<f32> = (0..128).map(|j| {
            ((i * j) as f32 / 100.0).sin()
        }).collect();
        VectorEntry::new(vector, format!("Vector {}", i))
    }).collect();
    println!("   ✓ Dataset ready\n");
    
    // Test different configurations
    let configs = vec![
        ("Default (m=16, ef=200)", HnswConfig::default()),
        ("High recall (m=32, ef=400)", HnswConfig {
            m: 32,
            m_max: 64,
            ef_construction: 400,
            ml: 1.0 / (32.0_f32).ln(),
            metric: Metric::Cosine,
        }),
        ("Fast insert (m=8, ef=100)", HnswConfig {
            m: 8,
            m_max: 16,
            ef_construction: 100,
            ml: 1.0 / (8.0_f32).ln(),
            metric: Metric::Cosine,
        }),
    ];
    
    let query: Vec<f32> = (0..128).map(|i| (i as f32 / 10.0).sin()).collect();
    
    for (name, config) in configs {
        println!("=== {} ===", name);
        
        let db_name = format!("test_hnsw_{}.db", name.split_whitespace().next().unwrap());
        let _ = std::fs::remove_file(&db_name);
        let _ = std::fs::remove_file(format!("{}.index.db", db_name.trim_end_matches(".db")));
        
        // Build index
        let start = Instant::now();
        let mut storage = VectorStorage::with_hnsw(&db_name, config).unwrap();
        storage.store_batch(entries.clone()).unwrap();
        let build_time = start.elapsed();
        
        println!("   Build time: {:?}", build_time);
        
        // Search performance
        let start = Instant::now();
        let results = storage.search(&query, 10, Metric::Cosine);
        let search_time = start.elapsed();
        
        println!("   Search time: {:?}", search_time);
        println!("   Top result score: {:.4}", results[0].score);
        
        // Index stats
        let stats = storage.index().stats();
        println!("   Nodes: {}", stats.total_nodes);
        println!("   Max layer: {}", stats.max_layer);
        println!("   Avg connections: {:.2}", stats.avg_connections);
        println!();
        
        // Clean up
        std::fs::remove_file(&db_name).unwrap();
        std::fs::remove_file(format!("{}.index.db", db_name.trim_end_matches(".db"))).unwrap();
    }
    
    println!("Key takeaways:");
    println!("  • Higher m = better recall, slower build, more memory");
    println!("  • Higher ef_construction = better index quality, slower build");
    println!("  • Lower m/ef = faster build, lower recall");
    println!("  • Default values (m=16, ef=200) are good balance");
}
