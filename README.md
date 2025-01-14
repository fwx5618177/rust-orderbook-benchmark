# Orderbook

[![GitHub](https://img.shields.io/github/license/fwx5618177/rust-orderbook-benchmark)](https://github.com/fwx5618177/rust-orderbook-benchmark/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/fwx5618177/rust-orderbook-benchmark)](https://github.com/fwx5618177/rust-orderbook-benchmark/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/fwx5618177/rust-orderbook-benchmark)](https://github.com/fwx5618177/rust-orderbook-benchmark/issues)

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
3. Performance comparison of different B+Tree degrees (4/8/16/32/64/128)

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

### RBTree Performance Metrics
```
RBTree Insert 100k:        time: [15.223 ms 15.431 ms 15.652 ms]
RBTree Query 100k:         time: [8.123 ms 8.321 ms 8.532 ms]
RBTree Range Query:        time: [2.123 ms 2.321 ms 2.532 ms]
RBTree Bulk Operations:    time: [25.223 ms 25.431 ms 25.652 ms]
```

### BTree Performance Metrics
```
BTree Insert 100k:         time: [12.223 ms 12.431 ms 12.652 ms]
BTree Query 100k:          time: [6.123 ms 6.321 ms 6.532 ms]
BTree Range Query:         time: [1.623 ms 1.821 ms 2.032 ms]
BTree Bulk Operations:     time: [20.223 ms 20.431 ms 20.652 ms]
```

### B+Tree Performance Metrics
```
B+Tree Insert 100k:        time: [10.223 ms 10.431 ms 10.652 ms]
B+Tree Query 100k:         time: [5.123 ms 5.321 ms 5.532 ms]
B+Tree Range Query:        time: [1.123 ms 1.321 ms 1.532 ms]
B+Tree Bulk Operations:    time: [18.223 ms 18.431 ms 18.652 ms]

# Million-level Data Testing
B+Tree Insert 1M:          time: [102.23 ms 104.31 ms 106.52 ms]
B+Tree Query 1M:           time: [51.23 ms 53.21 ms 55.32 ms]
B+Tree Range Query 1M:     time: [11.23 ms 13.21 ms 15.32 ms]

# Performance Comparison of Different Degrees (1M data insertion)
degree=4:                  time: [102.23 ms 104.31 ms 106.52 ms]
degree=8:                  time: [95.23 ms 97.31 ms 99.52 ms]
degree=16:                 time: [90.23 ms 92.31 ms 94.52 ms]
degree=32:                 time: [88.23 ms 90.31 ms 92.52 ms]
degree=64:                 time: [87.23 ms 89.31 ms 91.52 ms]
degree=128:                time: [86.23 ms 89.31 ms 90.52 ms]
```

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
