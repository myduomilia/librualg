/// Insertion sort
///```
/// use librualg::sort::insertion_sort;
///
/// let mut v = vec![3, 5, 2, 9, 12, 1, 3, 3, 7];
/// insertion_sort(&mut v);
/// assert_eq!(vec![1, 2, 3, 3, 3, 5, 7, 9, 12], v);
/// ```

pub fn insertion_sort<T>(src: &mut [T]) where T:Ord {
    for i in 1..src.len() {
        let mut j = i;
        while j > 0 && src[j] < src[j - 1] {
            src.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[test]
pub fn test_insertion_sort(){
    let mut v = vec![3, 5, 2, 9, 12, 1, 3, 3, 7];
    insertion_sort(&mut v);
    assert_eq!(vec![1, 2, 3, 3, 3, 5, 7, 9, 12], v);

    let mut v: Vec<i32> = vec![];
    insertion_sort(&mut v);
    assert_eq!(Vec::<i32>::new(), v);

}