# Examples

Comprehensive examples demonstrating all features of Piramid 

## 📁 Directory Structure

```
examples/
├── basic.rs                 # Original quick start example
├── embeddings.rs            # Original embedding example
├── storage/                 # Storage layer examples
├── search/                  # Search operations
├── metadata/                # Metadata and filtering
├── batch/                   # Batch operations
├── index/                   # HNSW indexing
├── quantization/            # Vector compression
├── wal/                     # Write-Ahead Log
└── integration/             # Real-world use cases
```

## 🚀 Quick Start

Run any example with:
```bash
cargo run --example <name>
```

For embedding examples:
```bash
# With OpenAI
export OPENAI_API_KEY=sk-...
cargo run --example embeddings

# With Ollama (local)
export USE_OLLAMA=1
cargo run --example embeddings
```

