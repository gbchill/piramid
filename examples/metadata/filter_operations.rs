// Advanced filter operations
// Demonstrates all filter condition types

use piramid::{VectorStorage, VectorEntry, Filter, metadata, Metric};

fn main() {
    println!("=== Filter Operations ===\n");
    
    let _ = std::fs::remove_file("test_filters.db");
    let _ = std::fs::remove_file("test_filters.index.db");
    
    let mut storage = VectorStorage::open("test_filters.db").unwrap();
    
    // Create a dataset of users
    let users = vec![
        ("Alice", 25, true, vec!["admin", "user"]),
        ("Bob", 30, true, vec!["user"]),
        ("Charlie", 35, false, vec!["user", "moderator"]),
        ("Diana", 28, true, vec!["admin"]),
        ("Eve", 22, false, vec!["user"]),
    ];
    
    println!("Creating user dataset...");
    for (i, (name, age, active, roles)) in users.iter().enumerate() {
        let vec = vec![i as f32, *age as f32, 0.0];
        let entry = VectorEntry::with_metadata(
            vec,
            name.to_string(),
            metadata([
                ("name", (*name).into()),
                ("age", (*age as i64).into()),
                ("active", (*active).into()),
                ("roles", piramid::MetadataValue::Array(
                    roles.iter().map(|r| (*r).into()).collect()
                )),
            ]),
        );
        storage.store(entry).unwrap();
    }
    println!("   Stored {} users\n", storage.count());
    
    let query = vec![0.0, 25.0, 0.0];
    
    // Filter 1: Equality
    println!("1. Equality filter (active = true):");
    let filter = Filter::new().eq("active", true);
    let results = storage.search_with_filter(&query, 10, Metric::Cosine, Some(&filter));
    for r in &results {
        println!("   {} (age: {})", r.text, r.metadata.get("age").unwrap());
    }
    
    // Filter 2: Greater than
    println!("\n2. Greater than filter (age > 28):");
    let filter = Filter::new().gt("age", 28i64);
    let results = storage.search_with_filter(&query, 10, Metric::Cosine, Some(&filter));
    for r in &results {
        println!("   {} (age: {})", r.text, r.metadata.get("age").unwrap());
    }
    
    // Filter 3: Less than
    println!("\n3. Less than filter (age < 30):");
    let filter = Filter::new().lt("age", 30i64);
    let results = storage.search_with_filter(&query, 10, Metric::Cosine, Some(&filter));
    for r in &results {
        println!("   {} (age: {})", r.text, r.metadata.get("age").unwrap());
    }
    
    // Filter 4: Greater than or equal
    println!("\n4. Greater or equal filter (age >= 28):");
    let filter = Filter::new().gte("age", 28i64);
    let results = storage.search_with_filter(&query, 10, Metric::Cosine, Some(&filter));
    for r in &results {
        println!("   {} (age: {})", r.text, r.metadata.get("age").unwrap());
    }
    
    // Filter 5: Less than or equal
    println!("\n5. Less or equal filter (age <= 28):");
    let filter = Filter::new().lte("age", 28i64);
    let results = storage.search_with_filter(&query, 10, Metric::Cosine, Some(&filter));
    for r in &results {
        println!("   {} (age: {})", r.text, r.metadata.get("age").unwrap());
    }
    
    // Filter 6: Combined conditions (chained)
    println!("\n6. Combined filters (active=true AND age>=25 AND age<=30):");
    let filter = Filter::new()
        .eq("active", true)
        .gte("age", 25i64)
        .lte("age", 30i64);
    let results = storage.search_with_filter(&query, 10, Metric::Cosine, Some(&filter));
    for r in &results {
        println!("   {} (age: {}, active: {})", 
            r.text, 
            r.metadata.get("age").unwrap(),
            r.metadata.get("active").unwrap()
        );
    }
    
    // Filter 7: String equality
    println!("\n7. String filter (name = 'Alice'):");
    let filter = Filter::new().eq("name", "Alice");
    let results = storage.search_with_filter(&query, 10, Metric::Cosine, Some(&filter));
    for r in &results {
        println!("   {}", r.text);
    }
    
    println!("\n✓ Filters support: eq, gt, lt, gte, lte");
    println!("✓ Multiple conditions work as AND (all must match)");
    
    // Clean up
    std::fs::remove_file("test_filters.db").unwrap();
    std::fs::remove_file("test_filters.index.db").unwrap();
}
