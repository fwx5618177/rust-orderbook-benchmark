use std::fmt::Debug;

/// 节点类型 (内部节点 or 叶子节点)
#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Internal,
    Leaf,
}

/// B+树节点
#[derive(Clone, Debug)]
pub struct Node<K: Ord + Clone + Debug, V: Clone + Debug> {
    /// 节点类型
    pub node_type: NodeType,
    /// 对 Leaf 节点：存 (key, val)
    /// 对 Internal 节点：只存 key (vals 对内部节点无实际用处)
    pub keys: Vec<K>,
    pub vals: Vec<V>,  // 仅在 Leaf 下使用
    /// 对 Internal 节点： children.len() = keys.len() + 1
    /// 对 Leaf 节点： children 为空
    pub children: Vec<Node<K, V>>,
    /// 叶子节点链表，用于 B+树范围查询优化
    pub next_leaf: Option<Box<Node<K, V>>>,
}

/// B+Tree 结构
#[derive(Clone, Debug)]
pub struct BPTree<K: Ord + Clone + Debug, V: Clone + Debug> {
    pub root: Box<Node<K, V>>,
    pub min_degree: usize,
}

impl<K: Ord + Clone + Debug, V: Clone + Debug> BPTree<K, V> {
    /// 创建 B+Tree, 初始只有一个叶子节点
    pub fn new(min_degree: usize) -> Self {
        assert!(min_degree >= 2, "B+Tree min_degree must >= 2");
        let leaf_node = Node {
            node_type: NodeType::Leaf,
            keys: Vec::new(),
            vals: Vec::new(),
            children: Vec::new(),
            next_leaf: None,
        };
        BPTree {
            root: Box::new(leaf_node),
            min_degree,
        }
    }

    /// 粗略估算整棵树的内存大小
    pub fn approximate_memory_usage(&self) -> usize {
        self.node_mem_usage(&self.root)
    }

    fn node_mem_usage(&self, node: &Node<K, V>) -> usize {
        let base_size = std::mem::size_of::<Node<K, V>>();
        let keys_size = node.keys.len() * std::mem::size_of::<K>();
        let vals_size = node.vals.len() * std::mem::size_of::<V>();
        let mut sum = base_size + keys_size + vals_size;
        for child in &node.children {
            sum += self.node_mem_usage(child);
        }
        if let Some(ref next) = node.next_leaf {
            // 如不想重复统计叶链，可注释此行
            sum += self.node_mem_usage(next);
        }
        sum
    }

    /// 插入 (key, value)
    pub fn insert(&mut self, key: K, value: V) {
        // 如果 root 满 => 分裂
        if self.is_full(&self.root) {
            let mut new_root = Node {
                node_type: NodeType::Internal,
                keys: Vec::new(),
                vals: Vec::new(),
                children: vec![(*self.root).clone()],
                next_leaf: None,
            };
            self.split_child(&mut new_root, 0);
            self.root = Box::new(new_root);
        }
        // 再插入
        let mut root_clone = (*self.root).clone();
        self.insert_non_full(&mut root_clone, key, value);
        self.root = Box::new(root_clone);
    }

    /// 查询
    pub fn get(&self, key: &K) -> Option<V> {
        self.search_node(&self.root, key)
    }

    /// 删除
    pub fn delete(&mut self, key: &K) {
        let mut root_clone = (*self.root).clone();
        self.delete_recur(&mut root_clone, key);

        // root 如果空并且是 Internal，就提升 child[0]
        if root_clone.node_type == NodeType::Internal && root_clone.keys.is_empty() {
            if !root_clone.children.is_empty() {
                root_clone = root_clone.children.remove(0);
            }
        }
        self.root = Box::new(root_clone);
    }

    /// 范围查询 [start..end]
    pub fn range_query(&self, start: &K, end: &K) -> Vec<V> {
        let mut result = Vec::new();
        self.range_query_recur(&self.root, start, end, &mut result);
        result
    }

    // ------------------- 内部逻辑 -------------------

    /// 判断节点是否满
    fn is_full(&self, node: &Node<K, V>) -> bool {
        match node.node_type {
            NodeType::Leaf => node.keys.len() >= 2 * self.min_degree - 1,
            NodeType::Internal => node.children.len() >= 2 * self.min_degree,
        }
    }

    /// 在非满节点插入
    fn insert_non_full(&mut self, node: &mut Node<K, V>, key: K, value: V) {
        match node.node_type {
            NodeType::Leaf => {
                let mut i = node.keys.len();
                while i > 0 && key < node.keys[i - 1] {
                    i -= 1;
                }
                node.keys.insert(i, key);
                node.vals.insert(i, value);
            }
            NodeType::Internal => {
                let mut i = node.keys.len();
                while i > 0 && key < node.keys[i - 1] {
                    i -= 1;
                }
                // ---- clamp i if needed ----
                if i >= node.children.len() {
                    i = node.children.len() - 1;
                }
    
                // check if full
                if self.is_full(&node.children[i]) {
                    self.split_child(node, i);
                    if i < node.keys.len() && key > node.keys[i] {
                        i += 1;
                    }
                    if i >= node.children.len() {
                        i = node.children.len() - 1;
                    }
                }
                self.insert_non_full(&mut node.children[i], key, value);
            }
        }
    }

    /// 分裂 node.children[i]
    fn split_child(&mut self, parent: &mut Node<K, V>, i: usize) {
        let t = self.min_degree;
        let mut child = parent.children[i].clone();

        let mut new_node = Node {
            node_type: child.node_type.clone(),
            keys: Vec::new(),
            vals: Vec::new(),
            children: Vec::new(),
            next_leaf: None,
        };

        if child.node_type == NodeType::Leaf {
            // 叶子节点分裂
            let mid = t - 1; // key数量 = 2t-1
            new_node.keys = child.keys.split_off(mid);
            new_node.vals = child.vals.split_off(mid);

            // 叶子链表
            new_node.next_leaf = child.next_leaf.take();
            child.next_leaf = Some(Box::new(new_node.clone()));

            let up_key = new_node.keys[0].clone();
            parent.keys.insert(i, up_key);
            parent.children.insert(i + 1, new_node);

        } else {
            // Internal 分裂
            let mid = t - 1;
            let up_key = child.keys[mid].clone();

            new_node.node_type = NodeType::Internal;
            new_node.keys = child.keys.split_off(mid + 1);
            if !child.vals.is_empty() {
                child.vals.truncate(mid + 1);
                new_node.vals = child.vals.split_off(mid + 1);
            }
            new_node.children = child.children.split_off(mid + 1);

            parent.keys.insert(i, up_key);
            parent.children.insert(i + 1, new_node);
        }
        parent.children[i] = child;
    }

    /// 查找 key
    fn search_node(&self, node: &Node<K, V>, key: &K) -> Option<V> {
        match node.node_type {
            NodeType::Leaf => {
                for (i, k) in node.keys.iter().enumerate() {
                    if k == key {
                        return Some(node.vals[i].clone());
                    }
                }
                None
            }
            NodeType::Internal => {
                let mut i = 0;
                while i < node.keys.len() && key > &node.keys[i] {
                    i += 1;
                }
                self.search_node(&node.children[i], key)
            }
        }
    }

    /// 删除递归
    fn delete_recur(&mut self, node: &mut Node<K, V>, key: &K) {
        match node.node_type {
            NodeType::Leaf => {
                if let Ok(pos) = node.keys.binary_search(key) {
                    node.keys.remove(pos);
                    node.vals.remove(pos);
                }
            }
            NodeType::Internal => {
                // 找 child
                let mut i = 0;
                while i < node.keys.len() && key > &node.keys[i] {
                    i += 1;
                }
                self.delete_recur(&mut node.children[i], key);
                if !self.child_sufficient(&node.children[i]) {
                    self.fix_after_delete(node, i);
                }
            }
        }
    }

    fn child_sufficient(&self, node: &Node<K, V>) -> bool {
        // Leaf: keys.len >= t-1
        // Internal: children.len >= t
        match node.node_type {
            NodeType::Leaf => node.keys.len() >= self.min_degree - 1,
            NodeType::Internal => node.children.len() >= self.min_degree,
        }
    }

    fn fix_after_delete(&mut self, node: &mut Node<K, V>, idx: usize) {
        if idx > 0 {
            // 检查左兄弟
            let left_ok = self.child_sufficient(&node.children[idx - 1]);
            if left_ok {
                self.borrow_from_left(node, idx);
            } else {
                self.merge_child(node, idx - 1);
            }
        } else if idx + 1 < node.children.len() {
            // 检查右兄弟
            let right_ok = self.child_sufficient(&node.children[idx + 1]);
            if right_ok {
                self.borrow_from_right(node, idx);
            } else {
                self.merge_child(node, idx);
            }
        }
    }

    fn borrow_from_left(&mut self, parent: &mut Node<K, V>, idx: usize) {
        let (left_half, right_half) = parent.children.split_at_mut(idx);
        // left 位于 left_half[idx-1], cur 位于 right_half[0]
        let left = &mut left_half[idx - 1];
        let cur = &mut right_half[0];
        
        if left.node_type == NodeType::Leaf {
            let borrow_k = left.keys.pop().unwrap();
            let borrow_v = left.vals.pop().unwrap();
            parent.keys[idx - 1] = borrow_k.clone();
            cur.keys.insert(0, borrow_k);
            cur.vals.insert(0, borrow_v);
        } else {
            let borrow_k = left.keys.pop().unwrap();
            parent.keys[idx - 1] = borrow_k.clone();
            cur.keys.insert(0, borrow_k);
        }
    }
    
    fn borrow_from_right(&mut self, parent: &mut Node<K, V>, idx: usize) {
        let (left_half, right_half) = parent.children.split_at_mut(idx + 1);
        // cur 位于 left_half[idx], right 位于 right_half[0]
        let cur = &mut left_half[idx];
        let right = &mut right_half[0];
    
        if right.node_type == NodeType::Leaf {
            let borrow_k = right.keys.remove(0);
            let borrow_v = right.vals.remove(0);
            parent.keys[idx] = borrow_k.clone();
            cur.keys.push(borrow_k);
            cur.vals.push(borrow_v);
        } else {
            let borrow_k = right.keys.remove(0);
            parent.keys[idx] = borrow_k.clone();
            cur.keys.push(borrow_k);
        }
    }

    fn merge_child(&mut self, parent: &mut Node<K, V>, idx: usize) {
        // merge children[idx] & children[idx+1]
        let mut right_node = parent.children.remove(idx + 1);
        let left_node = &mut parent.children[idx];

        if left_node.node_type == NodeType::Leaf {
            left_node.keys.extend_from_slice(&right_node.keys);
            left_node.vals.extend_from_slice(&right_node.vals);
            left_node.next_leaf = right_node.next_leaf.take();
        } else {
            let up_key = parent.keys.remove(idx);
            left_node.keys.push(up_key);
            left_node.keys.extend_from_slice(&right_node.keys);
            left_node.vals.extend_from_slice(&right_node.vals);
            left_node.children.extend_from_slice(&right_node.children);
        }
    }

    /// 范围查询
    fn range_query_recur(&self, node: &Node<K, V>, start: &K, end: &K, output: &mut Vec<V>) {
        match node.node_type {
            NodeType::Leaf => {
                // 直接扫描 (k, v)
                for (i, k) in node.keys.iter().enumerate() {
                    if k >= start && k <= end {
                        output.push(node.vals[i].clone());
                    }
                }
            }
            NodeType::Internal => {
                // 在内部节点上，需遍历 children[0..=keys.len()]
                // 先找所有 <start
                let mut i = 0;
                while i < node.keys.len() && &node.keys[i] < start {
                    self.range_query_recur(&node.children[i], start, end, output);
                    i += 1;
                }
                // 再找 [start..end]
                while i < node.keys.len() && &node.keys[i] <= end {
                    self.range_query_recur(&node.children[i], start, end, output);
                    i += 1;
                }
                // 处理最后一个 child
                if i < node.children.len() {
                    self.range_query_recur(&node.children[i], start, end, output);
                }
            }
        }
    }
}