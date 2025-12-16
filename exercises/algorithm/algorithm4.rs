/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => {
                node.insert(value);
            }
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match &self.root {
            Some(node) => node.search(value),
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 插入左子树
                match self.left {
                    Some(ref mut left) => left.insert(value),
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            Ordering::Greater => {
                // 插入右子树
                match self.right {
                    Some(ref mut right) => right.insert(value),
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            Ordering::Equal => {
                // 二叉搜索树通常不允许重复值
                // 根据测试要求，对于重复值不做任何操作
            }
        }
    }
    
    // Search for a value in the tree
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 在左子树中搜索
                match &self.left {
                    Some(left) => left.search(value),
                    None => false,
                }
            }
            Ordering::Greater => {
                // 在右子树中搜索
                match &self.right {
                    Some(right) => right.search(value),
                    None => false,
                }
            }
            Ordering::Equal => {
                // 找到值
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 初始为空树
        assert_eq!(bst.search(1), false);

        // 插入值
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 搜索存在的值
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 搜索不存在的值
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复值
        bst.insert(1);
        bst.insert(1);

        // 搜索应该成功
        assert_eq!(bst.search(1), true);

        // 检查树结构（应该只有一个节点）
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
