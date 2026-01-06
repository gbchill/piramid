<img width="1114" height="191" alt="Screenshot 2025-11-23 at 12 47 47 AM" src="https://github.com/user-attachments/assets/efaa4c47-62d1-4397-9899-8bd58d400fc6" />

<p align="center">
    <b>Vector database for Agentic Applications</b>
</p>

---


### Phase 1: Core Foundation 
- [x] Basic vector storage (HashMap + file persistence)
- [x] Binary serialization with bincode
- [x] UUID-based document IDs
- [x] Error handling with thiserror
- [x] Store and retrieve vectors by ID
- [x] Get all vectors
- [x] Persistence to disk

### Phase 2: Search & Similarity 
- [ ] **Distance metrics module**
  - [ ] Cosine similarity
  - [ ] Euclidean distance
  - [ ] Dot product
- [ ] **Similarity search API**
  - [ ] `search(query_vector, top_k)` → returns nearest neighbors
  - [ ] Return results with scores
- [ ] **Metadata support**
  - [ ] Add `metadata: HashMap<String, Value>` to VectorEntry
  - [ ] JSON-like metadata storage
- [ ] **Filtered search**
  - [ ] Filter by metadata during search
  - [ ] Support basic operators (eq, gt, lt, in)

### Phase 3: Data Operations
- [ ] **Delete operations**
  - [ ] Delete by ID
  - [ ] Bulk delete
- [ ] **Update operations**
  - [ ] Update vector by ID
  - [ ] Update metadata by ID
- [ ] **Batch operations**
  - [ ] Batch insert (insert many vectors at once)
  - [ ] Batch search (multiple queries)
- [ ] **Validation**
  - [ ] Dimension consistency checks
  - [ ] Vector normalization option

### Phase 4: Indexing (Performance)
- [ ] **HNSW (Hierarchical Navigable Small World)**
  - [ ] Build HNSW graph on insert
  - [ ] Approximate nearest neighbor search
  - [ ] Configurable ef_construction and M parameters
- [ ] **IVF (Inverted File Index)** - optional
  - [ ] K-means clustering
  - [ ] Cluster-based search
- [ ] **Index persistence**
  - [ ] Save/load index to disk
  - [ ] Incremental index updates

### Phase 5: Performance Optimization
- [ ] **SIMD acceleration**
  - [ ] SIMD distance calculations (AVX2/AVX-512)
  - [ ] Portable SIMD fallback
- [ ] **Memory optimization**
  - [ ] Memory-mapped files (mmap)
  - [ ] Quantization (scalar/product quantization)
- [ ] **Parallel processing**
  - [ ] Parallel search with rayon
  - [ ] Concurrent inserts

### Phase 6: Collections & Organization
- [ ] **Collections/Namespaces**
  - [ ] Create/delete collections
  - [ ] Separate storage per collection
  - [ ] Collection-level configuration
- [ ] **Schema support**
  - [ ] Define expected dimensions per collection
  - [ ] Metadata schema validation

### Phase 7: Production Features
- [ ] **Async API**
  - [ ] Async storage operations
  - [ ] Tokio integration
- [ ] **Server mode** (optional)
  - [ ] REST API
  - [ ] gRPC API
- [ ] **Observability**
  - [ ] Metrics (insert latency, search latency, index size)
  - [ ] Logging

### Phase 8: Advanced Features (Future Innovation)
- [ ] Hybrid search (vector + keyword)
- [ ] Multi-vector documents
- [ ] Clustering & auto-organization
- [ ] Streaming inserts
- [ ] Replication
- [ ] Custom distance functions

---

## License


