// Execution mode configuration for vector operations
// Allows users to choose between implementations

use serde::{Deserialize, Serialize};

// Execution mode for vector operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionMode {
    Auto,
    Simd,
    Scalar,
    // Offload to GPU via CUDA/OpenCL
    Gpu,
    // Multi-threaded CPU execution
    Parallel,
    // Use bitwise operations on 1-bit quantized vectors
    Binary,
    // Use Just-In-Time compiled kernels for specific vector dimensions
    Jit,
}
impl Default for ExecutionMode {
    fn default() -> Self {
        ExecutionMode::Auto
    }
}

impl ExecutionMode {
    // Resolve execution mode based on CPU capabilities
    pub fn resolve(&self) -> ExecutionMode {
        match self {
            ExecutionMode::Auto => {
                // Auto-detect best execution mode based on CPU features
                #[cfg(target_arch = "x86_64")]
                {
                    if is_x86_feature_detected!("avx2") {
                        ExecutionMode::Simd
                    } else {
                        ExecutionMode::Scalar
                    }
                }
                
                #[cfg(target_arch = "aarch64")]
                {
                    if std::arch::is_aarch64_feature_detected!("neon") {
                        ExecutionMode::Simd
                    } else {
                        ExecutionMode::Scalar
                    }
                }
                
                #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
                {
                    ExecutionMode::Scalar
                }
            },
            // For explicit modes, return as-is (with fallback to Scalar for unsupported)
            ExecutionMode::Simd => {
                #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
                {
                    ExecutionMode::Simd
                }
                #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
                {
                    ExecutionMode::Scalar
                }
            },
            ExecutionMode::Scalar => ExecutionMode::Scalar,
            ExecutionMode::Gpu => {
                // GPU not yet implemented, fallback to SIMD/Scalar
                ExecutionMode::Auto.resolve()
            },
            ExecutionMode::Parallel => {
                // Parallel uses multi-threading with best available vector ops
                ExecutionMode::Parallel
            },
            ExecutionMode::Binary => {
                // Binary quantization not yet implemented, fallback
                ExecutionMode::Auto.resolve()
            },
            ExecutionMode::Jit => {
                // JIT compilation not yet implemented, fallback
                ExecutionMode::Auto.resolve()
            },
        }
    }
    
    // Check if SIMD is available and should be used
    pub fn use_simd(&self) -> bool {
        matches!(self.resolve(), ExecutionMode::Simd | ExecutionMode::Parallel)
    }
    
    // Check if parallel execution should be used
    pub fn use_parallel(&self) -> bool {
        matches!(self.resolve(), ExecutionMode::Parallel)
    }
}
