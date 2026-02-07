// Basic storage operations example
// Demonstrates: create, store, retrieve, delete

use piramid::{VectorStorage, VectorEntry};

fn main() {
    println!("=== Basic Storage Operations ===\n");
    
    // Clean up
    let _ = std::fs::remove_file("test_basic.db");
    let _ = std::fs::remove_file("test_basic.index.db");
    
    // Create storage
    let mut storage = VectorStorage::open("test_basic.db").unwrap();
    println!("✓ Storage created: test_basic.db\n");
    
    // Store a vector
    println!("1. Store Operation:");
    let entry = VectorEntry::new(
        vec![1.0, 2.0, 3.0, 4.0],
        "My first vector".to_string(),
    );
    let id = storage.store(entry).unwrap();
    println!("   Stored vector with ID: {}", id);
    println!("   Total vectors: {}\n", storage.count());
    
    // Retrieve the vector
    println!("2. Retrieve Operation:");
    let retrieved = storage.get(&id).unwrap();
    println!("   ID: {}", retrieved.id);
    println!("   Vector: {:?}", retrieved.get_vector());
    println!("   Text: {}\n", retrieved.text);
    
    // Store multiple vectors
    println!("3. Store Multiple Vectors:");
    for i in 1..=5 {
        let entry = VectorEntry::new(
            vec![i as f32, i as f32 * 2.0, i as f32 * 3.0],
            format!("Vector {}", i),
        );
        let id = storage.store(entry).unwrap();
        println!("   Stored: {}", id);
    }
    println!("   Total vectors: {}\n", storage.count());
    
    // Get all vectors
    println!("4. Get All Vectors:");
    let all = storage.get_all();
    for (idx, entry) in all.iter().enumerate() {
        println!("   {}: {} - {:?}", idx + 1, entry.text, entry.get_vector());
    }
    println!();
    
    // Delete a vector
    println!("5. Delete Operation:");
    let deleted = storage.delete(&id).unwrap();
    println!("   Deleted: {}", deleted);
    println!("   Total vectors: {}\n", storage.count());
    
    // Update vector
    println!("6. Update Vector:");
    let first_entry = storage.get_all()[0].clone();
    let new_vector = vec![99.0, 88.0, 77.0];
    storage.update_vector(&first_entry.id, new_vector.clone()).unwrap();
    let updated = storage.get(&first_entry.id).unwrap();
    println!("   Updated vector: {:?}", updated.get_vector());
    
    println!("\n✓ All operations completed successfully!");
    
    // Clean up
    std::fs::remove_file("test_basic.db").unwrap();
    std::fs::remove_file("test_basic.index.db").unwrap();
}
