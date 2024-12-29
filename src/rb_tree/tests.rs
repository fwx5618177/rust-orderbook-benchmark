#[cfg(test)]
mod tests {
    use crate::rb_tree::rb_tree::RBTree;

    #[test]
    fn test_rb_tree_insert_find() {
        let mut tree = RBTree::new();
        tree.insert(10, "ten");
        tree.insert(5, "five");
        tree.insert(15, "fifteen");

        assert_eq!(tree.find(&10), Some(&"ten"));
        assert_eq!(tree.find(&5), Some(&"five"));
        assert_eq!(tree.find(&15), Some(&"fifteen"));
        assert_eq!(tree.find(&999), None);
    }
}