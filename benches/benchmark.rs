use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::Rng;
use rust_orderbook_benchmark::rb_tree::rb_tree::RBTree;
use rust_orderbook_benchmark::btree_map::btree_map::BTreeMap;
use rust_orderbook_benchmark::bptree::bptree::BPTree;

fn generate_random_pairs(n: usize) -> Vec<(u32, u32)> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| (rng.gen_range(1..100_000), rng.gen_range(1..100_000)))
        .collect()
}

fn bench_b_plus_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("BPTree");
    group.sample_size(10).measurement_time(std::time::Duration::new(5, 0));

    let data_100k = generate_random_pairs(100_000);
    let data_50k = generate_random_pairs(50_000);

    // 0. 10w vs 5w
    group.bench_function("bptree_insert_100k", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    group.bench_function("bptree_insert_50k", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_50k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 1. 高频 vs 低频
    group.bench_function("bptree_insert_high_freq", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_insert_low_freq", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in data_100k.iter().step_by(10) {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 2. 小批量 vs 大批量
    group.bench_function("bptree_insert_small_batch", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in data_100k.iter().take(100) {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_insert_large_batch", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 3. 逐笔 vs 批量
    group.bench_function("bptree_insert_single", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_insert_batch", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for chunk in data_100k.chunks(100) {
                for (k, v) in chunk {
                    bpt.insert(*k, *v);
                }
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 4. 批量 vs 单笔
    group.bench_function("bptree_insert_bulk", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_insert_single_op", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 5. 单条插入/删除/查询/区间查询
    group.bench_function("bptree_single_insert", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            bpt.insert(1, 1);
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_single_delete", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            bpt.insert(1, 1);
            bpt.delete(&1);
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_single_query", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            bpt.insert(1, 1);
            black_box(bpt.get(&1));
        })
    });
    group.bench_function("bptree_single_range_query", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            bpt.insert(1, 1);
            black_box(bpt.range_query(&1, &10));
        })
    });

    // 6. 批量插入/删除/查询/区间查询
    group.bench_function("bptree_bulk_insert", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_bulk_delete", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            // 先插入所有数据
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            // 准备要删除的键
            let delete_keys: Vec<_> = data_100k.iter()
                .map(|(k, _)| *k)
                .collect();
            // 批量删除
            bpt.bulk_delete(&delete_keys);
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_bulk_query", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            for (k, _) in &data_100k {
                black_box(bpt.get(k));
            }
        })
    });
    group.bench_function("bptree_bulk_range_query", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            black_box(bpt.range_query(&1, &10));
        })
    });

    // 7. 区间插入/删除/查询/区间查询
    group.bench_function("bptree_range_insert", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in data_100k.iter().take(10) {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_range_delete", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in data_100k.iter().take(10) {
                bpt.insert(*k, *v);
            }
            for (k, _) in data_100k.iter().take(10) {
                bpt.delete(k);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });
    group.bench_function("bptree_range_query", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in data_100k.iter().take(10) {
                bpt.insert(*k, *v);
            }
            black_box(bpt.range_query(&1, &10));
        })
    });

    // 添加百万级数据测试
    let data_1m = generate_random_pairs(1_000_000);
    let data_5m = generate_random_pairs(5_000_000);

    // 百万级数据插入测试
    group.bench_function("bptree_insert_1m", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_1m {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    group.bench_function("bptree_insert_5m", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_5m {
                bpt.insert(*k, *v);
            }
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 百万级数据查询测试
    group.bench_function("bptree_query_1m", |b| {
        let mut bpt = BPTree::new(3);
        for (k, v) in &data_1m {
            bpt.insert(*k, *v);
        }
        let query_keys: Vec<_> = data_1m.iter().step_by(1000).map(|(k, _)| k).collect();
        
        b.iter(|| {
            for k in &query_keys {
                black_box(bpt.get(k));
            }
        })
    });

    // 百万级数据范围查询测试
    group.bench_function("bptree_range_query_1m", |b| {
        let mut bpt = BPTree::new(3);
        for (k, v) in &data_1m {
            bpt.insert(*k, *v);
        }
        
        b.iter(|| {
            black_box(bpt.range_query(&1, &50_000));
        })
    });

    // 百万级数据批量操作测试
    group.bench_function("bptree_bulk_ops_1m", |b| {
        let mut bpt = BPTree::new(3);
        
        b.iter(|| {
            // 插入100万条数据
            for (k, v) in &data_1m {
                bpt.insert(*k, *v);
            }
            
            // 随机查询1000条
            for (k, _) in data_1m.iter().step_by(1000) {
                black_box(bpt.get(k));
            }
            
            // 范围查询
            black_box(bpt.range_query(&1, &50_000));
            
            // 删除1000条
            for (k, _) in data_1m.iter().step_by(1000) {
                bpt.delete(k);
            }
            
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 测试不同 min_degree 对性能的影响
    for degree in [4, 8, 16, 32, 64] {
        group.bench_function(&format!("bptree_insert_1m_degree_{}", degree), |b| {
            b.iter(|| {
                let mut bpt = BPTree::new(degree);
                for (k, v) in &data_1m {
                    bpt.insert(*k, *v);
                }
                black_box(bpt.approximate_memory_usage());
            })
        });
    }

    // 添加大数据量的批量删除测试
    group.bench_function("bptree_bulk_delete_1m", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            // 插入100万数据
            for (k, v) in &data_1m {
                bpt.insert(*k, *v);
            }
            // 准备要删除的键
            let delete_keys: Vec<_> = data_1m.iter()
                .step_by(10)  // 每10个删除一个，减少测试时间
                .map(|(k, _)| *k)
                .collect();
            // 批量删除
            bpt.bulk_delete(&delete_keys);
            black_box(bpt.approximate_memory_usage());
        })
    });

    // 添加不同模式的批量删除测试
    group.bench_function("bptree_bulk_delete_random", |b| {
        b.iter(|| {
            let mut bpt = BPTree::new(3);
            for (k, v) in &data_100k {
                bpt.insert(*k, *v);
            }
            // 随机选择要删除的键
            let mut rng = rand::thread_rng();
            let delete_keys: Vec<_> = data_100k.iter()
                .filter(|_| rng.gen_bool(0.5))  // 随机选择50%的键删除
                .map(|(k, _)| *k)
                .collect();
            bpt.bulk_delete(&delete_keys);
            black_box(bpt.approximate_memory_usage());
        })
    });

    group.finish();
}

fn bench_rb_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("RBTree Insert/Find");
    group.sample_size(10).measurement_time(std::time::Duration::new(3, 0));

    let data_100k = generate_random_pairs(100_000);
    let data_50k = generate_random_pairs(50_000);

    // 0. 10w条以上的数据 vs 10w条以下的数据的性能对比
    group.bench_function("rb_tree_insert_100k", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_insert_50k", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_50k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    // 1. 高频 vs 低频
    group.bench_function("rb_tree_insert_high_freq", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_insert_low_freq", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter().step_by(10) {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    // 2. 小批量 vs 大批量
    group.bench_function("rb_tree_insert_small_batch", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter().take(100) {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_insert_large_batch", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    // 3. 逐笔交易 vs 批量交易
    group.bench_function("rb_tree_insert_single", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_insert_batch", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for chunk in data_100k.chunks(100) {
                for (k, v) in chunk {
                    tree.insert(*k, *v);
                }
            }
            black_box(tree);
        })
    });

    // 4. 批量 vs 单笔
    group.bench_function("rb_tree_insert_bulk", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_insert_single_op", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    // 5. 单条插入/删除/查询/区间查询的耗时对比
    group.bench_function("rb_tree_single_insert", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            tree.insert(1, 1);
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_single_delete", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            tree.insert(1, 1);
            tree.delete(&1);
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_single_query", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            tree.insert(1, 1);
            black_box(tree.find(&1));
        })
    });

    group.bench_function("rb_tree_single_range_query", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            tree.insert(1, 1);
            black_box(tree.range_query(&1, &10));
        })
    });

    // 6. 批量插入/删除/查询/区间查询的耗时对比
    group.bench_function("rb_tree_bulk_insert", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_bulk_delete", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            for (k, _) in data_100k.iter() {
                tree.delete(k);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_bulk_query", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            for (k, _) in data_100k.iter() {
                black_box(tree.find(k));
            }
        })
    });

    group.bench_function("rb_tree_bulk_range_query", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter() {
                tree.insert(*k, *v);
            }
            black_box(tree.range_query(&1, &10));
        })
    });

    // 7. 区间插入/删除/查询/区间查询的耗时对比
    group.bench_function("rb_tree_range_insert", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter().take(10) {
                tree.insert(*k, *v);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_range_delete", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter().take(10) {
                tree.insert(*k, *v);
            }
            for (k, _) in data_100k.iter().take(10) {
                tree.delete(k);
            }
            black_box(tree);
        })
    });

    group.bench_function("rb_tree_range_query", |b| {
        b.iter(|| {
            let mut tree = RBTree::new();
            for (k, v) in data_100k.iter().take(10) {
                tree.insert(*k, *v);
            }
            black_box(tree.range_query(&1, &10));
        })
    });

    group.finish();
}

fn bench_btree_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("BTreeMap Insert/Get");
    group.sample_size(10).measurement_time(std::time::Duration::new(3, 0));

    let data_100k = generate_random_pairs(100_000);
    let data_50k = generate_random_pairs(50_000);

    // 0. 10w条以上的数据 vs 10w条以下的数据的性能对比
    group.bench_function("btree_map_insert_100k", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_insert_50k", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_50k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    // 1. 高频 vs 低频
    group.bench_function("btree_map_insert_high_freq", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_insert_low_freq", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter().step_by(10) {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    // 2. 小批量 vs 大批量
    group.bench_function("btree_map_insert_small_batch", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter().take(100) {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_insert_large_batch", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    // 3. 逐笔交易 vs 批量交易
    group.bench_function("btree_map_insert_single", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_insert_batch", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for chunk in data_100k.chunks(100) {
                for (k, v) in chunk {
                    btree.insert(*k, *v);
                }
            }
            black_box(btree);
        })
    });

    // 4. 批量 vs 单笔
    group.bench_function("btree_map_insert_bulk", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_insert_single_op", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    // 5. 单条插入/删除/查询/区间查询的耗时对比
    group.bench_function("btree_map_single_insert", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            btree.insert(1, 1);
            black_box(btree);
        })
    });

    group.bench_function("btree_map_single_delete", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            btree.insert(1, 1);
            btree.delete(&1);
            black_box(btree);
        })
    });

    group.bench_function("btree_map_single_query", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            btree.insert(1, 1);
            black_box(btree.get(&1));
        })
    });

    group.bench_function("btree_map_single_range_query", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            btree.insert(1, 1);
            black_box(btree.range_query(&1, &10));
        })
    });

    // 6. 批量插入/删除/查询/区间查询的耗时对比
    group.bench_function("btree_map_bulk_insert", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_bulk_delete", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            for (k, _) in data_100k.iter() {
                btree.delete(k);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_bulk_query", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            for (k, _) in data_100k.iter() {
                black_box(btree.get(k));
            }
        })
    });

    group.bench_function("btree_map_bulk_range_query", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter() {
                btree.insert(*k, *v);
            }
            black_box(btree.range_query(&1, &10));
        })
    });

    // 7. 区间插入/删除/查询/区间查询的耗时对比
    group.bench_function("btree_map_range_insert", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter().take(10) {
                btree.insert(*k, *v);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_range_delete", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter().take(10) {
                btree.insert(*k, *v);
            }
            for (k, _) in data_100k.iter().take(10) {
                btree.delete(k);
            }
            black_box(btree);
        })
    });

    group.bench_function("btree_map_range_query", |b| {
        b.iter(|| {
            let mut btree = BTreeMap::new(3);
            for (k, v) in data_100k.iter().take(10) {
                btree.insert(*k, *v);
            }
            black_box(btree.range_query(&1, &10));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_b_plus_tree, bench_rb_tree, bench_btree_map);
// criterion_group!(benches, bench_rb_tree, bench_btree_map);
criterion_main!(benches);