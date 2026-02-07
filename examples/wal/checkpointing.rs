// WAL checkpointing
// Shows periodic checkpointing to manage WAL size

use piramid::{VectorStorage, VectorEntry};
use std::time::Instant;

fn main() {
    println!("=== WAL Checkpointing ===\n");
    
    let db_path = "test_checkpoint.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("test_checkpoint.index.db");
    let _ = std::fs::remove_file("test_checkpoint.wal");
    
    let mut storage = VectorStorage::open(db_path).unwrap();
    
    // Simulate application lifecycle with periodic checkpoints
    println!("Simulating application with 1000 operations...");
    println!("Checkpointing every 100 operations\n");
    
    let checkpoint_interval = 100;
    let total_ops = 1000;
    
    for i in 0..total_ops {
        // Perform operation
        let vector: Vec<f32> = (0..128).map(|j| ((i * j) as f32 / 100.0).sin()).collect();
        let entry = VectorEntry::new(vector, format!("Vector {}", i));
        storage.store(entry).unwrap();
        
        // Checkpoint periodically
        if (i + 1) % checkpoint_interval == 0 {
            let start = Instant::now();
            storage.checkpoint().unwrap();
            let duration = start.elapsed();
            
            println!("Checkpoint at {} operations (took {:?})", i + 1, duration);
            
            // Check WAL file size (should be small after checkpoint)
            if let Ok(metadata) = std::fs::metadata("test_checkpoint.wal") {
                println!("   WAL size: {} bytes", metadata.len());
            }
        }
    }
    
    println!("\n✓ Operations complete: {} vectors stored", storage.count());
    
    // Final checkpoint
    println!("\nFinal checkpoint...");
    storage.checkpoint().unwrap();
    storage.flush().unwrap();
    println!("   ✓ All data persisted");
    
    // Check final file sizes
    println!("\nFile sizes:");
    if let Ok(metadata) = std::fs::metadata(db_path) {
        println!("   Main DB: {} KB", metadata.len() / 1024);
    }
    if let Ok(metadata) = std::fs::metadata("test_checkpoint.index.db") {
        println!("   Index: {} KB", metadata.len() / 1024);
    }
    if let Ok(metadata) = std::fs::metadata("test_checkpoint.wal") {
        println!("   WAL: {} bytes", metadata.len());
    }
    
    println!("\n=== Benefits of Checkpointing ===");
    println!("✓ Keeps WAL size manageable");
    println!("✓ Faster recovery (less replay)");
    println!("✓ Better I/O performance");
    println!("\nRecommended: Checkpoint every N operations or every T seconds");
    
    // Clean up
    std::fs::remove_file(db_path).unwrap();
    std::fs::remove_file("test_checkpoint.index.db").unwrap();
    std::fs::remove_file("test_checkpoint.wal").unwrap();
}
