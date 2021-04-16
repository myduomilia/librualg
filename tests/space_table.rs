use librualg::sparse_table::{SparseTableMax, SparseTableMin};

#[test]
fn test_sparse_table_min() {

    let arr = [5, 2, 3, 4, 5, 6, 1, 18, 9, 10];
    let table = SparseTableMin::build(&arr);
    assert_eq!(table.query(0, 9), 1);
    assert_eq!(table.query(5, 7), 1);
    assert_eq!(table.query(7, 7), 18);
}

#[test]
fn test_sparse_table_max() {

    let arr = [5, 2, 3, 4, 5, 6, 1, 18, 9, 10];
    let table = SparseTableMax::build(&arr);
    assert_eq!(table.query(0, 9), 18);
    assert_eq!(table.query(1, 4), 5);
    assert_eq!(table.query(7, 7), 18);

}
