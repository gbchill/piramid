// Basic metadata operations
// Store and query different metadata types

use piramid::{VectorStorage, VectorEntry, metadata, MetadataValue};

fn main() {
    println!("=== Metadata Operations ===\n");
    
    let _ = std::fs::remove_file("test_metadata.db");
    let _ = std::fs::remove_file("test_metadata.index.db");
    
    let mut storage = VectorStorage::open("test_metadata.db").unwrap();
    
    // Different metadata types
    println!("1. Storing vectors with various metadata types:\n");
    
    // String metadata
    let entry = VectorEntry::with_metadata(
        vec![1.0, 0.0, 0.0],
        "String metadata example".to_string(),
        metadata([
            ("title", "Example Document".into()),
            ("author", "John Doe".into()),
        ]),
    );
    let id1 = storage.store(entry).unwrap();
    println!("   ✓ String metadata: {}", id1);
    
    // Integer metadata
    let entry = VectorEntry::with_metadata(
        vec![0.0, 1.0, 0.0],
        "Integer metadata example".to_string(),
        metadata([
            ("year", 2024i64.into()),
            ("count", 42i64.into()),
            ("version", 3i64.into()),
        ]),
    );
    let id2 = storage.store(entry).unwrap();
    println!("   ✓ Integer metadata: {}", id2);
    
    // Boolean metadata
    let entry = VectorEntry::with_metadata(
        vec![0.0, 0.0, 1.0],
        "Boolean metadata example".to_string(),
        metadata([
            ("published", true.into()),
            ("verified", false.into()),
            ("featured", true.into()),
        ]),
    );
    let id3 = storage.store(entry).unwrap();
    println!("   ✓ Boolean metadata: {}", id3);
    
    // Float metadata
    let entry = VectorEntry::with_metadata(
        vec![0.5, 0.5, 0.0],
        "Float metadata example".to_string(),
        metadata([
            ("rating", 4.5f64.into()),
            ("price", 99.99f64.into()),
        ]),
    );
    let id4 = storage.store(entry).unwrap();
    println!("   ✓ Float metadata: {}", id4);
    
    // Array metadata
    let entry = VectorEntry::with_metadata(
        vec![0.3, 0.3, 0.3],
        "Array metadata example".to_string(),
        metadata([
            ("tags", MetadataValue::Array(vec![
                "rust".into(),
                "database".into(),
                "vector".into(),
            ])),
        ]),
    );
    let id5 = storage.store(entry).unwrap();
    println!("   ✓ Array metadata: {}", id5);
    
    // Mixed metadata
    let entry = VectorEntry::with_metadata(
        vec![0.2, 0.4, 0.6],
        "Mixed metadata example".to_string(),
        metadata([
            ("name", "Complex Entry".into()),
            ("count", 100i64.into()),
            ("active", true.into()),
            ("score", 87.5f64.into()),
            ("labels", MetadataValue::Array(vec![
                "important".into(),
                "reviewed".into(),
            ])),
        ]),
    );
    let id6 = storage.store(entry).unwrap();
    println!("   ✓ Mixed metadata: {}", id6);
    
    // Retrieve and display
    println!("\n2. Retrieving and displaying metadata:\n");
    
    for id in [id1, id2, id3, id4, id5, id6] {
        let entry = storage.get(&id).unwrap();
        println!("   {} - {}", entry.text, entry.id);
        for (key, value) in &entry.metadata {
            println!("      {}: {:?}", key, value);
        }
        println!();
    }
    
    // Update metadata
    println!("3. Updating metadata:\n");
    let updated_metadata = metadata([
        ("title", "Updated Title".into()),
        ("modified", true.into()),
    ]);
    storage.update_metadata(&id1, updated_metadata).unwrap();
    
    let entry = storage.get(&id1).unwrap();
    println!("   Updated entry:");
    for (key, value) in &entry.metadata {
        println!("      {}: {:?}", key, value);
    }
    
    println!("\n✓ Metadata supports: String, Integer, Float, Boolean, Array!");
    
    // Clean up
    std::fs::remove_file("test_metadata.db").unwrap();
    std::fs::remove_file("test_metadata.index.db").unwrap();
}
