// End-to-end RAG (Retrieval Augmented Generation) example
// Demonstrates building a simple document Q&A system

use piramid::{
    VectorStorage, VectorEntry, Metric, metadata,
    embeddings::{EmbeddingConfig, providers::create_embedder},
};

#[tokio::main]
async fn main() -> piramid::Result<()> {
    println!("=== RAG (Retrieval Augmented Generation) Example ===\n");
    
    // Setup
    let use_ollama = std::env::var("USE_OLLAMA").is_ok();
    
    let config = if use_ollama {
        println!("Using Ollama for embeddings\n");
        EmbeddingConfig {
            provider: "ollama".to_string(),
            model: "nomic-embed-text".to_string(),
            api_key: None,
            base_url: Some("http://localhost:11434".to_string()),
            options: serde_json::json!({}),
        }
    } else {
        println!("Using OpenAI for embeddings");
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
    println!("   Provider: {}", embedder.provider_name());
    println!("   Model: {}\n", embedder.model_name());
    
    // Knowledge base - documents about Piramid
    let documents = vec![
        ("Piramid is a vector database written in Rust, designed for AI applications.", "intro"),
        ("Piramid uses HNSW indexing for fast approximate nearest neighbor search.", "indexing"),
        ("The database supports OpenAI and Ollama embedding providers.", "embeddings"),
        ("Vectors are stored using memory-mapped files for efficient I/O.", "storage"),
        ("Piramid includes a Write-Ahead Log for crash recovery.", "durability"),
        ("Scalar quantization reduces memory usage by 4x with minimal accuracy loss.", "optimization"),
        ("The REST API is built with Axum and supports batch operations.", "api"),
        ("Metadata filtering allows combining semantic search with structured queries.", "search"),
    ];
    
    // Build knowledge base
    println!("Building knowledge base ({} documents)...", documents.len());
    let db_path = "rag_knowledge_base.db";
    let _ = std::fs::remove_file(db_path);
    let _ = std::fs::remove_file("rag_knowledge_base.index.db");
    
    let mut storage = VectorStorage::open(db_path)?;
    
    for (text, topic) in &documents {
        let response = embedder.embed(text).await?;
        let entry = VectorEntry::with_metadata(
            response.embedding,
            text.to_string(),
            metadata([
                ("topic", (*topic).into()),
                ("length", (text.len() as i64).into()),
            ]),
        );
        storage.store(entry)?;
    }
    
    println!("   ✓ Knowledge base ready with {} documents\n", storage.count());
    
    // Question answering
    let questions = vec![
        "How does Piramid handle crashes?",
        "What indexing algorithm does Piramid use?",
        "What embedding providers are supported?",
        "How much memory can be saved?",
    ];
    
    println!("=== Question Answering ===\n");
    
    for (i, question) in questions.iter().enumerate() {
        println!("{}. Question: \"{}\"", i + 1, question);
        
        // Embed question
        let response = embedder.embed(question).await?;
        
        // Retrieve relevant documents
        let results = storage.search(&response.embedding, 2, Metric::Cosine);
        
        println!("   Retrieved context:");
        for (j, result) in results.iter().enumerate() {
            println!("      {}. [score: {:.4}] {}", 
                j + 1, result.score, result.text);
        }
        
        println!("   → Answer would be generated here using context + LLM");
        println!();
    }
    
    println!("=== RAG Pipeline ===");
    println!("1. User asks question");
    println!("2. Embed question → query vector");
    println!("3. Search knowledge base → retrieve relevant docs");
    println!("4. Pass context + question to LLM → generate answer");
    println!("\n✓ Piramid handles steps 2-3!");
    
    // Clean up
    std::fs::remove_file(db_path)?;
    std::fs::remove_file("rag_knowledge_base.index.db")?;
    
    Ok(())
}
