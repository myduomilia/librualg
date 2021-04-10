extern crate librualg;

use librualg::*;
use librualg::segment_tree::{RmqMin, RmqMax};

#[test]
fn test_rsq() {
    let arr = [1, 2, 3, 4, 5];
    let tree = segment_tree::Rsq::new(&arr);

    assert_eq!(tree.query(0, 4).unwrap(), 15);
    assert_eq!(tree.query(1, 4).unwrap(), 14);
    assert_eq!(tree.query(4, 1).unwrap(), 14);
    assert_eq!(tree.query(3, 1).unwrap(), 9);
    assert_eq!(tree.query(4, 0).unwrap(), 15);
    assert_eq!(tree.query(3, 11), None);

    let arr: Vec<i32> = vec![];
    let tree = segment_tree::Rsq::new(&arr);
    assert_eq!(tree.query(0, 0), None);
}

#[test]
fn test_rmq_min() {
    let arr = [1, 2, 3, 4, 5];
    let tree = RmqMin::new(&arr);

    assert_eq!(tree.query(0, 4).unwrap(), 1);
    assert_eq!(tree.query(1, 4).unwrap(), 2);
    assert_eq!(tree.query(4, 1).unwrap(), 2);
    assert_eq!(tree.query(3, 1).unwrap(), 2);
    assert_eq!(tree.query(2, 2).unwrap(), 3);

    assert_eq!(tree.query(2, 7), None);
}

#[test]
fn test_rmq_max() {
    let arr = [1, 2, 3, 4, 5];
    let tree = RmqMax::new(&arr);

    assert_eq!(tree.query(0, 4).unwrap(), 5);
    assert_eq!(tree.query(1, 4).unwrap(), 5);
    assert_eq!(tree.query(4, 1).unwrap(), 5);
    assert_eq!(tree.query(3, 1).unwrap(), 4);
    assert_eq!(tree.query(2, 2).unwrap(), 3);

    assert_eq!(tree.query(2, 7), None);
}