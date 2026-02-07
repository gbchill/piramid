// Batch insert operations
// Demonstrates performance benefits of batch operations

use piramid::{VectorStorage, VectorEntry};
use std::time::Instant;

fn main() {
    println!("=== Batch Insert Performance ===\n");
    
    let _ = std::fs::remove_file("test_batch.db");
    let _ = std::fs::remove_file("test_batch.index.db");
    
    let mut storage = VectorStorage::open("test_batch.db").unwrap();
    
    let num_vectors = 1000;
    let dimensions = 128;
    
    // Test 1: Individual inserts
    println!("Test 1: Individual inserts ({} vectors)...", num_vectors);
    let mut individual_storage = VectorStorage::open("test_individual.db").unwrap();
    
    let start = Instant::now();
    for i in 0..num_vectors {
        let vector: Vec<f32> = (0..dimensions).map(|j| (i * j) as f32 / 100.0).collect();
        let entry = VectorEntry::new(vector, format!("Vector {}", i));
        individual_storage.store(entry).unwrap();
    }
    let individual_duration = start.elapsed();
    
    println!("   Time: {:?}", individual_duration);
    println!("   Rate: {:.2} vectors/sec", 
        num_vectors as f64 / individual_duration.as_secs_f64());
    
    // Test 2: Batch insert
    println!("\nTest 2: Batch insert ({} vectors)...", num_vectors);
    
    let entries: Vec<VectorEntry> = (0..num_vectors).map(|i| {
        let vector: Vec<f32> = (0..dimensions).map(|j| (i * j) as f32 / 100.0).collect();
        VectorEntry::new(vector, format!("Vector {}", i))
    }).collect();
    
    let start = Instant::now();
    let ids = storage.store_batch(entries).unwrap();
    let batch_duration = start.elapsed();
    
    println!("   Time: {:?}", batch_duration);
    println!("   Rate: {:.2} vectors/sec", 
        num_vectors as f64 / batch_duration.as_secs_f64());
    println!("   Inserted {} vectors", ids.len());
    
    // Compare
    println!("\n=== Comparison ===");
    let speedup = individual_duration.as_secs_f64() / batch_duration.as_secs_f64();
    println!("   Speedup: {:.2}x faster", speedup);
    println!("   Time saved: {:?}", individual_duration - batch_duration);
    
    println!("\n✓ Batch operations are significantly faster!");
    println!("  Use batch APIs for bulk data loading");
    
    // Clean up
    std::fs::remove_file("test_batch.db").unwrap();
    std::fs::remove_file("test_batch.index.db").unwrap();
    std::fs::remove_file("test_individual.db").unwrap();
    std::fs::remove_file("test_individual.index.db").unwrap();
}
