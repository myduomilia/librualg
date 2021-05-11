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
#[derive(Clone)]
pub enum BinaryTree<T> where T: std::cmp::Ord + Clone + Display {
    Empty,
    NonEmpty(Box<Node<T>>)
}

#[derive(Clone)]
pub struct Node<T> where T: std::cmp::Ord + Clone + Display{
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}
impl <T> BinaryTree<T> where T: std::cmp::Ord + Clone +Display {
    pub fn new() -> Self {
        BinaryTree::Empty
    }
    pub fn add(&mut self, value: T) {
        match self {
            BinaryTree::NonEmpty(ref mut tree) => {
                if value > tree.value {
                    tree.right.add(value);
                } else {
                    tree.left.add(value);
                }
            }
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(Node {
                    value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty
                }))
            }
        }
    }
    pub fn get(&self, value: &T) -> Option<&T> {
        return match self {
            BinaryTree::NonEmpty(ref tree) => {
                match value.cmp(&tree.value) {
                    Ordering::Equal => {
                        Some(&tree.value)
                    }
                    Ordering::Greater => {
                        tree.right.get(value)
                    }
                    Ordering::Less => {
                        tree.left.get(value)
                    }
                }
            }
            BinaryTree::Empty => {
                None
            }
        }
    }

    pub fn remove(&mut self, value: &T) -> Option<T> {
        return match self {
            BinaryTree::NonEmpty(ref mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Equal => {
                        match (&mut node.left, &mut node.right) {
                            (BinaryTree::Empty, BinaryTree::Empty) => {
                                let elem = node.value.clone();
                                *self = BinaryTree::Empty;
                                Some(elem)
                            }
                            (BinaryTree::NonEmpty(left), BinaryTree::Empty) => {
                                let elem = node.value.clone();
                                *self = BinaryTree::NonEmpty(left.clone());
                                Some(elem)
                            }
                            (BinaryTree::Empty, BinaryTree::NonEmpty(right)) => {
                                let elem = node.value.clone();
                                *self = BinaryTree::NonEmpty(right.clone());
                                Some(elem)
                            }
                            // (BinaryTree::NonEmpty(_), BinaryTree::NonEmpty(_)) => {
                            //     let mut maximux = &mut node.left;
                            //     while let BinaryTree::NonEmpty(ref mut node) = maximux {
                            //         // if let BinaryTree::Empty = node.right {
                            //         //     break;
                            //         // }
                            //         maximux = &mut node.right;
                            //     }
                            //     let elem = node.value.clone();
                            //
                            //     if let BinaryTree::NonEmpty(ref mut value) = maximux {
                            //         node.value = value.value.clone();
                            //         match (&value.left, &value.right) {
                            //             (BinaryTree::Empty, BinaryTree::Empty) => {
                            //                 *maximux = BinaryTree::Empty;
                            //             }
                            //             (BinaryTree::NonEmpty(left), BinaryTree::Empty) => {
                            //                 *maximux = BinaryTree::NonEmpty(left.clone());
                            //             }
                            //             _ => {
                            //                 unreachable!()
                            //             }
                            //         }
                            //     }
                            //     Some(elem)
                            // }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                    Ordering::Greater => {
                        node.right.remove(value)
                    }
                    Ordering::Less => {
                        node.left.remove(value)
                    }
                }
            }
            BinaryTree::Empty => {
                None
            }
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

// #[test]
// fn test_remove_root() {
//     let mut tree = BinaryTree::new();
//     tree.add(1);
//     assert_eq!(tree.get(&1), Some(&1));
//     tree.remove(&1);
//     assert_eq!(tree.get(&1), None);
//     tree.add(2);
//     assert_eq!(tree.get(&2), Some(&2));
// }
//
// #[test]
// fn test_remove_leaf() {
//     let mut tree = BinaryTree::new();
//     tree.add(5);
//     tree.add(7);
//     tree.add(2);
//     tree.add(6);
//     assert_eq!(tree.get(&6), Some(&6));
//     tree.remove(&6);
//     assert_eq!(tree.get(&6), None);
//     tree.remove(&2);
//     assert_eq!(tree.get(&2), None);
// }
//
// #[test]
// fn test_remove() {
//     let mut tree = BinaryTree::new();
//     tree.add(4);
//     tree.add(7);
//     tree.add(2);
//     tree.add(6);
//     tree.add(5);
//     tree.add(9);
//
//     tree.remove(&7);
//     assert_eq!(tree.get(&5), Some(&5));
// }
//
// #[test]
// fn test_remove_exist_two_child(){
//     let mut tree = BinaryTree::new();
//     tree.add(3);
//     tree.add(7);
//     tree.add(2);
//     tree.add(9);
//     tree.add(5);
//     assert_eq!(tree.get(&7), Some(&7));
//     assert_eq!(tree.get(&8), None);
//     tree.remove(&7);
//     assert_eq!(tree.get(&7), None);
//     assert_eq!(tree.get(&5), Some(&5));
//     assert_eq!(tree.get(&5), Some(&9));
// }