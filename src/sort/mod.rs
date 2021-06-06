/// Insertion sort
///```
/// use librualg::sort::insertion_sort;
///
/// let mut v = vec![3, 5, 2, 9, 12, 1, 3, 3, 7];
/// insertion_sort(&mut v);
/// assert_eq!(vec![1, 2, 3, 3, 3, 5, 7, 9, 12], v);
/// ```

pub fn insertion_sort<T>(src: &mut [T]) where T:Ord + Copy {
    for i in 1..src.len() {
        let mut j = i;
        let ptr = &mut src[0] as *mut T;
        unsafe {
            while j > 0 && *ptr.offset(j as isize) < *ptr.offset(j as isize - 1) {
                swap(ptr.offset(j as isize), ptr.offset(j as isize - 1));
                j -= 1;
            }
        }
    }
}

unsafe fn swap<T:Ord + Copy>(a: *mut T, b: *mut T) {
    let value = *a;
    *a = *b;
    *b = value;
}

#[test]
pub fn test_insertion_sort(){
    let mut v = vec![3, 5, 2, 9, 12, 1, 3, 3, 7];
    insertion_sort(&mut v);
    assert_eq!(vec![1, 2, 3, 3, 3, 5, 7, 9, 12], v);

    let mut v: Vec<i32> = vec![];
    insertion_sort(&mut v);
    assert_eq!(Vec::<i32>::new(), v);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    insertion_sort(&mut v);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], v);

    let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    insertion_sort(&mut v);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], v);

}