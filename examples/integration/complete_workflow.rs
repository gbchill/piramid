// Complete workflow example
// Combines multiple features in a realistic scenario

use piramid::{
    VectorStorage, VectorEntry, Metric, Filter, metadata,
    embeddings::{EmbeddingConfig, providers::create_embedder},
};

#[tokio::main]
async fn main() -> piramid::Result<()> {
    println!("=== Complete Workflow Example ===");
    println!("Scenario: Building a product recommendation system\n");
    
    // Setup
    let use_ollama = std::env::var("USE_OLLAMA").is_ok();
    let config = if use_ollama {
        EmbeddingConfig {
            provider: "ollama".to_string(),
            model: "nomic-embed-text".to_string(),
            api_key: None,
            base_url: Some("http://localhost:11434".to_string()),
            options: serde_json::json!({}),
        }
    } else {
        let api_key = std::env::var("OPENAI_API_KEY")
            .expect("Set OPENAI_API_KEY or USE_OLLAMA=1");
        EmbeddingConfig {
            provider: "openai".to_string(),
            model: "text-embedding-3-small".to_string(),
            api_key: Some(api_key),
            base_url: None,
            options: serde_json::json!({}),
        }
    };
    
    let embedder = create_embedder(&config)?;
    
    // Product catalog
    let products = vec![
        ("Premium wireless noise-canceling headphones", "electronics", 299, 50),
        ("Ergonomic mechanical keyboard with RGB", "electronics", 149, 30),
        ("4K ultra-wide gaming monitor", "electronics", 599, 15),
        ("Comfortable office chair with lumbar support", "furniture", 399, 20),
        ("Standing desk with electric adjustment", "furniture", 499, 10),
        ("Minimalist desk organizer set", "furniture", 49, 100),
        ("Running shoes with advanced cushioning", "sports", 129, 75),
        ("Yoga mat with alignment guides", "sports", 39, 60),
        ("Smart fitness tracker watch", "electronics", 199, 40),
        ("Professional camera tripod", "electronics", 89, 35),
    ];
    
    // Step 1: Batch embed and store
    println!("Step 1: Building product catalog...");
    let db_path = "product_catalog.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("product_catalog.index.db");
    
    let mut storage = VectorStorage::open(db_path)?;
    
    let texts: Vec<String> = products.iter().map(|(desc, _, _, _)| desc.to_string()).collect();
    let embeddings = embedder.embed_batch(&texts).await?;
    
    let entries: Vec<VectorEntry> = products.iter()
        .zip(embeddings)
        .map(|((desc, category, price, stock), embedding)| {
            VectorEntry::with_metadata(
                embedding.embedding,
                desc.to_string(),
                metadata([
                    ("category", (*category).into()),
                    ("price", (*price as i64).into()),
                    ("stock", (*stock as i64).into()),
                    ("in_stock", (*stock > 0).into()),
                ]),
            )
        })
        .collect();
    
    storage.store_batch(entries)?;
    println!("   ✓ Indexed {} products\n", storage.count());
    
    // Step 2: Semantic search
    println!("Step 2: Semantic product search...");
    let query = "computer accessories for developers";
    println!("   Query: \"{}\"", query);
    
    let query_embedding = embedder.embed(query).await?;
    let results = storage.search(&query_embedding.embedding, 5, Metric::Cosine);
    
    println!("   Results:");
    for (i, r) in results.iter().enumerate() {
        println!("      {}. {} (${}, stock: {})", 
            i + 1, r.text,
            r.metadata.get("price").unwrap(),
            r.metadata.get("stock").unwrap()
        );
    }
    
    // Step 3: Filtered search
    println!("\nStep 3: Filtered search (electronics under $200)...");
    let filter = Filter::new()
        .eq("category", "electronics")
        .lt("price", 200i64)
        .eq("in_stock", true);
    
    let results = storage.search_with_filter(
        &query_embedding.embedding, 
        10, 
        Metric::Cosine, 
        Some(&filter)
    );
    
    println!("   Results:");
    for (i, r) in results.iter().enumerate() {
        println!("      {}. {} (${}, stock: {})", 
            i + 1, r.text,
            r.metadata.get("price").unwrap(),
            r.metadata.get("stock").unwrap()
        );
    }
    
    // Step 4: Recommendations (similar products)
    println!("\nStep 4: Similar product recommendations...");
    let product = "Premium wireless noise-canceling headphones";
    println!("   Based on: \"{}\"", product);
    
    let product_embedding = embedder.embed(product).await?;
    let similar = storage.search(&product_embedding.embedding, 4, Metric::Cosine);
    
    println!("   You might also like:");
    for (i, r) in similar.iter().skip(1).enumerate() { // Skip first (itself)
        println!("      {}. {} (${}, score: {:.3})", 
            i + 1, r.text,
            r.metadata.get("price").unwrap(),
            r.score
        );
    }
    
    // Step 5: Update inventory
    println!("\nStep 5: Update product metadata (stock change)...");
    let entry = storage.get_all().iter()
        .find(|e| e.text.contains("yoga mat"))
        .unwrap()
        .clone();
    
    let new_stock = 65;
    let updated_metadata = metadata([
        ("category", "sports".into()),
        ("price", 39i64.into()),
        ("stock", new_stock.into()),
        ("in_stock", (new_stock > 0).into()),
    ]);
    
    storage.update_metadata(&entry.id, updated_metadata)?;
    println!("   ✓ Updated stock for: {}", entry.text);
    
    // Step 6: Persistence
    println!("\nStep 6: Checkpoint and persistence...");
    storage.checkpoint()?;
    storage.flush()?;
    println!("   ✓ Data persisted to disk");
    
    // Summary
    println!("\n=== Workflow Complete ===");
    println!("✓ Batch embedded {} products", products.len());
    println!("✓ Semantic search with embeddings");
    println!("✓ Filtered search (category + price)");
    println!("✓ Similar item recommendations");
    println!("✓ Metadata updates");
    println!("✓ Data persistence with WAL");
    
    // Clean up
    std::fs::remove_file(db_path)?;
    std::fs::remove_file("product_catalog.index.db")?;
    let _ = std::fs::remove_file("product_catalog.wal");
    
    Ok(())
}
