// Euclidean distance between two vectors
// Measures the straight-line distance in vector space

mod scalar;
mod simd;
mod parallel;
mod binary;
mod jit;

use crate::config::ExecutionMode;
pub use scalar::{euclidean_distance_scalar, euclidean_distance_squared_scalar};
pub use simd::{euclidean_distance_simd, euclidean_distance_squared_simd};
pub use parallel::{euclidean_distance_parallel, euclidean_distance_squared_parallel};
pub use binary::{euclidean_distance_binary, euclidean_distance_squared_binary};
pub use jit::{euclidean_distance_jit, euclidean_distance_squared_jit};

pub fn euclidean_distance(a: &[f32], b: &[f32], mode: ExecutionMode) -> f32 {
    let resolved = mode.resolve();
    match resolved {
        ExecutionMode::Simd => euclidean_distance_simd(a, b),
        ExecutionMode::Scalar => euclidean_distance_scalar(a, b),
        ExecutionMode::Parallel => euclidean_distance_parallel(a, b),
        ExecutionMode::Binary => euclidean_distance_binary(a, b),
        ExecutionMode::Jit => euclidean_distance_jit(a, b),
        _ => euclidean_distance_scalar(a, b),
    }
}

pub fn euclidean_distance_squared(a: &[f32], b: &[f32], mode: ExecutionMode) -> f32 {
    let resolved = mode.resolve();
    match resolved {
        ExecutionMode::Simd => euclidean_distance_squared_simd(a, b),
        ExecutionMode::Scalar => euclidean_distance_squared_scalar(a, b),
        ExecutionMode::Parallel => euclidean_distance_squared_parallel(a, b),
        ExecutionMode::Binary => euclidean_distance_squared_binary(a, b),
        ExecutionMode::Jit => euclidean_distance_squared_jit(a, b),
        _ => euclidean_distance_squared_scalar(a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_vectors() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(euclidean_distance(&v, &v, crate::config::ExecutionMode::Auto), 0.0);
    }

    #[test]
    fn test_3_4_5_triangle() {
        let v1 = vec![0.0, 0.0];
        let v2 = vec![3.0, 4.0];
        let distance = euclidean_distance(&v1, &v2, crate::config::ExecutionMode::Auto);
        assert!((distance - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_unit_distance() {
        let v1 = vec![0.0];
        let v2 = vec![1.0];
        assert!((euclidean_distance(&v1, &v2, crate::config::ExecutionMode::Auto) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_squared_distance() {
        let v1 = vec![0.0, 0.0];
        let v2 = vec![3.0, 4.0];
        assert!((euclidean_distance_squared(&v1, &v2, crate::config::ExecutionMode::Auto) - 25.0).abs() < 1e-6);
    }

    #[test]
    #[should_panic(expected = "Vectors must have same length")]
    fn test_different_lengths() {
        let v1 = vec![1.0, 2.0];
        let v2 = vec![1.0, 2.0, 3.0];
        euclidean_distance(&v1, &v2, crate::config::ExecutionMode::Auto);
    }
}
