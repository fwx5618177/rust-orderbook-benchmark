use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::Rng;
use rust_orderbook_benchmark::rb_tree::rb_tree::RBTree;
use rust_orderbook_benchmark::btree_map::btree_map::BTreeMap;

fn generate_random_pairs(n: usize) -> Vec<(u32, u32)> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| (rng.gen_range(1..100_000), rng.gen_range(1..100_000)))
        .collect()
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

criterion_group!(benches, bench_rb_tree, bench_btree_map);
criterion_main!(benches);