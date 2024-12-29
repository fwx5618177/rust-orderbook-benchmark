#[cfg(test)]
mod tests {
    use crate::btree_map::btree_map::BTreeMap;

    #[test]
    fn test_btree_insert_get() {
        let mut btree = BTreeMap::new(3);
        btree.insert(10, "ten");
        btree.insert(5, "five");
        btree.insert(15, "fifteen");

        assert_eq!(btree.get(&10), Some(&"ten"));
        assert_eq!(btree.get(&5), Some(&"five"));
        assert_eq!(btree.get(&15), Some(&"fifteen"));
        assert_eq!(btree.get(&999), None);
    }
}