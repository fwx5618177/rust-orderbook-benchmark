use crate::rb_tree::rb_tree::RBTree;
use crate::btree_map::btree_map::BTreeMap;
use rand::Rng;

pub fn simple_test_rb_tree() {
    let mut tree = RBTree::new();
    tree.insert(10, "ten");
    tree.insert(5, "five");
    tree.insert(15, "fifteen");

    println!("RBTree find(5) => {:?}", tree.find(&5));
    println!("RBTree find(15) => {:?}", tree.find(&15));
    println!("RBTree find(999) => {:?}", tree.find(&999));
}

pub fn simple_test_btree() {
    let mut btree = BTreeMap::new(3); // 最小度 = 3
    btree.insert(10, "ten");
    btree.insert(5, "five");
    btree.insert(15, "fifteen");

    println!("BTreeMap find(5) => {:?}", btree.get(&5));
    println!("BTreeMap find(15) => {:?}", btree.get(&15));
    println!("BTreeMap find(999) => {:?}", btree.get(&999));
}


pub fn generate_random_pairs(n: usize) -> Vec<(u32, u32)> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| (rng.gen_range(1..100_000), rng.gen_range(1..100_000)))
        .collect()
}
