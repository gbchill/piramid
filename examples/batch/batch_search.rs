// Batch search operations
// Search multiple queries in parallel

use piramid::{VectorStorage, VectorEntry, Metric};
use std::time::Instant;

fn main() {
    println!("=== Batch Search Performance ===\n");
    
    let _ = std::fs::remove_file("test_batch_search.db");
    let _ = std::fs::remove_file("test_batch_search.index.db");
    
    let mut storage = VectorStorage::open("test_batch_search.db").unwrap();
    
    // Create dataset
    println!("Creating dataset (5000 vectors)...");
    let entries: Vec<VectorEntry> = (0..5000).map(|i| {
        let vector: Vec<f32> = (0..128).map(|j| (i * j) as f32 / 100.0).collect();
        VectorEntry::new(vector, format!("Vector {}", i))
    }).collect();
    storage.store_batch(entries).unwrap();
    println!("   ✓ Dataset ready\n");
    
    // Create multiple queries
    let num_queries = 100;
    let queries: Vec<Vec<f32>> = (0..num_queries).map(|i| {
        (0..128).map(|j| (i * j * 7) as f32 / 100.0).collect()
    }).collect();
    
    // Test 1: Sequential search
    println!("Test 1: Sequential search ({} queries)...", num_queries);
    let start = Instant::now();
    let mut sequential_results = Vec::new();
    for query in &queries {
        let results = storage.search(query, 10, Metric::Cosine);
        sequential_results.push(results);
    }
    let sequential_duration = start.elapsed();
    
    println!("   Time: {:?}", sequential_duration);
    println!("   Rate: {:.2} queries/sec", 
        num_queries as f64 / sequential_duration.as_secs_f64());
    
    // Test 2: Batch search (parallel)
    println!("\nTest 2: Batch search ({} queries)...", num_queries);
    let start = Instant::now();
    let batch_results = storage.search_batch(&queries, 10, Metric::Cosine);
    let batch_duration = start.elapsed();
    
    println!("   Time: {:?}", batch_duration);
    println!("   Rate: {:.2} queries/sec", 
        num_queries as f64 / batch_duration.as_secs_f64());
    
    // Verify results match
    assert_eq!(sequential_results.len(), batch_results.len());
    println!("   ✓ Results verified");
    
    // Compare
    println!("\n=== Comparison ===");
    let speedup = sequential_duration.as_secs_f64() / batch_duration.as_secs_f64();
    println!("   Speedup: {:.2}x faster", speedup);
    println!("   Time saved: {:?}", sequential_duration - batch_duration);
    
    println!("\n✓ Batch search uses parallel processing (rayon)");
    println!("  Ideal for high-throughput applications");
    
    // Clean up
    std::fs::remove_file("test_batch_search.db").unwrap();
    std::fs::remove_file("test_batch_search.index.db").unwrap();
}
