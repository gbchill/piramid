// Dot product of two vectors
// Fast similarity metric for normalized vectors

mod scalar;
mod simd;
mod parallel;
mod binary;
mod jit;

use crate::config::ExecutionMode;
pub use scalar::dot_product_scalar;
pub use simd::dot_product_simd;
pub use parallel::dot_product_parallel;
pub use binary::dot_product_binary;
pub use jit::dot_product_jit;

pub fn dot_product(a: &[f32], b: &[f32], mode: ExecutionMode) -> f32 {
    let resolved = mode.resolve();
    match resolved {
        ExecutionMode::Simd => dot_product_simd(a, b), 
        ExecutionMode::Scalar => dot_product_scalar(a, b),
        ExecutionMode::Parallel => dot_product_parallel(a, b),
        ExecutionMode::Binary => dot_product_binary(a, b),
        ExecutionMode::Jit => dot_product_jit(a, b),
        _ => dot_product_scalar(a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_dot_product() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        let result = dot_product(&v1, &v2, crate::config::ExecutionMode::Auto);
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert!((result - 32.0).abs() < 1e-6);
    }

    #[test]
    fn test_orthogonal_vectors() {
        let v1 = vec![1.0, 0.0];
        let v2 = vec![0.0, 1.0];
        assert!(dot_product(&v1, &v2, crate::config::ExecutionMode::Auto).abs() < 1e-6);
    }

    #[test]
    fn test_unit_vectors() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        assert!((dot_product(&v1, &v2, crate::config::ExecutionMode::Auto) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_negative_values() {
        let v1 = vec![1.0, -1.0];
        let v2 = vec![-1.0, 1.0];
        assert!((dot_product(&v1, &v2, crate::config::ExecutionMode::Auto) - (-2.0)).abs() < 1e-6);
    }

    #[test]
    #[should_panic(expected = "Vectors must have same length")]
    fn test_different_lengths() {
        let v1 = vec![1.0, 2.0];
        let v2 = vec![1.0, 2.0, 3.0];
        dot_product(&v1, &v2, crate::config::ExecutionMode::Auto);
    }

    #[test]
    fn test_simd_large_vectors() {
        // Test with realistic embedding size (OpenAI uses 1536)
        let size = 1536;
        let v1: Vec<f32> = (0..size).map(|i| i as f32 * 0.1).collect();
        let v2: Vec<f32> = (0..size).map(|i| (i + 1) as f32 * 0.2).collect();
        
        let result = dot_product(&v1, &v2, crate::config::ExecutionMode::Auto);
        
        // Verify it's reasonable (should be a large positive number)
        assert!(result > 0.0);
        assert!(result.is_finite());
    }
}
