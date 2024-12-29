use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct BTreeMap<K: Ord + Clone + Debug, V: Clone + Debug> {
    root: Option<BTreeNode<K, V>>,
    min_degree: usize,
}

#[derive(Debug, Clone)]
struct BTreeNode<K: Ord + Clone + Debug, V: Clone + Debug> {
    keys: Vec<K>,
    vals: Vec<V>,
    children: Vec<Option<BTreeNode<K, V>>>,
    leaf: bool,
}

impl<K: Ord + Clone + Debug, V: Clone + Debug> BTreeNode<K, V> {
    fn new(leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            vals: Vec::new(),
            children: Vec::new(),
            leaf,
        }
    }
}

impl<K: Ord + Clone + Debug, V: Clone + Debug> BTreeMap<K, V> {
    pub fn new(min_degree: usize) -> Self {
        assert!(min_degree >= 2, "[BTreeMap] min_degree must be >= 2");
        BTreeMap {
            root: Some(BTreeNode::new(true)),
            min_degree,
        }
    }

    /// 调试日志
    fn debug_log(&self, _msg: &str) {
        // println!("[BTreeMap DEBUG] {}", msg);
    }

    /// 插入 key-value
    pub fn insert(&mut self, key: K, value: V) {
        self.debug_log(&format!("Insert key={:?} val={:?}", key, value));
        if self.root.is_none() {
            self.root = Some(BTreeNode::new(true));
        }

        // 如果 root 满了，需要先分裂
        let root_full = {
            let root_node = self.root.as_ref().unwrap();
            root_node.keys.len() == 2 * self.min_degree - 1
        };

        if root_full {
            // 创建新 root
            let mut new_root = BTreeNode::new(false);
            new_root.children.push(self.root.take());
            self.split_child(&mut new_root, 0);
            self.insert_non_full(&mut new_root, key, value);
            self.root = Some(new_root);
        } else {
            // 直接插入
            if let Some(ref mut _root_node) = self.root {
                let mut root_node = self.root.take().unwrap();
                self.insert_non_full(&mut root_node, key, value);
                self.root = Some(root_node);
            }
        }
    }

    fn insert_non_full(&mut self, node: &mut BTreeNode<K, V>, key: K, value: V) {
        let mut i = node.keys.len();
        if node.leaf {
            // 在叶子节点插入
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            node.keys.insert(i, key);
            node.vals.insert(i, value);
        } else {
            // 在内部节点插入
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            // 若子节点已满 -> 分裂
            if node.children[i].as_ref().unwrap().keys.len() == 2 * self.min_degree - 1 {
                self.split_child(node, i);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            self.insert_non_full(node.children[i].as_mut().unwrap(), key, value);
        }
    }

    /// 分裂 children[i]
    fn split_child(&mut self, parent: &mut BTreeNode<K, V>, i: usize) {
        self.debug_log(&format!("split_child i={}", i));
        let min_degree = self.min_degree;
        let child = parent.children[i].take().unwrap();

        let mut new_node = BTreeNode::new(child.leaf);
        // Middle index
        let mid_idx = min_degree - 1;

        // 上提
        let up_key = child.keys[mid_idx].clone();
        let up_val = child.vals[mid_idx].clone();

        // new_node 拿 child 后半部分
        new_node.keys.extend(child.keys[mid_idx + 1..].iter().cloned());
        new_node.vals.extend(child.vals[mid_idx + 1..].iter().cloned());

        // child 截断
        let mut left_node = child;
        left_node.keys.truncate(mid_idx);
        left_node.vals.truncate(mid_idx);

        if !left_node.leaf {
            // 拿 children
            let split_children = left_node.children.split_off(min_degree);
            new_node.children = split_children;
        }

        // 在 parent 插入 up_key/up_val
        parent.keys.insert(i, up_key);
        parent.vals.insert(i, up_val);

        parent.children.insert(i + 1, Some(new_node));
        parent.children[i] = Some(left_node);
    }

    /// 查询
    pub fn get(&self, key: &K) -> Option<&V> {
        self.debug_log(&format!("get key={:?}", key));
        if let Some(ref root) = self.root {
            return self.search(root, key);
        }
        None
    }

    fn search<'a>(&'a self, node: &'a BTreeNode<K, V>, key: &K) -> Option<&'a V> {
        let mut i = 0;
        while i < node.keys.len() && key > &node.keys[i] {
            i += 1;
        }
        if i < node.keys.len() && &node.keys[i] == key {
            Some(&node.vals[i])
        } else if node.leaf {
            None
        } else {
            self.search(node.children[i].as_ref().unwrap(), key)
        }
    }

    // 删除
    pub fn delete(&mut self, key: &K) {
        self.debug_log(&format!("delete key={:?}", key));
        if self.root.is_none() {
            return;
        }

        let mut root = self.root.take().unwrap();
        self.delete_node(&mut root, key);
        self.root = Some(root);

        // 若 root 空 & 非叶
        if let Some(root) = self.root.as_ref() {
            if root.keys.is_empty() && !root.leaf {
                let child = root.children[0].clone();
                self.root = child;
            }
        }
    }

    fn delete_node(&mut self, node: &mut BTreeNode<K, V>, key: &K) {
        let idx = match node.keys.binary_search(key) {
            Ok(i) => i,
            Err(i) => i,
        };

        // key 在本节点
        if idx < node.keys.len() && &node.keys[idx] == key {
            if node.leaf {
                // 叶子节点，直接删除
                node.keys.remove(idx);
                node.vals.remove(idx);
            } else {
                // 内部节点
                let left_len = node.children[idx].as_ref().unwrap().keys.len();
                let right_len = node.children[idx + 1].as_ref().unwrap().keys.len();

                // 前驱
                if left_len >= self.min_degree {
                    let (pk, pv) = self.get_predecessor(node.children[idx].as_mut().unwrap());
                    node.keys[idx] = pk;
                    node.vals[idx] = pv;
                    self.delete_node(node.children[idx].as_mut().unwrap(), &node.keys[idx]);
                }
                // 后继
                else if right_len >= self.min_degree {
                    let (sk, sv) = self.get_successor(node.children[idx + 1].as_mut().unwrap());
                    node.keys[idx] = sk;
                    node.vals[idx] = sv;
                    self.delete_node(node.children[idx + 1].as_mut().unwrap(), &node.keys[idx]);
                }
                // merge
                else {
                    self.merge(node, idx);
                    self.delete_node(node.children[idx].as_mut().unwrap(), key);
                }
            }
        } else if !node.leaf {
            // key 不在本节点
            if idx >= node.children.len() {
                return;
            }

            // 下探前，若子节点不够，则fill
            if node.children[idx].as_ref().unwrap().keys.len() < self.min_degree {
                self.fill(node, idx);
            }
            let c_len = node.children.len();
            if idx >= c_len {
                self.delete_node(node.children[idx - 1].as_mut().unwrap(), key);
            } else {
                self.delete_node(node.children[idx].as_mut().unwrap(), key);
            }
        }
    }

    fn get_predecessor(&self, node: &mut BTreeNode<K, V>) -> (K, V) {
        let mut cur = node;
        while !cur.leaf {
            let last_idx = cur.keys.len();
            cur = cur.children[last_idx].as_mut().unwrap();
        }
        let idx = cur.keys.len() - 1;
        (cur.keys[idx].clone(), cur.vals[idx].clone())
    }

    fn get_successor(&self, node: &mut BTreeNode<K, V>) -> (K, V) {
        let mut cur = node;
        while !cur.leaf {
            cur = cur.children[0].as_mut().unwrap();
        }
        (cur.keys[0].clone(), cur.vals[0].clone())
    }

    fn merge(&mut self, node: &mut BTreeNode<K, V>, idx: usize) {
        let key = node.keys.remove(idx);
        let val = node.vals.remove(idx);

        let left_child = node.children[idx].take().unwrap();
        let right_child = node.children.remove(idx + 1).take().unwrap();

        let mut merged = left_child;
        merged.keys.push(key);
        merged.vals.push(val);

        merged.keys.extend(right_child.keys);
        merged.vals.extend(right_child.vals);

        if !right_child.leaf {
            merged.children.extend(right_child.children);
        }
        node.children[idx] = Some(merged);
    }

    fn fill(&mut self, node: &mut BTreeNode<K, V>, idx: usize) {
        if idx > 0 && node.children[idx - 1].as_ref().unwrap().keys.len() >= self.min_degree {
            self.borrow_from_prev(node, idx);
        } else if idx + 1 < node.children.len()
            && node.children[idx + 1].as_ref().unwrap().keys.len() >= self.min_degree
        {
            self.borrow_from_next(node, idx);
        } else {
            let merge_idx = if idx < node.keys.len() { idx } else { idx - 1 };
            self.merge(node, merge_idx);
        }
    }

    fn borrow_from_prev(&mut self, node: &mut BTreeNode<K, V>, idx: usize) {
        let (sibling_key, sibling_val, sibling_child) = {
            let sibling = node.children[idx - 1].as_mut().unwrap();
            let s_last = sibling.keys.len() - 1;
            let key = sibling.keys.remove(s_last);
            let val = sibling.vals.remove(s_last);
            let child = if !sibling.leaf {
                sibling.children.remove(s_last + 1)
            } else {
                None
            };
            (key, val, child)
        };

        let child = node.children[idx].as_mut().unwrap();
        child.keys.insert(0, node.keys[idx - 1].clone());
        child.vals.insert(0, node.vals[idx - 1].clone());
        if let Some(c) = sibling_child {
            child.children.insert(0, Some(c));
        }
        node.keys[idx - 1] = sibling_key;
        node.vals[idx - 1] = sibling_val;
    }

    fn borrow_from_next(&mut self, node: &mut BTreeNode<K, V>, idx: usize) {
        let (sibling_key, sibling_val, sibling_child) = {
            let sibling = node.children[idx + 1].as_mut().unwrap();
            let key = sibling.keys.remove(0);
            let val = sibling.vals.remove(0);
            let child = if !sibling.leaf {
                sibling.children.remove(0)
            } else {
                None
            };
            (key, val, child)
        };

        let child = node.children[idx].as_mut().unwrap();
        child.keys.push(node.keys[idx].clone());
        child.vals.push(node.vals[idx].clone());
        if let Some(c) = sibling_child {
            child.children.push(Some(c));
        }
        node.keys[idx] = sibling_key;
        node.vals[idx] = sibling_val;
    }

    /// 区间查询
    pub fn range_query(&self, start: &K, end: &K) -> Vec<&V> {
        let mut result = Vec::new();
        if let Some(ref root_node) = self.root {
            self.range_query_node(root_node, start, end, &mut result);
        }
        result
    }

    fn range_query_node<'a>(
        &'a self,
        node: &'a BTreeNode<K, V>,
        start: &K,
        end: &K,
        output: &mut Vec<&'a V>,
    ) {
        let mut i = 0;
        // 跳过小于 start 的
        while i < node.keys.len() && &node.keys[i] < start {
            if !node.leaf {
                self.range_query_node(node.children[i].as_ref().unwrap(), start, end, output);
            }
            i += 1;
        }
        // 收集 [start, end] 范围内
        while i < node.keys.len() && &node.keys[i] <= end {
            if !node.leaf {
                self.range_query_node(node.children[i].as_ref().unwrap(), start, end, output);
            }
            output.push(&node.vals[i]);
            i += 1;
        }
        // 继续处理右子
        if !node.leaf && i < node.children.len() {
            self.range_query_node(node.children[i].as_ref().unwrap(), start, end, output);
        }
    }
}