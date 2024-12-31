#![allow(non_snake_case)]

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
            keys: Vec::with_capacity(2 * min_degree - 1),  // 预分配容量
            vals: Vec::with_capacity(2 * min_degree - 1),
            children: Vec::new(),
            next_leaf: None,  // 叶子节点链表
        };
        BPTree {
            root: Box::new(leaf_node),
            min_degree,
        }
    }

    /// 粗略估算内存使用
    pub fn approximate_memory_usage(&self) -> usize {
        let mut total = 0;
        let mut stack = vec![&*self.root];
        
        while let Some(node) = stack.pop() {
            total += std::mem::size_of::<Node<K, V>>();
            total += node.keys.capacity() * std::mem::size_of::<K>();
            total += node.vals.capacity() * std::mem::size_of::<V>();
            
            for child in &node.children {
                stack.push(child);
            }
        }
        total
    }

    /// 插入 (key, value)
    pub fn insert(&mut self, key: K, value: V) {
        let root = &mut self.root;
        if Self::is_full(self.min_degree, root) {
            // 创建新的根节点
            let mut new_root = Box::new(Node {
                node_type: NodeType::Internal,
                keys: Vec::with_capacity(2 * self.min_degree),
                vals: Vec::new(),
                children: Vec::with_capacity(2 * self.min_degree + 1),
                next_leaf: None,
            });
            // 交换新旧根节点
            std::mem::swap(root, &mut new_root);
            // 将旧根节点作为新根节点的第一个子节点
            (*root).children.push(*new_root);
            // 分裂子节点
            Self::split_child(self.min_degree, root, 0);
        }
        
        // 直接在根节点上操作，避免克隆
        Self::insert_non_full(self.min_degree, root, key, value);
    }

    /// 查询
    pub fn get(&self, key: &K) -> Option<V> {
        let mut current = &*self.root;
        while current.node_type == NodeType::Internal {
            let i = match current.keys.binary_search(key) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            
            // 添加边界检查
            if i >= current.children.len() {
                if current.children.is_empty() {
                    return None;
                }
                current = &current.children[current.children.len() - 1];
            } else {
                current = &current.children[i];
            }
        }
        
        match current.keys.binary_search(key) {
            Ok(i) => Some(current.vals[i].clone()),
            Err(_) => None,
        }
    }

    /// 删除
    pub fn delete(&mut self, key: &K) {
        Self::delete_recur(&mut self.root, key, self.min_degree);
        
        // 如果根节点是内部节点且为空，提升其唯一的子节点为新根
        if self.root.node_type == NodeType::Internal && self.root.keys.is_empty() {
            if !self.root.children.is_empty() {
                let new_root = self.root.children.remove(0);
                self.root = Box::new(new_root);
            }
        }
    }

    /// 范围查询 [start..end] - B+树的独特优势
    pub fn range_query(&self, start: &K, end: &K) -> Vec<V> {
        let mut result = Vec::new();
        let mut current = &*self.root;
        
        // 1. 快速定位起始叶子节点
        while current.node_type == NodeType::Internal {
            let i = match current.keys.binary_search(start) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            current = &current.children[i];
        }
        
        // 2. 利用叶子节点链表进行高效的范围扫描
        loop {
            for (i, k) in current.keys.iter().enumerate() {
                if k >= start && k <= end {
                    result.push(current.vals[i].clone());
                }
                if k > end {
                    return result;
                }
            }
            
            match &current.next_leaf {
                Some(next) => current = next,
                None => break,
            }
        }
        result
    }

    /// 批量插入操作 - 针对大数据量优化
    pub fn bulk_insert(&mut self, mut pairs: Vec<(K, V)>) {
        // 1. 先排序，提高插入效率
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        
        // 2. 批量插入
        for (key, value) in pairs {
            // 检查根节点是否需要分裂
            if Self::is_full(self.min_degree, &self.root) {
                let mut new_root = Box::new(Node {
                    node_type: NodeType::Internal,
                    keys: Vec::with_capacity(2 * self.min_degree),
                    vals: Vec::new(),
                    children: Vec::with_capacity(2 * self.min_degree + 1),
                    next_leaf: None,
                });
                std::mem::swap(&mut self.root, &mut new_root);
                self.root.children.push(*new_root);
                Self::split_child(self.min_degree, &mut self.root, 0);
            }
            
            // 优化的插入路径
            let mut current = &mut *self.root;
            
            // 向下遍历到叶子节点
            while current.node_type == NodeType::Internal {
                let i = match current.keys.binary_search(&key) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                };
                
                // 如果子节点满了，先分裂
                if Self::is_full(self.min_degree, &current.children[i]) {
                    Self::split_child(self.min_degree, current, i);
                    // 分裂后重新确定插入位置
                    if key > current.keys[i] {
                        current = &mut current.children[i + 1];
                    } else {
                        current = &mut current.children[i];
                    }
                } else {
                    current = &mut current.children[i];
                }
            }
            
            // 在叶子节点插入
            let i = match current.keys.binary_search(&key) {
                Ok(i) => i,
                Err(i) => i,
            };
            current.keys.insert(i, key);
            current.vals.insert(i, value);
        }
    }

    /// 批量删除操作
    pub fn bulk_delete(&mut self, keys: &[K]) {
        for key in keys {
            self.delete(key);
            
            // 调整树高度 - 确保根节点非空
            while self.root.node_type == NodeType::Internal && 
                  self.root.keys.is_empty() && 
                  self.root.children.len() == 1 {  // 修改：明确检查只有一个子节点的情况
                let new_root = self.root.children.remove(0);
                self.root = Box::new(new_root);
            }
        }
    }

    // ------------------- 内部逻辑 -------------------

    /// 判断节点是否满
    fn is_full(min_degree: usize, node: &Node<K, V>) -> bool {
        match node.node_type {
            NodeType::Leaf => node.keys.len() >= 2 * min_degree - 1,
            NodeType::Internal => node.children.len() >= 2 * min_degree,
        }
    }

    /// 在非满节点插入
    fn insert_non_full(min_degree: usize, node: &mut Node<K, V>, key: K, value: V) {
        match node.node_type {
            NodeType::Leaf => {
                let i = match node.keys.binary_search(&key) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                node.keys.insert(i, key);
                node.vals.insert(i, value);
            }
            NodeType::Internal => {
                let mut i = match node.keys.binary_search(&key) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                };
                
                // 添加边界检查
                if i >= node.children.len() {
                    i = node.children.len() - 1;
                }
                
                // 安全检查子节点是否存在
                if !node.children.is_empty() {
                    if Self::is_full(min_degree, &node.children[i]) {
                        Self::split_child(min_degree, node, i);
                        if key > node.keys[i] {
                            i += 1;
                        }
                    }
                    Self::insert_non_full(min_degree, &mut node.children[i], key, value);
                } else {
                    // 如果没有子节点，创建一个新的叶子节点
                    let mut leaf = Node {
                        node_type: NodeType::Leaf,
                        keys: Vec::with_capacity(2 * min_degree - 1),
                        vals: Vec::with_capacity(2 * min_degree - 1),
                        children: Vec::new(),
                        next_leaf: None,
                    };
                    leaf.keys.push(key);
                    leaf.vals.push(value);
                    node.children.push(leaf);
                }
            }
        }
    }

    /// 分裂子节点 - 针对大数据量优化
    fn split_child(min_degree: usize, parent: &mut Node<K, V>, i: usize) {
        let t = min_degree;
        let child = &mut parent.children[i];
        
        let mut new_node = Node {
            node_type: child.node_type.clone(),
            keys: Vec::with_capacity(2 * t - 1),  // 预分配容量
            vals: Vec::with_capacity(2 * t - 1),
            children: Vec::with_capacity(if child.node_type == NodeType::Internal { 2 * t } else { 0 }),
            next_leaf: None,
        };

        if child.node_type == NodeType::Leaf {
            // 叶子节点：保持所有数据，维护链表
            let mid = t - 1;
            new_node.keys = child.keys.split_off(mid);
            new_node.vals = child.vals.split_off(mid);
            
            new_node.next_leaf = child.next_leaf.take();
            child.next_leaf = Some(Box::new(new_node.clone()));
            
            parent.keys.insert(i, new_node.keys[0].clone());
            parent.children.insert(i + 1, new_node);
        } else {
            // 内部节点：只存储索引键
            let mid = t - 1;
            let up_key = child.keys[mid].clone();
            new_node.keys = child.keys.split_off(mid + 1);
            child.keys.pop();
            new_node.children = child.children.split_off(mid + 1);
            
            parent.keys.insert(i, up_key);
            parent.children.insert(i + 1, new_node);
        }
    }

    /// 删除递归
    fn delete_recur(node: &mut Node<K, V>, key: &K, min_degree: usize) {
        match node.node_type {
            NodeType::Leaf => {
                if let Ok(pos) = node.keys.binary_search(key) {
                    node.keys.remove(pos);
                    node.vals.remove(pos);
                }
            }
            NodeType::Internal => {
                if node.children.is_empty() {
                    return;
                }

                let i = match node.keys.binary_search(key) {
                    Ok(i) => {
                        if i < node.keys.len() {
                            let has_left = i < node.children.len() && 
                                         node.children[i].keys.len() >= min_degree;
                            let has_right = i + 1 < node.children.len() && 
                                          node.children[i + 1].keys.len() >= min_degree;

                            if has_left {
                                if let Some(pred) = Self::get_predecessor(node, i) {
                                    node.keys[i] = pred.0.clone();
                                    Self::delete_recur(&mut node.children[i], &pred.0, min_degree);
                                }
                            } else if has_right {
                                if let Some(succ) = Self::get_successor(node, i) {
                                    node.keys[i] = succ.0.clone();
                                    Self::delete_recur(&mut node.children[i + 1], &succ.0, min_degree);
                                }
                            } else if i < node.children.len() - 1 {
                                Self::merge_child(node, i);
                                if !node.children.is_empty() {
                                    Self::delete_recur(&mut node.children[i], key, min_degree);
                                }
                            }
                        }
                        i
                    }
                    Err(i) => i,
                };

                if i < node.children.len() {
                    Self::delete_recur(&mut node.children[i], key, min_degree);
                    
                    if !node.children.is_empty() && 
                       i < node.children.len() && 
                       node.children[i].keys.len() < min_degree - 1 {
                        Self::fix_child_deficiency(node, i, min_degree);
                    }
                }
            }
        }
    }

    /// 修复节点不足的情况
    fn fix_child_deficiency(node: &mut Node<K, V>, idx: usize, min_degree: usize) {
        if node.children.is_empty() || idx >= node.children.len() {
            return;
        }

        let can_borrow_left = idx > 0 && node.children[idx - 1].keys.len() >= min_degree;
        let can_borrow_right = idx + 1 < node.children.len() && node.children[idx + 1].keys.len() >= min_degree;

        if can_borrow_left {
            Self::borrow_from_left(node, idx);
        } else if can_borrow_right {
            Self::borrow_from_right(node, idx);
        } else if idx > 0 {
            Self::merge_child(node, idx - 1);
        } else if idx + 1 < node.children.len() {
            Self::merge_child(node, idx);
        }
    }

    fn borrow_from_left(parent: &mut Node<K, V>, idx: usize) {
        // 首先检查边界条件
        if idx == 0 || idx - 1 >= parent.keys.len() {
            return;
        }
    
        let (left_half, right_half) = parent.children.split_at_mut(idx);
        let left = &mut left_half[idx - 1];
        let cur = &mut right_half[0];
        
        if left.node_type == NodeType::Leaf {
            // 确保左节点和当前节点都有键
            if !left.keys.is_empty() && !cur.keys.is_empty() {
                let borrow_k = left.keys.pop().unwrap();
                let borrow_v = left.vals.pop().unwrap();
                
                // 更新父节点的键，确保当前节点有键可用
                if idx - 1 < parent.keys.len() && !cur.keys.is_empty() {
                    parent.keys[idx - 1] = cur.keys[0].clone();
                }
                
                cur.keys.insert(0, borrow_k);
                cur.vals.insert(0, borrow_v);
            }
        } else {
            // 内部节点的情况
            if !left.keys.is_empty() && !cur.keys.is_empty() {
                let borrow_k = left.keys.pop().unwrap();
                
                if idx - 1 < parent.keys.len() {
                    let temp = parent.keys[idx - 1].clone();
                    parent.keys[idx - 1] = borrow_k;
                    cur.keys.insert(0, temp);
                }
                
                if !left.children.is_empty() {
                    let child = left.children.pop().unwrap();
                    cur.children.insert(0, child);
                }
            }
        }
    }
    
    fn borrow_from_right(parent: &mut Node<K, V>, idx: usize) {
        // 首先检查边界条件
        if idx >= parent.children.len() - 1 || idx >= parent.keys.len() {
            return;
        }

        let (left_half, right_half) = parent.children.split_at_mut(idx + 1);
        // cur 位于 left_half[idx], right 位于 right_half[0]
        let cur = &mut left_half[idx];
        let right = &mut right_half[0];
    
        if right.node_type == NodeType::Leaf {
            // 确保右节点有足够的键可以借用
            if !right.keys.is_empty() {
                let borrow_k = right.keys.remove(0);
                let borrow_v = right.vals.remove(0);
                
                // 更新父节点的键
                if idx < parent.keys.len() {
                    parent.keys[idx] = right.keys[0].clone();  // 使用右节点的新首键
                }
                
                cur.keys.push(borrow_k);
                cur.vals.push(borrow_v);
            }
        } else {
            // 内部节点的情况
            if !right.keys.is_empty() {
                let borrow_k = right.keys.remove(0);
                
                // 更新父节点的键
                if idx < parent.keys.len() {
                    let temp = parent.keys[idx].clone();
                    parent.keys[idx] = borrow_k;
                    cur.keys.push(temp);
                }
                
                // 如果有子节点需要移动
                if !right.children.is_empty() {
                    let child = right.children.remove(0);
                    cur.children.push(child);
                }
            }
        }
    }

    fn merge_child(parent: &mut Node<K, V>, idx: usize) {
        if parent.children.is_empty() || idx >= parent.children.len() - 1 {
            return;
        }

        let right = parent.children.remove(idx + 1);
        let left = &mut parent.children[idx];

        if left.node_type == NodeType::Leaf {
            left.keys.extend(right.keys);
            left.vals.extend(right.vals);
            left.next_leaf = right.next_leaf;
        } else if idx < parent.keys.len() {
            if let Some(up_key) = parent.keys.get(idx).cloned() {
                parent.keys.remove(idx);
                left.keys.push(up_key);
                left.keys.extend(right.keys);
                left.children.extend(right.children);
            }
        }
    }

    /// 获取前驱节点的键值对
    fn get_predecessor(node: &mut Node<K, V>, idx: usize) -> Option<(K, V)> {
        if idx >= node.children.len() {
            return None;
        }
        
        let mut current = &node.children[idx];
        while current.node_type == NodeType::Internal {
            if current.children.is_empty() {
                return None;
            }
            current = &current.children[current.children.len() - 1];
        }
        
        if current.keys.is_empty() {
            None
        } else {
            let last_idx = current.keys.len() - 1;
            Some((current.keys[last_idx].clone(), current.vals[last_idx].clone()))
        }
    }

    /// 获取后继节点的键值对
    fn get_successor(node: &mut Node<K, V>, idx: usize) -> Option<(K, V)> {
        if idx + 1 >= node.children.len() {
            return None;
        }
        
        let mut current = &node.children[idx + 1];
        while current.node_type == NodeType::Internal {
            if current.children.is_empty() {
                return None;
            }
            current = &current.children[0];
        }
        
        if current.keys.is_empty() {
            None
        } else {
            Some((current.keys[0].clone(), current.vals[0].clone()))
        }
    }
}
