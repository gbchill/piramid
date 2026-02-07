// Memory-mapped file efficiency demonstration
// Shows how mmap handles large datasets efficiently

use piramid::{VectorStorage, VectorEntry};
use std::time::Instant;

fn main() {
    println!("=== Memory-Mapped Storage Efficiency ===\n");
    
    let db_path = "test_mmap.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("test_mmap.index.db");
    
    // Create storage with many vectors
    println!("Creating storage with 10,000 vectors...");
    let start = Instant::now();
    
    let mut storage = VectorStorage::open(db_path).unwrap();
    
    // Generate 10k random vectors
    for i in 0..10_000 {
        let vector: Vec<f32> = (0..128).map(|j| (i * j) as f32 / 1000.0).collect();
        let entry = VectorEntry::new(vector, format!("Vector {}", i));
        storage.store(entry).unwrap();
        
        if (i + 1) % 1000 == 0 {
            println!("   Stored {} vectors...", i + 1);
        }
    }
    
    let duration = start.elapsed();
    println!("\n✓ Stored 10,000 vectors in {:?}", duration);
    println!("   Average: {:.2}ms per vector", duration.as_millis() as f32 / 10_000.0);
    
    // Random access test
    println!("\nRandom access test (100 reads):");
    let start = Instant::now();
    
    let all_entries = storage.get_all();
    for i in 0..100 {
        let idx = (i * 73) % all_entries.len(); // Pseudo-random
        let entry = &all_entries[idx];
        let _ = storage.get(&entry.id);
    }
    
    let duration = start.elapsed();
    println!("   100 random reads in {:?}", duration);
    println!("   Average: {:.2}μs per read", duration.as_micros() as f32 / 100.0);
    
    // File size
    let metadata = std::fs::metadata(db_path).unwrap();
    println!("\nFile size: {} MB", metadata.len() / 1_024_000);
    
    println!("\n✓ mmap handles large datasets efficiently!");
    println!("  (OS only loads pages into memory as needed)");
    
    // Clean up
    std::fs::remove_file(db_path).unwrap();
    std::fs::remove_file("test_mmap.index.db").unwrap();
}
