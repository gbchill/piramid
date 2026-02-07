// k-NN search with different k values
// Shows how k parameter affects results

use piramid::{VectorStorage, VectorEntry, Metric};

fn main() {
    println!("=== k-Nearest Neighbors Search ===\n");
    
    let _ = std::fs::remove_file("test_knn.db");
    let _ = std::fs::remove_file("test_knn.index.db");
    
    let mut storage = VectorStorage::open("test_knn.db").unwrap();
    
    // Create a dataset with clear clusters
    println!("Creating dataset with 3 clusters...\n");
    
    // Cluster 1: Around [1, 0, 0]
    for i in 0..5 {
        let noise = i as f32 * 0.1;
        let vec = vec![1.0 - noise, 0.0 + noise, 0.0];
        let entry = VectorEntry::new(vec, format!("Cluster 1 - Item {}", i));
        storage.store(entry).unwrap();
    }
    
    // Cluster 2: Around [0, 1, 0]
    for i in 0..5 {
        let noise = i as f32 * 0.1;
        let vec = vec![0.0 + noise, 1.0 - noise, 0.0];
        let entry = VectorEntry::new(vec, format!("Cluster 2 - Item {}", i));
        storage.store(entry).unwrap();
    }
    
    // Cluster 3: Around [0, 0, 1]
    for i in 0..5 {
        let noise = i as f32 * 0.1;
        let vec = vec![0.0, 0.0 + noise, 1.0 - noise];
        let entry = VectorEntry::new(vec, format!("Cluster 3 - Item {}", i));
        storage.store(entry).unwrap();
    }
    
    println!("Total vectors: {}\n", storage.count());
    
    // Query near Cluster 1
    let query = vec![0.95, 0.05, 0.0];
    println!("Query: {:?} (near Cluster 1)\n", query);
    
    // Different k values
    for k in [1, 3, 5, 10] {
        println!("k = {} results:", k);
        let results = storage.search(&query, k, Metric::Cosine);
        
        for (i, r) in results.iter().enumerate() {
            println!("   {}. {:.4} - {}", i + 1, r.score, r.text);
        }
        println!();
    }
    
    println!("Observations:");
    println!("  • k=1: Only closest neighbor");
    println!("  • k=3-5: Mostly from target cluster");
    println!("  • k=10: Includes items from other clusters");
    
    // Clean up
    std::fs::remove_file("test_knn.db").unwrap();
    std::fs::remove_file("test_knn.index.db").unwrap();
}
