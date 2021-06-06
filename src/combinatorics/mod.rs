use std::option::Option::Some;

/// Algorithm of permutation generation (Narayana Pandita).
///```
/// use librualg::combinatorics::next_permutation;
///
/// let arr = vec![[0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
/// let mut values = vec![0, 1, 2];
/// let mut idx = 0;
/// while let Some(_) = next_permutation(&mut values) {
///     assert_eq!(vec![values[0], values[1], values[2]], arr[idx]);
///     idx += 1;
/// }
/// ```

pub fn next_permutation<T: Ord + Copy>(arr: &mut [T]) -> Option<()> {
    for i in (1 .. arr.len()).rev() {
        unsafe {
            let ptr = &mut arr[0] as *mut T;
            if *ptr.offset(i as isize - 1) < *ptr.offset(i as isize) {
                for j in (i .. arr.len()).rev() {
                    if *ptr.offset(j as isize) > *ptr.offset(i as isize - 1){
                        swap(ptr.offset(i as isize - 1), ptr.offset(j as isize));
                        arr[i .. ].reverse();
                        return Some(());
                    }
                }
            }
        }
    }
    None
}

unsafe fn swap<T:Ord + Copy>(a: *mut T, b: *mut T) {
    let value = *a;
    *a = *b;
    *b = value;
}

#[test]
fn test() {
    let arr = vec![[0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut values = vec![0, 1, 2];
    let mut idx = 0;
    while let Some(_) = next_permutation(&mut values) {
        assert_eq!(vec![values[0], values[1], values[2]], arr[idx]);
        idx += 1;
    }
    assert_eq!(idx, arr.len());
}
