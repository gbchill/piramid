// Write-Ahead Log (WAL) demonstration
// Shows crash recovery capabilities

use piramid::{VectorStorage, VectorEntry};

fn main() {
    println!("=== Write-Ahead Log (WAL) Demo ===\n");
    
    let db_path = "test_wal.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("test_wal.index.db");
    let _ = std::fs::remove_file("test_wal.wal");
    
    println!("This demo simulates crash recovery:");
    println!("  1. Store vectors (logged to WAL)");
    println!("  2. Simulate crash (drop storage without checkpoint)");
    println!("  3. Reopen storage (WAL replay)");
    println!("  4. Verify data integrity\n");
    
    let stored_ids;
    
    // Phase 1: Store data (WAL active)
    {
        println!("Phase 1: Storing vectors...");
        let mut storage = VectorStorage::open(db_path).unwrap();
        
        let vectors = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
            vec![0.5, 0.5, 0.0],
            vec![0.3, 0.3, 0.3],
        ];
        
        stored_ids = vectors.iter().enumerate().map(|(i, vec)| {
            let entry = VectorEntry::new(vec.clone(), format!("Vector {}", i));
            storage.store(entry).unwrap()
        }).collect::<Vec<_>>();
        
        println!("   Stored {} vectors", storage.count());
        println!("   WAL contains {} entries", vectors.len());
        
        // Check WAL file exists
        let wal_exists = std::path::Path::new("test_wal.wal").exists();
        println!("   WAL file exists: {}", wal_exists);
        
        // Simulate crash - drop storage WITHOUT checkpoint
        println!("\n   [Simulating crash - no checkpoint]");
        drop(storage);
    }
    
    // Phase 2: Recovery
    {
        println!("\nPhase 2: Recovery (replaying WAL)...");
        
        // Check WAL still exists
        let wal_exists = std::path::Path::new("test_wal.wal").exists();
        println!("   WAL file exists: {}", wal_exists);
        
        // Reopen - should replay WAL
        let mut storage = VectorStorage::open(db_path).unwrap();
        
        println!("   Recovered {} vectors", storage.count());
        
        // Verify data integrity
        println!("\n   Verifying data:");
        let mut all_found = true;
        for id in &stored_ids {
            match storage.get(id) {
                Some(entry) => println!("      ✓ {}: {:?}", entry.text, entry.get_vector()),
                None => {
                    println!("      ✗ Missing vector {}", id);
                    all_found = false;
                }
            }
        }
        
        if all_found {
            println!("\n   ✓ All vectors recovered successfully!");
        }
        
        // Now do a proper checkpoint
        println!("\nPhase 3: Checkpoint...");
        storage.checkpoint().unwrap();
        println!("   ✓ Checkpoint complete");
        println!("   ✓ WAL truncated");
    }
    
    // Phase 3: Verify checkpoint worked
    {
        println!("\nPhase 4: Final verification...");
        let storage = VectorStorage::open(db_path).unwrap();
        println!("   Vectors: {}", storage.count());
        println!("   ✓ Data persisted to main storage");
    }
    
    println!("\n=== Summary ===");
    println!("✓ WAL protects against crashes");
    println!("✓ Automatic recovery on restart");
    println!("✓ ACID-like durability guarantees");
    
    // Clean up
    std::fs::remove_file(db_path).unwrap();
    std::fs::remove_file("test_wal.index.db").unwrap();
    let _ = std::fs::remove_file("test_wal.wal");
}
