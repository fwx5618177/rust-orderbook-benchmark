use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub color: Color,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V, color: Color) -> Self {
        Node {
            key,
            value,
            color,
            left: None,
            right: None,
        }
    }
}

pub struct RBTree<K, V> {
    pub root: Option<Box<Node<K, V>>>,
}

impl<K: Ord + Clone, V: Clone> RBTree<K, V> {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    /// 查找
    pub fn find(&self, key: &K) -> Option<&V> {
        let mut curr = &self.root;
        while let Some(node) = curr {
            match key.cmp(&node.key) {
                Ordering::Less => curr = &node.left,
                Ordering::Greater => curr = &node.right,
                Ordering::Equal => return Some(&node.value),
            }
        }
        None
    }

    /// 插入
    pub fn insert(&mut self, key: K, value: V) {
        self.root = Self::insert_node(self.root.take(), key, value);
        // 根节点必须是黑色
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    fn insert_node(node: Option<Box<Node<K, V>>>, key: K, value: V) -> Option<Box<Node<K, V>>> {
        // 标准 BST 插入
        let mut n = match node {
            None => {
                let new_node = Box::new(Node::new(key, value, Color::Red));
                return Some(new_node);
            }
            Some(mut n) => {
                if key < n.key {
                    n.left = Self::insert_node(n.left.take(), key, value);
                } else if key > n.key {
                    n.right = Self::insert_node(n.right.take(), key, value);
                } else {
                    // key 相等，更新 value
                    n.value = value;
                }
                n
            }
        };

        // 红黑树修正
        // 1. 右链接是红，左链接不是红 => 左旋
        if Self::is_red(&n.right) && !Self::is_red(&n.left) {
            n = Self::rotate_left(n);
        }
        // 2. 左链接是红，且左的左链接也是红 => 右旋
        if Self::is_red(&n.left) && Self::is_red(&n.left.as_ref().unwrap().left) {
            n = Self::rotate_right(n);
        }
        // 3. 左右链接都为红 => flip
        if Self::is_red(&n.left) && Self::is_red(&n.right) {
            Self::flip_colors(&mut n);
        }

        Some(n)
    }

    /// 外部删除接口
    pub fn delete(&mut self, key: &K) {
        if self.root.is_none() {
            return;
        }
        // 如果根的两个子节点都是黑色，将根设为红色（LLRB 逻辑）
        if !Self::is_red(&self.root.as_ref().unwrap().left)
            && !Self::is_red(&self.root.as_ref().unwrap().right)
        {
            if let Some(ref mut root) = self.root {
                root.color = Color::Red;
            }
        }

        self.root = Self::delete_node(self.root.take(), key);

        // 将根设为黑色
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    /// 内部删除逻辑 (Left-Leaning Red-Black Tree)
    fn delete_node(node: Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
        let mut h = node?;

        if key < &h.key {
            // key 在左子树
            if !Self::is_red(&h.left) {
                // 检查 left 和 left.left，若都不是红 => move_red_left
                if let Some(ref left_child) = h.left {
                    if !Self::is_red(&left_child.left) {
                        h = Self::move_red_left(h);
                    }
                }
            }
            h.left = Self::delete_node(h.left.take(), key);
        } else {
            // 如果左子是红 => 右旋
            if Self::is_red(&h.left) {
                h = Self::rotate_right(h);
            }
            // 找到 key 且右子为空 => 删除该节点
            if key == &h.key && h.right.is_none() {
                return None;
            }
            // 检查右子和右子的左子，若都不是红 => move_red_right
            if let Some(ref right_child) = h.right {
                if !Self::is_red(&h.right) && !Self::is_red(&right_child.left) {
                    h = Self::move_red_right(h);
                }
            }
            if key == &h.key {
                // 用右子中的最小节点替换
                if let Some(min_node) = Self::min(&h.right) {
                    h.key = min_node.key.clone();
                    h.value = min_node.value.clone();
                    h.right = Self::delete_min(h.right.take());
                }
            } else {
                h.right = Self::delete_node(h.right.take(), key);
            }
        }

        Some(Self::fix_up(h))
    }

    fn delete_min(node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let mut h = node?;
        if h.left.is_none() {
            return None;
        }
        if !Self::is_red(&h.left) {
            if let Some(ref left_child) = h.left {
                if !Self::is_red(&left_child.left) {
                    h = Self::move_red_left(h);
                }
            }
        }
        h.left = Self::delete_min(h.left.take());
        Some(Self::fix_up(h))
    }

    fn move_red_left(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        Self::flip_colors(&mut h);
        if let Some(ref r) = h.right {
            if Self::is_red(&r.left) {
                let right_box = h.right.take().unwrap();
                h.right = Some(Self::rotate_right(right_box));
                h = Self::rotate_left(h);
                Self::flip_colors(&mut h);
            }
        }
        h
    }

    fn move_red_right(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        Self::flip_colors(&mut h);
        if let Some(ref l) = h.left {
            if Self::is_red(&l.left) {
                h = Self::rotate_right(h);
                Self::flip_colors(&mut h);
            }
        }
        h
    }

    fn fix_up(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        // 如果右子是红 => 左旋
        if Self::is_red(&h.right) {
            h = Self::rotate_left(h);
        }
        // 如果左子是红且左子的左子也是红 => 右旋
        if Self::is_red(&h.left) && Self::is_red(&h.left.as_ref().unwrap().left) {
            h = Self::rotate_right(h);
        }
        // 如果左右子都是红 => flip
        if Self::is_red(&h.left) && Self::is_red(&h.right) {
            Self::flip_colors(&mut h);
        }
        h
    }

    fn rotate_left(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.right.take().unwrap();
        h.right = x.left.take();
        x.left = Some(h);
        x.color = x.left.as_ref().unwrap().color;
        x.left.as_mut().unwrap().color = Color::Red;
        x
    }

    fn rotate_right(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.left.take().unwrap();
        h.left = x.right.take();
        x.right = Some(h);
        x.color = x.right.as_ref().unwrap().color;
        x.right.as_mut().unwrap().color = Color::Red;
        x
    }

    fn flip_colors(h: &mut Box<Node<K, V>>) {
        h.color = match h.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
        if let Some(ref mut left) = h.left {
            left.color = match left.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
        if let Some(ref mut right) = h.right {
            right.color = match right.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
    }

    fn is_red(node: &Option<Box<Node<K, V>>>) -> bool {
        match node {
            Some(n) => n.color == Color::Red,
            None => false,
        }
    }

    fn min(node: &Option<Box<Node<K, V>>>) -> Option<&Box<Node<K, V>>> {
        match node {
            Some(n) => match &n.left {
                Some(_) => Self::min(&n.left),
                None => Some(n),
            },
            None => None,
        }
    }

    /// 简易的区间查询 (用于测试演示)
    pub fn range_query(&self, start: &K, end: &K) -> Vec<&V> {
        let mut result = Vec::new();
        self.range_query_node(&self.root, start, end, &mut result);
        result
    }

    fn range_query_node<'a>(
        &self,
        node: &'a Option<Box<Node<K, V>>>,
        start: &K,
        end: &K,
        result: &mut Vec<&'a V>,
    ) {
        if let Some(n) = node {
            if start < &n.key {
                self.range_query_node(&n.left, start, end, result);
            }
            if start <= &n.key && end >= &n.key {
                result.push(&n.value);
            }
            if end > &n.key {
                self.range_query_node(&n.right, start, end, result);
            }
        }
    }
}