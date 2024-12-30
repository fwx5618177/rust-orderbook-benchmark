#[cfg(test)]
mod tests {
    use crate::bptree::bptree::BPTree;

    #[test]
    fn test_bptree_insert_find() {
        let mut bpt = BPTree::new(3);
        bpt.insert(10, "ten".to_string());
        bpt.insert(5, "five".to_string());
        bpt.insert(15, "fifteen".to_string());

        assert_eq!(bpt.get(&10), Some("ten".to_string()));
        assert_eq!(bpt.get(&5), Some("five".to_string()));
        assert_eq!(bpt.get(&15), Some("fifteen".to_string()));
        assert_eq!(bpt.get(&999), None);
    }

    #[test]
    fn test_bptree_delete() {
        let mut bpt = BPTree::new(3);
        bpt.insert(20, "twenty".to_string());
        bpt.insert(10, "ten".to_string());
        bpt.insert(30, "thirty".to_string());
        bpt.insert(25, "twenty-five".to_string());
        bpt.insert(35, "thirty-five".to_string());

        bpt.delete(&999); // 不存在的key
        assert_eq!(bpt.get(&30), Some("thirty".to_string()));

        bpt.delete(&30);
        assert_eq!(bpt.get(&30), None);

        bpt.delete(&20);
        assert_eq!(bpt.get(&20), None);
        assert_eq!(bpt.get(&10), Some("ten".to_string()));
    }
}