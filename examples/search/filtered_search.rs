// Filtered search example
// Search with metadata filtering

use piramid::{VectorStorage, VectorEntry, Metric, Filter, metadata};

fn main() {
    println!("=== Filtered Search Example ===\n");
    
    let _ = std::fs::remove_file("test_filtered.db");
    let _ = std::fs::remove_file("test_filtered.index.db");
    
    let mut storage = VectorStorage::open("test_filtered.db").unwrap();
    
    // Create a product catalog with embeddings
    let products = vec![
        (vec![0.9, 0.1, 0.0], "Laptop", "electronics", 999, true),
        (vec![0.85, 0.15, 0.0], "Smartphone", "electronics", 599, true),
        (vec![0.88, 0.12, 0.0], "Tablet", "electronics", 399, false),
        (vec![0.1, 0.9, 0.0], "Desk Chair", "furniture", 199, true),
        (vec![0.15, 0.85, 0.0], "Standing Desk", "furniture", 499, true),
        (vec![0.12, 0.88, 0.0], "Bookshelf", "furniture", 149, false),
        (vec![0.0, 0.1, 0.9], "Running Shoes", "sports", 89, true),
        (vec![0.0, 0.15, 0.85], "Yoga Mat", "sports", 29, true),
    ];
    
    println!("Storing product catalog...");
    for (vec, name, category, price, in_stock) in products {
        let entry = VectorEntry::with_metadata(
            vec,
            name.to_string(),
            metadata([
                ("category", category.into()),
                ("price", (price as i64).into()),
                ("in_stock", in_stock.into()),
            ]),
        );
        storage.store(entry).unwrap();
    }
    println!("   Stored {} products\n", storage.count());
    
    // Query: Looking for tech products
    let query = vec![0.9, 0.1, 0.0];
    
    // Search 1: Only electronics
    println!("Search 1: Electronics only");
    let filter = Filter::new().eq("category", "electronics");
    let results = storage.search_with_filter(&query, 5, Metric::Cosine, Some(&filter));
    print_results(&results);
    
    // Search 2: In stock only
    println!("\nSearch 2: In stock products only");
    let filter = Filter::new().eq("in_stock", true);
    let results = storage.search_with_filter(&query, 5, Metric::Cosine, Some(&filter));
    print_results(&results);
    
    // Search 3: Price range
    println!("\nSearch 3: Products under $200");
    let filter = Filter::new().lt("price", 200i64);
    let results = storage.search_with_filter(&query, 5, Metric::Cosine, Some(&filter));
    print_results(&results);
    
    // Search 4: Combined filters
    println!("\nSearch 4: Electronics under $700 that are in stock");
    let filter = Filter::new()
        .eq("category", "electronics")
        .lt("price", 700i64)
        .eq("in_stock", true);
    let results = storage.search_with_filter(&query, 5, Metric::Cosine, Some(&filter));
    print_results(&results);
    
    // Search 5: Furniture query
    let furniture_query = vec![0.1, 0.9, 0.0];
    println!("\nSearch 5: Available furniture (in stock)");
    let filter = Filter::new()
        .eq("category", "furniture")
        .eq("in_stock", true);
    let results = storage.search_with_filter(&furniture_query, 5, Metric::Cosine, Some(&filter));
    print_results(&results);
    
    println!("\n✓ Filtered search combines semantic similarity + metadata!");
    
    // Clean up
    std::fs::remove_file("test_filtered.db").unwrap();
    std::fs::remove_file("test_filtered.index.db").unwrap();
}

fn print_results(results: &[piramid::SearchResult]) {
    if results.is_empty() {
        println!("   No results found");
        return;
    }
    for (i, r) in results.iter().enumerate() {
        println!("   {}. {:.4} - {} {:?}", 
            i + 1, r.score, r.text, r.metadata);
    }
}
