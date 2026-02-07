// Similarity metrics comparison
// Demonstrates different distance metrics and their use cases

use piramid::{VectorStorage, VectorEntry, Metric};

fn main() {
    println!("=== Similarity Metrics Comparison ===\n");
    
    let _ = std::fs::remove_file("test_metrics.db");
    let _ = std::fs::remove_file("test_metrics.index.db");
    
    let mut storage = VectorStorage::open("test_metrics.db").unwrap();
    
    // Store test vectors
    let vectors = vec![
        (vec![1.0, 0.0, 0.0], "Pure X-axis"),
        (vec![0.0, 1.0, 0.0], "Pure Y-axis"),
        (vec![0.0, 0.0, 1.0], "Pure Z-axis"),
        (vec![0.7, 0.7, 0.0], "Diagonal XY"),
        (vec![0.5, 0.5, 0.5], "Uniform"),
        (vec![0.9, 0.1, 0.0], "Mostly X"),
    ];
    
    println!("Stored vectors:");
    for (vec, label) in &vectors {
        let entry = VectorEntry::new(vec.clone(), label.to_string());
        storage.store(entry).unwrap();
        println!("   {} - {:?}", label, vec);
    }
    
    // Query vector
    let query = vec![1.0, 0.0, 0.0];
    println!("\nQuery: {:?} (Pure X-axis)\n", query);
    
    // Compare all metrics
    println!("=== Cosine Similarity ===");
    println!("(Measures angle between vectors, ignores magnitude)");
    let results = storage.search(&query, 3, Metric::Cosine);
    print_results(&results);
    
    println!("\n=== Euclidean Distance ===");
    println!("(Measures straight-line distance in space)");
    let results = storage.search(&query, 3, Metric::Euclidean);
    print_results(&results);
    
    println!("\n=== Dot Product ===");
    println!("(Measures both angle and magnitude)");
    let results = storage.search(&query, 3, Metric::DotProduct);
    print_results(&results);
    
    // Normalized vs unnormalized
    println!("\n=== Impact of Vector Magnitude ===");
    let mut storage2 = VectorStorage::open("test_metrics2.db").unwrap();
    
    let scaled_vectors = vec![
        (vec![1.0, 0.0, 0.0], "Unit vector"),
        (vec![10.0, 0.0, 0.0], "10x scaled"),
        (vec![0.1, 0.0, 0.0], "0.1x scaled"),
    ];
    
    for (vec, label) in &scaled_vectors {
        let entry = VectorEntry::new(vec.clone(), label.to_string());
        storage2.store(entry).unwrap();
    }
    
    println!("Query: [1.0, 0.0, 0.0]");
    println!("\nCosine (magnitude-invariant):");
    let results = storage2.search(&query, 3, Metric::Cosine);
    for r in &results {
        println!("   {:.4} - {}", r.score, r.text);
    }
    
    println!("\nEuclidean (magnitude-sensitive):");
    let results = storage2.search(&query, 3, Metric::Euclidean);
    for r in &results {
        println!("   {:.4} - {}", r.score, r.text);
    }
    
    println!("\n✓ Use Cosine for semantic similarity (embeddings)");
    println!("✓ Use Euclidean for spatial data");
    println!("✓ Use DotProduct for recommendation systems");
    
    // Clean up
    std::fs::remove_file("test_metrics.db").unwrap();
    std::fs::remove_file("test_metrics.index.db").unwrap();
    std::fs::remove_file("test_metrics2.db").unwrap();
    std::fs::remove_file("test_metrics2.index.db").unwrap();
}

fn print_results(results: &[piramid::SearchResult]) {
    for (i, r) in results.iter().enumerate() {
        println!("   {}. Score: {:.4} - {}", i + 1, r.score, r.text);
    }
}
