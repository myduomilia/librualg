use std::cmp::Ordering;
use std::fmt::Display;

/// Binary Tree
/// ```
/// use librualg::binary_tree::BinaryTree;
///
/// let mut tree = BinaryTree::new();
/// tree.add(3);
/// tree.add(7);
/// tree.add(2);
/// tree.add(9);
/// tree.add(5);
///
/// assert_eq!(tree.get(&7), Some(&7));
/// assert_eq!(tree.get(&8), None);
/// tree.remove(&7);
/// assert_eq!(tree.get(&7), None);
/// assert_eq!(tree.get(&5), Some(&5));
/// ```

pub struct BinaryTree<T> where T: std::cmp::Ord + Clone + Display {
    head: Option<Box<Node<T>>>
}

impl <T> BinaryTree<T> where T: std::cmp::Ord + Clone + Display {
    pub fn new() -> Self {
        BinaryTree {
            head: None,
        }
    }
    pub fn add(&mut self, value: T) {
        match &mut self.head {
            Some(ref mut node) => {
                node.add(value);
            }
            None => {
                self.head = Some(Box::new(Node::new(value)));
            }
        }
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        match &self.head {
            Some(node) => {
                node.get(value)
            }
            None => None
        }
    }

    pub fn remove(&mut self, value: &T){
        if let Some(root) = self.head.take() {
            self.head = Node::remove(root, value);
        }
    }
}

pub struct Node <T> where T: std::cmp::Ord + Clone + Display {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl <T> Node<T> where T: std::cmp::Ord + Clone + Display {

    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None
        }
    }

    pub fn add(&mut self, value: T) {
        if value >= self.value {
            match &mut self.right {
                Some(node) => {
                    node.add(value.clone());
                }
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
        } else {
            match &mut self.left {
                Some(node) => {
                    node.add(value.clone());
                }
                None => {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
        }
    }
    pub fn get(&self, value: &T) -> Option<&T> {
        match self.value.cmp(value) {
            Ordering::Equal => {
                Some(&self.value)
            }
            Ordering::Greater => {
                match &self.left {
                    Some(node) => {
                        node.get(value)
                    }
                    None => {
                        None
                    }
                }
            }
            Ordering::Less => {
                match &self.right {
                    Some(node) => {
                        node.get(value)
                    }
                    None => {
                        None
                    }
                }
            }
        }
    }

    fn right_most_child(&mut self) -> Option<Box<Node<T>>> {
        match self.right {
            Some(ref mut right) =>  {
                if let Some(t) = right.right_most_child() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = std::mem::replace(&mut r.left, None);
                    }
                    r
                }
            },
            None => None
        }
    }

    pub fn remove(mut box_node: Box<Node<T>>, value: &T) -> Option<Box<Node<T>>> {
        if value < &box_node.value {
            if let Some(left) = box_node.left.take() {
                box_node.left = Self::remove(left, value);
            }
            return Some(box_node);
        }

        if value > &box_node.value {
            if let Some(right) = box_node.right.take() {
                box_node.right = Self::remove(right, value);
            }
            return Some(box_node);
        }

        match (box_node.left.take(), box_node.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(right)) => {
                if let Some(mut rightmost) = left.right_most_child() {
                    rightmost.left = Some(left);
                    rightmost.right = Some(right);
                    Some(rightmost)
                } else {
                    left.right = Some(right);
                    Some(left)
                }
            },
        }
    }
}

#[test]
fn test() {
    let mut tree = BinaryTree::new();
    tree.add(3);
    tree.add(7);
    tree.add(2);

    assert_eq!(tree.get(&7), Some(&7));
    assert_eq!(tree.get(&8), None);
}

#[test]
fn test_remove_root() {
    let mut tree = BinaryTree::new();
    tree.add(1);
    assert_eq!(tree.get(&1), Some(&1));
    tree.remove(&1);
    assert_eq!(tree.get(&1), None);
    tree.add(2);
    assert_eq!(tree.get(&2), Some(&2));
}

#[test]
fn test_remove_leaf() {
    let mut tree = BinaryTree::new();
    tree.add(5);
    tree.add(7);
    tree.add(2);
    tree.add(6);
    assert_eq!(tree.get(&6), Some(&6));
    tree.remove(&6);
    assert_eq!(tree.get(&6), None);
    tree.remove(&2);
    assert_eq!(tree.get(&2), None);
}

#[test]
fn test_remove() {
    let mut tree = BinaryTree::new();
    tree.add(4);
    tree.add(7);
    tree.add(2);
    tree.add(6);
    tree.add(5);
    tree.add(9);

    tree.remove(&7);
    assert_eq!(tree.get(&5), Some(&5));
    assert_eq!(tree.get(&9), Some(&9));
}

#[test]
fn test_remove_exist_two_child(){
    let mut tree = BinaryTree::new();
    tree.add(3);
    tree.add(7);
    tree.add(2);
    tree.add(9);
    tree.add(5);
    assert_eq!(tree.get(&7), Some(&7));
    assert_eq!(tree.get(&8), None);
    tree.remove(&7);
    assert_eq!(tree.get(&7), None);
    assert_eq!(tree.get(&5), Some(&5));
    assert_eq!(tree.get(&9), Some(&9));
}