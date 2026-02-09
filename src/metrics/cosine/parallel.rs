// Parallel implementation of cosine similarity
// Uses rayon for multi-threaded computation

use rayon::prelude::*;

pub fn cosine_similarity_parallel(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same length");
    
    let chunk_size = (a.len() / num_cpus::get()).max(1024);
    
    let (dot, norm_a, norm_b): (f32, f32, f32) = a.par_chunks(chunk_size)
        .zip(b.par_chunks(chunk_size))
        .map(|(chunk_a, chunk_b)| {
            let mut dot = 0.0;
            let mut norm_a = 0.0;
            let mut norm_b = 0.0;
            for i in 0..chunk_a.len() {
                dot += chunk_a[i] * chunk_b[i];
                norm_a += chunk_a[i] * chunk_a[i];
                norm_b += chunk_b[i] * chunk_b[i];
            }
            (dot, norm_a, norm_b)
        })
        .reduce(|| (0.0, 0.0, 0.0), |(d1, na1, nb1), (d2, na2, nb2)| {
            (d1 + d2, na1 + na2, nb1 + nb2)
        });
    
    let denominator = norm_a.sqrt() * norm_b.sqrt();
    if denominator == 0.0 {
        0.0
    } else {
        dot / denominator
    }
}
