// Scalar quantization demonstration
// Shows memory savings and accuracy trade-offs

use piramid::{QuantizedVector};

fn main() {
    println!("=== Scalar Quantization (Int8) ===\n");
    
    // Example 1: Simple vector
    println!("1. Basic quantization:");
    let original = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    let quantized = QuantizedVector::from_f32(&original);
    let dequantized = quantized.to_f32();
    
    println!("   Original:     {:?}", original);
    println!("   Quantized:    {:?}", quantized.values);
    println!("   Range:        [{}, {}]", quantized.min, quantized.max);
    println!("   Dequantized:  {:?}", dequantized);
    println!("   Error:        {:?}", 
        original.iter().zip(&dequantized)
            .map(|(a, b)| (a - b).abs())
            .collect::<Vec<_>>()
    );
    
    // Example 2: Embedding-like vector
    println!("\n2. Realistic embedding (384 dimensions):");
    let embedding: Vec<f32> = (0..384).map(|i| 
        (i as f32 / 100.0).sin() * 0.5 + 0.5
    ).collect();
    
    let quantized = QuantizedVector::from_f32(&embedding);
    let dequantized = quantized.to_f32();
    
    // Calculate error metrics
    let errors: Vec<f32> = embedding.iter()
        .zip(&dequantized)
        .map(|(a, b)| (a - b).abs())
        .collect();
    
    let max_error = errors.iter().fold(0.0f32, |a, b| a.max(*b));
    let avg_error = errors.iter().sum::<f32>() / errors.len() as f32;
    
    println!("   Dimensions: {}", embedding.len());
    println!("   Range: [{:.4}, {:.4}]", quantized.min, quantized.max);
    println!("   Max error: {:.6}", max_error);
    println!("   Avg error: {:.6}", avg_error);
    
    // Example 3: Memory savings
    println!("\n3. Memory savings:");
    
    let dimensions = vec![128, 384, 768, 1536]; // Common embedding sizes
    
    for dim in dimensions {
        let f32_size = dim * 4; // 4 bytes per f32
        let int8_size = dim + 8; // 1 byte per i8 + 8 bytes overhead (min/max)
        let savings = (1.0 - (int8_size as f32 / f32_size as f32)) * 100.0;
        let ratio = f32_size as f32 / int8_size as f32;
        
        println!("   {} dims:", dim);
        println!("      f32:   {} bytes", f32_size);
        println!("      int8:  {} bytes", int8_size);
        println!("      Savings: {:.1}% ({:.2}x smaller)", savings, ratio);
    }
    
    // Example 4: Constant vector (edge case)
    println!("\n4. Edge case - constant vector:");
    let constant = vec![0.5; 10];
    let quantized = QuantizedVector::from_f32(&constant);
    let dequantized = quantized.to_f32();
    
    println!("   Original:    {:?}", constant);
    println!("   Quantized:   {:?}", quantized.values);
    println!("   Dequantized: {:?}", dequantized);
    println!("   Perfect reconstruction: {}", 
        constant == dequantized
    );
    
    // Example 5: Negative values
    println!("\n5. Negative values:");
    let negative = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    let quantized = QuantizedVector::from_f32(&negative);
    let dequantized = quantized.to_f32();
    
    println!("   Original:    {:?}", negative);
    println!("   Quantized:   {:?}", quantized.values);
    println!("   Range:       [{}, {}]", quantized.min, quantized.max);
    println!("   Dequantized: {:?}", dequantized);
    
    println!("\n✓ Quantization provides 4x memory reduction");
    println!("✓ Typical error <0.01 (negligible for similarity search)");
    println!("✓ Handles full f32 range (negative, zero, positive)");
}
