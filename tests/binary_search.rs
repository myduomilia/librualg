extern crate librualg;

use librualg::*;
use std::time::Instant;

#[test]
fn binary_search() {
    let seq = [1, 2, 3, 3, 4, 5];
    assert_eq!(binary_search::upper_bound(&seq, &3), Some(3));
    assert_eq!(binary_search::lower_bound(&seq, &3), Some(2));
    assert_eq!(binary_search::lower_bound(&seq, &6), None);
}