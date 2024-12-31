 # Orderbook

<div align="center">
  <h2>Language / 语言 / 言語</h2>
  <a href="README.md">English</a> |
  <a href="README_ZH.md">中文</a> |
  <a href="README_JP.md">日本語</a>
</div>

```shell
rust-orderbook-benchmark
├── Cargo.toml
├── src
│   ├── lib.rs                # Core Library
│   ├── main.rs               # Entry Point (Benchmark Execution)
│   ├── rb_tree               # Red-Black Tree Implementation
│   │   ├── mod.rs
│   │   ├── rb_tree.rs
│   │   └── tests.rs
│   ├── btree_map             # BTreeMap Implementation
│   │   ├── mod.rs
│   │   ├── btree_map.rs
│   │   └── tests.rs
│   ├── bptree                # B+Tree Implementation
│   │   ├── mod.rs
│   │   ├── bptree.rs
│   │   └── tests.rs
│   └── benchmark.rs          # Unified Benchmark Logic
└── benches
    └── benchmark.rs          # Benchmark Entry Point
```

## Benchmark Scenarios

### Data Scale Testing
1. Below 100k vs Above 100k entries
2. Million-level data testing (1M/5M)
3. Performance comparison of different B+Tree degrees (4/8/16/32/64)

### Operation Pattern Testing
1. High-frequency vs Low-frequency operations
2. Small batch vs Large batch operations
3. Single trade vs Batch trade
4. Batch vs Single operations

### Basic Operation Performance
1. Single insert/delete/query/range query
2. Batch insert/delete/query/range query
3. Range insert/delete/query/range query

## Performance Test Results

[Performance metrics sections remain the same as they are numerical data]

## Algorithm Implementation Details

### 1. Red-Black Tree
- Standard red-black tree implementation with features:
  - Each node is either red or black
  - Root node is black
  - All leaf nodes (NIL) are black
  - If a node is red, its children must be black
  - All paths from any node to its leaf nodes contain the same number of black nodes
- Core Operations:
  - Left and right rotations for tree balancing
  - Color flipping to maintain red-black properties
  - Post-insertion repair operations
  - Post-deletion repair operations

### 2. B-Tree
- Standard B-tree implementation:
  - Each node can have up to m child nodes
  - Each non-root and non-leaf node has at least ⌈m/2⌉ child nodes
  - All leaf nodes are at the same level
- Core Implementation:
  - Node splitting operations
  - Node merging operations
  - Key-value pair lookup, insertion, and deletion
  - Range query support

### 3. B+Tree
- B+Tree implementation optimized for high-performance scenarios:
  - All data stored in leaf nodes
  - Internal nodes store only index information
  - Leaf nodes connected via linked list for efficient range queries
- Performance Optimizations:
  - Pre-allocated node capacity to reduce memory allocation
  - Leaf node linked list for optimized range queries
  - Batch operation optimization
  - Configurable tree degree for different scenarios
- Special Features:
  - Support for batch insert and delete operations
  - Memory usage estimation
  - Efficient range queries
  - Optimization strategies for large data volumes
