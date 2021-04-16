use std::cmp::{min, max};
use std::mem::swap;

/// Sparse Table Min
///```
/// use librualg::sparse_table::SparseTableMin;
///
/// let arr = [5, 2, 3, 4, 5, 6, 1, 18, 9, 10];
/// let table = SparseTableMin::build(&arr);
/// assert_eq!(table.query(0, 9), 1);
/// assert_eq!(table.query(5, 7), 1);
/// assert_eq!(table.query(7, 7), 18);
/// ```
pub struct SparseTableMin <T: Default + Clone + Copy + Ord> {
    data: Vec<Vec<T>>,
    plog: Vec<usize>
}

/// Sparse Table Max
///```
/// use librualg::sparse_table::SparseTableMax;
///
/// let arr = [5, 2, 3, 4, 5, 6, 1, 18, 9, 10];
/// let table = SparseTableMax::build(&arr);
/// assert_eq!(table.query(0, 9), 18);
/// assert_eq!(table.query(1, 4), 5);
/// assert_eq!(table.query(7, 7), 18);
/// ```
pub struct SparseTableMax <T: Default + Clone + Copy + Ord> {
    data: Vec<Vec<T>>,
    plog: Vec<usize>
}

impl <T> SparseTableMin<T> where T: Default + Clone + Copy + Ord {
    pub fn build(src: &[T]) -> Self {
        let mut k = 0;
        while (1 << k) <= src.len() {
            k += 1;
        }
        let mut data = vec![vec![T::default(); src.len()]; k];
        let mut plog  = vec![0; src.len()];
        for i in 0..src.len() {
            data[0][i] = src[i];
            if i > 0 {
                if (1 << plog[i - 1]) * 2 < i + 1 {
                    plog[i] = plog[i - 1] + 1;
                } else {
                    plog[i] = plog[i - 1];
                }
            }
        }
        for i in 1..k {
            for j in 0..src.len() - (1 << i) + 1 {
                data[i][j] = min(data[i - 1][j], data[i - 1][j + (1 << (i - 1))]);
            }
        }
       SparseTableMin{data, plog}
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        if l > r {
            swap(&mut l, &mut r);
        }
        let k = self.plog[r - l];
        min(self.data[k][l], self.data[k][(r + 1) - (1 << k)])
    }
}

impl <T> SparseTableMax<T> where T: Default + Clone + Copy + Ord {
    pub fn build(src: &[T]) -> Self {
        let mut k = 0;
        while (1 << k) <= src.len() {
            k += 1;
        }
        let mut data = vec![vec![T::default(); src.len()]; k];
        let mut plog  = vec![0; src.len()];
        for i in 0..src.len() {
            data[0][i] = src[i];
            if i > 0 {
                if (1 << plog[i - 1]) * 2 < i + 1 {
                    plog[i] = plog[i - 1] + 1;
                } else {
                    plog[i] = plog[i - 1];
                }
            }
        }
        for i in 1..k {
            for j in 0..src.len() - (1 << i) + 1 {
                data[i][j] = max(data[i - 1][j], data[i - 1][j + (1 << (i - 1))]);
            }
        }
        SparseTableMax{data, plog}
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        if l > r {
            swap(&mut l, &mut r);
        }
        let k = self.plog[r - l];
        max(self.data[k][l], self.data[k][(r + 1) - (1 << k)])
    }
}

#[test]
fn test_sparse_table_min() {

    use rand::Rng;

    let arr = [5, 2, 3, 4, 5, 6, 1, 18, 9, 10];
    let table = SparseTableMin::build(&arr);
    assert_eq!(table.query(0, 9), 1);
    assert_eq!(table.query(5, 7), 1);
    assert_eq!(table.query(7, 7), 18);

    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1, 1000);
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = rand::random::<u16>() % 1000;
        }
        let table = SparseTableMin::build(&v);
        for _ in 0..100 {
            let mut l = rand::random::<usize>() % n;
            let mut r = rand::random::<usize>() % n;
            if l > r {
                swap(&mut l, &mut r);
            }
            let ans = v[l..r + 1].iter().min().unwrap();
            assert_eq!(table.query(l, r), *ans);
        }
    }
}

#[test]
fn test_sparse_table_max() {

    use rand::Rng;

    let arr = [5, 2, 3, 4, 5, 6, 1, 18, 9, 10];
    let table = SparseTableMax::build(&arr);
    assert_eq!(table.query(0, 9), 18);
    assert_eq!(table.query(1, 4), 5);
    assert_eq!(table.query(7, 7), 18);

    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(1, 1000);
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = rand::random::<u16>() % 1000;
        }
        let table = SparseTableMax::build(&v);
        for _ in 0..100 {
            let mut l = rand::random::<usize>() % n;
            let mut r = rand::random::<usize>() % n;
            if l > r {
                swap(&mut l, &mut r);
            }
            let ans = v[l..r + 1].iter().max().unwrap();
            assert_eq!(table.query(l, r), *ans);
        }
    }
}
