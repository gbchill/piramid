// Persistence example
// Demonstrates data survives restart

use piramid::{VectorStorage, VectorEntry};

fn main() {
    println!("=== Storage Persistence Demo ===\n");
    
    let db_path = "test_persist.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("test_persist.index.db");
    
    let stored_ids;
    
    // Phase 1: Store data
    {
        println!("Phase 1: Storing data...");
        let mut storage = VectorStorage::open(db_path).unwrap();
        
        let vectors = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        
        stored_ids = vectors.iter().enumerate().map(|(i, vec)| {
            let entry = VectorEntry::new(vec.clone(), format!("Vector {}", i));
            storage.store(entry).unwrap()
        }).collect::<Vec<_>>();
        
        println!("   Stored {} vectors", storage.count());
        println!("   IDs: {:?}\n", stored_ids);
        
        // Storage goes out of scope - files are closed
    }
    
    // Phase 2: Reopen and verify
    {
        println!("Phase 2: Reopening storage...");
        let storage = VectorStorage::open(db_path).unwrap();
        
        println!("   Found {} vectors", storage.count());
        
        // Verify each vector
        for id in &stored_ids {
            let entry = storage.get(id).unwrap();
            println!("   ✓ Retrieved: {} - {:?}", entry.text, entry.get_vector());
        }
    }
    
    println!("\n✓ Persistence test passed!");
    
    // Clean up
    std::fs::remove_file(db_path).unwrap();
    std::fs::remove_file("test_persist.index.db").unwrap();
}
