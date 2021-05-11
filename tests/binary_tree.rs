use librualg::binary_tree::BinaryTree;

#[test]
fn test() {
    let mut tree = BinaryTree::new();
    tree.add(3);
    tree.add(7);
    tree.add(2);
    assert_eq!(tree.get(&7), Some(&7));
    assert_eq!(tree.get(&8), None);
}