extern crate librualg;

use librualg::combinatorics::next_permutation;

#[test]
fn permutations(){
    let arr = vec![[0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut values = vec![0, 1, 2];
    let mut idx = 0;
    while let Some(_) = next_permutation(&mut values) {
        assert_eq!(vec![values[0], values[1], values[2]], arr[idx]);
        idx += 1;
    }
    assert_eq!(idx, arr.len());
}