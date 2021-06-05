use librualg::sort::insertion_sort;

#[test]
pub fn test_insertion_sort(){
    let mut v = vec![3, 5, 2, 9, 12, 1, 3, 3, 7];
    insertion_sort(&mut v);
    assert_eq!(vec![1, 2, 3, 3, 3, 5, 7, 9, 12], v);
}