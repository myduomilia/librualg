extern crate librualg;

use librualg::combinatorics::Permutations;

#[test]
fn permutations(){
    let per = Permutations::new(3);
    let arr = vec![[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    for (i, value) in per.into_iter().enumerate() {
        assert_eq!(vec![value[0], value[1], value[2]], arr[i]);
    }
}