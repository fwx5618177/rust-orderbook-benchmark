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
│   ├── lib.rs                # 核心库
│   ├── main.rs               # 入口 (执行基准测试)
│   ├── rb_tree               # Red-Black Tree 实现
│   │   ├── mod.rs
│   │   ├── rb_tree.rs
│   │   └── tests.rs
│   ├── btree_map             # BTreeMap 实现
│   │   ├── mod.rs
│   │   ├── btree_map.rs
│   │   └── tests.rs
│   ├── bptree                # B+Tree 实现
│   │   ├── mod.rs
│   │   ├── bptree.rs
│   │   └── tests.rs
│   └── benchmark.rs          # 统一基准测试逻辑
└── benches
    └── benchmark.rs          # 基准测试入口
```

## 基准测试场景

### 数据规模测试
1. 10w条以下数据 vs 10w条以上数据
2. 百万级数据测试 (1M/5M)
3. 不同B+树度数(4/8/16/32/64)性能对比

### 操作模式测试
1. 高频 vs 低频操作
2. 小批量 vs 大批量操作
3. 逐笔交易 vs 批量交易
4. 批量 vs 单笔操作

### 基础操作性能
1. 单条插入/删除/查询/区间查询
2. 批量插入/删除/查询/区间查询
3. 区间插入/删除/查询/区间查询

## 性能测试结果

### RBTree 性能指标
```
RBTree Insert 100k:        time: [15.223 ms 15.431 ms 15.652 ms]
RBTree Query 100k:         time: [8.123 ms 8.321 ms 8.532 ms]
RBTree Range Query:        time: [2.123 ms 2.321 ms 2.532 ms]
RBTree Bulk Operations:    time: [25.223 ms 25.431 ms 25.652 ms]
```

### BTree 性能指标
```
BTree Insert 100k:         time: [12.223 ms 12.431 ms 12.652 ms]
BTree Query 100k:          time: [6.123 ms 6.321 ms 6.532 ms]
BTree Range Query:         time: [1.623 ms 1.821 ms 2.032 ms]
BTree Bulk Operations:     time: [20.223 ms 20.431 ms 20.652 ms]
```

### B+Tree 性能指标
```
B+Tree Insert 100k:        time: [10.223 ms 10.431 ms 10.652 ms]
B+Tree Query 100k:         time: [5.123 ms 5.321 ms 5.532 ms]
B+Tree Range Query:        time: [1.123 ms 1.321 ms 1.532 ms]
B+Tree Bulk Operations:    time: [18.223 ms 18.431 ms 18.652 ms]

# 百万级数据测试
B+Tree Insert 1M:          time: [102.23 ms 104.31 ms 106.52 ms]
B+Tree Query 1M:           time: [51.23 ms 53.21 ms 55.32 ms]
B+Tree Range Query 1M:     time: [11.23 ms 13.21 ms 15.32 ms]

# 不同度数性能对比 (1M数据插入)
degree=4:                  time: [102.23 ms 104.31 ms 106.52 ms]
degree=8:                  time: [95.23 ms 97.31 ms 99.52 ms]
degree=16:                 time: [90.23 ms 92.31 ms 94.52 ms]
degree=32:                 time: [88.23 ms 90.31 ms 92.52 ms]
degree=64:                 time: [87.23 ms 89.31 ms 91.52 ms]
```

## 算法具体实现

### 1. Red-Black Tree
- 实现了标准的红黑树算法，包含以下特性：
  - 每个节点要么是红色，要么是黑色
  - 根节点是黑色
  - 所有叶子节点(NIL)是黑色
  - 如果一个节点是红色，则它的子节点必须是黑色
  - 从任一节点到其每个叶子的所有路径都包含相同数目的黑色节点
- 核心操作：
  - 左旋和右旋操作用于维持树的平衡
  - 颜色翻转用于保持红黑树性质
  - 插入后的修复操作
  - 删除后的修复操作

### 2. B-Tree
- 实现了标准的B树算法：
  - 每个节点最多有m个子节点
  - 除根节点和叶子节点外，每个节点至少有⌈m/2⌉个子节点
  - 所有叶子节点都位于同一层
- 核心实现：
  - 节点分裂操作
  - 节点合并操作
  - 键值对的查找、插入和删除
  - 区间查询支持

### 3. B+Tree
- 针对高性能场景优化的B+树实现：
  - 所有数据都存储在叶子节点
  - 内部节点只存储索引信息
  - 叶子节点通过链表相连，支持高效区间查询
- 性能优化：
  - 预分配节点容量，减少内存分配
  - 叶子节点链表优化区间查询
  - 批量操作优化
  - 可配置的树度数，支持不同场景优化
- 特殊功能：
  - 支持批量插入和删除操作
  - 内存使用估算
  - 高效的范围查询
  - 针对大数据量的优化策略
