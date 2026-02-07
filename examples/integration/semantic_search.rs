// Semantic search example
// Demonstrates natural language query understanding

use piramid::{
    VectorStorage, VectorEntry, Metric,
    embeddings::{EmbeddingConfig, providers::create_embedder},
};

#[tokio::main]
async fn main() -> piramid::Result<()> {
    println!("=== Semantic Search Example ===\n");
    
    // Setup embedder
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
    println!("Using: {} - {}\n", embedder.provider_name(), embedder.model_name());
    
    // Create a movie database
    let movies = vec![
        "A young wizard attends a magical school and fights dark forces",
        "A hobbit must destroy an evil ring in a fantasy world",
        "Scientist creates dinosaurs from DNA in a theme park",
        "Time travelers go back to prevent robot apocalypse",
        "Space opera about rebellion against galactic empire",
        "Superhero team fights alien invasion in New York",
        "Dream thieves plant ideas in corporate executive's mind",
        "A virus turns people into zombies during zombie apocalypse",
        "Detective solves crime in a noir cyberpunk future",
        "Astronauts communicate with aliens through music",
    ];
    
    println!("Building movie database ({} movies)...", movies.len());
    let db_path = "semantic_search.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("semantic_search.index.db");
    
    let mut storage = VectorStorage::open(db_path)?;
    
    // Embed and store movies
    for (idx, movie) in movies.iter().enumerate() {
        let response = embedder.embed(movie).await?;
        let entry = VectorEntry::new(response.embedding, movie.to_string());
        storage.store(entry)?;
        
        if (idx + 1) % 3 == 0 {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }
    println!(" Done!\n");
    
    // Semantic queries
    let queries = vec![
        "movies about magic and wizards",
        "sci-fi with time travel",
        "stories set in space",
        "films with zombies",
        "superhero action movies",
        "fantasy adventures with rings",
    ];
    
    println!("=== Semantic Search Queries ===\n");
    
    for query in queries {
        println!("Query: \"{}\"", query);
        
        // Embed query
        let response = embedder.embed(query).await?;
        
        // Search
        let results = storage.search(&response.embedding, 3, Metric::Cosine);
        
        println!("   Top matches:");
        for (i, result) in results.iter().enumerate() {
            println!("      {}. [score: {:.4}] {}", 
                i + 1, result.score, result.text);
        }
        println!();
    }
    
    println!("Notice:");
    println!("  • Queries don't use exact keywords");
    println!("  • Semantic meaning is captured");
    println!("  • Synonyms and related concepts match");
    println!("  • \"wizard\" → \"magical school\" ✓");
    println!("  • \"space\" → \"galactic empire\" ✓");
    
    // Clean up
    std::fs::remove_file(db_path)?;
    std::fs::remove_file("semantic_search.index.db")?;
    
    Ok(())
}
