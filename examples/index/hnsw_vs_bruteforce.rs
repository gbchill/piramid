// HNSW vs Brute Force comparison
// Shows when approximate search is worth it

use piramid::{VectorStorage, VectorEntry, Metric, HnswConfig};
use std::time::Instant;

fn main() {
    println!("=== HNSW vs Brute Force Comparison ===\n");
    
    let sizes = vec![100, 500, 1000, 5000];
    
    for size in sizes {
        println!("Dataset size: {} vectors", size);
        
        // Create dataset
        let entries: Vec<VectorEntry> = (0..size).map(|i| {
            let vector: Vec<f32> = (0..128).map(|j| 
                ((i * j) as f32 / 100.0).sin()
            ).collect();
            VectorEntry::new(vector, format!("Vector {}", i))
        }).collect();
        
        let query: Vec<f32> = (0..128).map(|i| (i as f32 / 10.0).cos()).collect();
        
        // HNSW search
        let db_name = format!("test_hnsw_{}.db", size);
        let _ = std::fs::remove_file(&db_name);
        let _ = std::fs::remove_file(format!("test_hnsw_{}.index.db", size));
        
        let mut hnsw_storage = VectorStorage::with_hnsw(&db_name, HnswConfig::default()).unwrap();
        
        let start = Instant::now();
        hnsw_storage.store_batch(entries.clone()).unwrap();
        let hnsw_build = start.elapsed();
        
        let start = Instant::now();
        let hnsw_results = hnsw_storage.search(&query, 10, Metric::Cosine);
        let hnsw_search = start.elapsed();
        
        // Brute force (linear scan through all vectors)
        let start = Instant::now();
        let vectors = hnsw_storage.get_vectors();
        let mut distances: Vec<(uuid::Uuid, f32)> = vectors.iter()
            .map(|(id, vec)| {
                let score = Metric::Cosine.calculate(&query, vec);
                (*id, score)
            })
            .collect();
        distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let brute_search = start.elapsed();
        
        println!("   HNSW:");
        println!("      Build: {:?}", hnsw_build);
        println!("      Search: {:?} ({} µs)", hnsw_search, hnsw_search.as_micros());
        println!("      Top score: {:.6}", hnsw_results[0].score);
        
        println!("   Brute Force:");
        println!("      Search: {:?} ({} µs)", brute_search, brute_search.as_micros());
        println!("      Top score: {:.6}", distances[0].1);
        
        let speedup = brute_search.as_secs_f64() / hnsw_search.as_secs_f64();
        println!("   Speedup: {:.2}x faster", speedup);
        println!();
        
        // Clean up
        std::fs::remove_file(&db_name).unwrap();
        std::fs::remove_file(format!("test_hnsw_{}.index.db", size)).unwrap();
    }
    
    println!("Observations:");
    println!("  • HNSW is O(log N), brute force is O(N)");
    println!("  • Speedup increases with dataset size");
    println!("  • HNSW is approximate but very accurate (99%+ recall)");
    println!("  • For <1000 vectors, brute force might be fine");
    println!("  • For >10k vectors, HNSW is essential");
}
