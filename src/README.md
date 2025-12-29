# piramid source code

basic vector storage system. stores vectors embeddings with their text on disk.

## what this does

stores vectors to disk and loads them back. that's it. no search, no indexing, just basic storage.

## files

### lib.rs

exports the main types: Config, VectorStorage, VectorEntry, and error types.

### config.rs

holds the config. just the storage path for now. defaults to `./data`.

### error.rs

two error types:

- io errors (file problems)
- serialization errors (can't convert to/from binary)

### storage.rs

the storage layer. reads and writes vectors to disk.

- keeps all vectors in memory (hashmap: uuid -> entry)
- saves the entire hashmap to disk on every write
- uses bincode for binary serialization
- loads everything back on startup

**VectorEntry:**

- id: unique uuid
- vector: array of floats
- text: original text

## testing

run the tests:
```bash
cargo test
```
