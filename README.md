# Orderbook

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
│   └── benchmark.rs          # 统一基准测试逻辑
└── benches
    └── benchmark.rs          # 基准测试入口
```


基准测试场景：
0. 10w条以上的数据 vs 10w条以下的数据的性能对比
1. 高频 vs 低频
2. 小批量 vs 大批量
3. 逐笔交易 vs 批量交易
4. 批量 vs 单笔
5. 单条插入/删除/查询/区间查询的耗时对比
6. 批量插入/删除/查询/区间查询的耗时对比
7. 区间插入/删除/查询/区间查询的耗时对比

性能指标：
- 插入 (Insert)
- 删除 (Delete)
- 查询 (Query)
- 区间查询 (Range Query)
撮合引擎测试
- 订单管理模拟真实交易环境


输出结果：
RBTree Insert         time:   [3.5 ms 3.7 ms 3.8 ms]
RBTree Query          time:   [1.2 ms 1.3 ms 1.4 ms]
BTreeMap Insert       time:   [2.2 ms 2.4 ms 2.5 ms]
BTreeMap Range Query  time:   [0.9 ms 1.0 ms 1.1 ms]
