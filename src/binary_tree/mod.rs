/// Binary Tree
/// ```
/// use librualg::binary_tree::BinaryTree;
///
/// let mut tree = BinaryTree::new();
/// tree.add(3);
/// tree.add(7);
/// tree.add(2);
/// ```
pub enum BinaryTree<T> where T: std::cmp::PartialOrd {
    Empty,
    NonEmpty(Box<Node<T>>)
}

pub struct Node<T> where T: std::cmp::PartialOrd {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}
impl <T> BinaryTree<T> where T: std::cmp::PartialOrd {
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
}

#[test]
fn test() {
    let mut tree = BinaryTree::new();
    tree.add(3);
    tree.add(7);
    tree.add(2);
}