// Scalar implementation of cosine similarity
// Pure Rust, no vectorization

pub fn cosine_similarity_scalar(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same length");
    
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    
    for i in 0..a.len() {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    
    let denominator = norm_a.sqrt() * norm_b.sqrt();
    
    if denominator == 0.0 {
        0.0
    } else {
        dot / denominator
    }
}
