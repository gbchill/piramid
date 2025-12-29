<img width="1114" height="191" alt="Screenshot 2025-11-23 at 12 47 47 AM" src="https://github.com/user-attachments/assets/efaa4c47-62d1-4397-9899-8bd58d400fc6" />

<p align="center">
    <b>Vector database for Agentic Applications</b>
</p>

## status: phase 1 complete ✅

basic vector storage working. stores vectors with text to disk and loads them back.

## what works now

- store vectors (embeddings) with their text
- save/load to disk using binary serialization
- retrieve vectors by id
- persistent storage (survives restart)

## usage

```rust
use piramid::{Config, VectorStorage, VectorEntry};

// open storage
let config = Config::default();
let mut storage = VectorStorage::open("./data/vectors.db")?;

// store a vector
let entry = VectorEntry::new(vec![0.1, 0.2, 0.3], "hello world".to_string());
let id = storage.store(entry)?;

// get it back
let retrieved = storage.get(&id).unwrap();
println!("{}", retrieved.text);
```

## roadmap

**phase 1: core infrastructure** ✅
- [x] rust project setup
- [x] config struct (storage path)
- [x] vector storage with persistence
- [x] tests (store/retrieve + persistence)

**phase 2: indexing** (next)
- [ ] document chunking
- [ ] embedding provider integration
- [ ] index method for processing documents

**phase 3: search**
- [ ] cosine similarity
- [ ] vector search
- [ ] query method

**phase 4: crud**
- [ ] add/delete documents
- [ ] update operations

## testing

```bash
cargo test
```

currently 2 passing tests:
- store and retrieve vectors
- persistence across restarts

## architecture

simple and clean:
- **185 lines total**
- config, storage, error handling
- in-memory hashmap backed by disk
- bincode serialization
- no external dependencies except serde/bincode/uuid

see [src/README.md](src/README.md) for detailed architecture docs.
