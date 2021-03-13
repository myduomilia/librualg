/// Algorithm of permutation generation (Narayana Pandita).
///```
/// use librualg::combinatorics::Permutations;
///
/// let per = Permutations::new(3);
/// let arr = vec![[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
/// for (i, value) in per.into_iter().enumerate() {
///     assert_eq!(vec![value[0], value[1], value[2]], arr[i]);
/// }
/// ```

pub struct Permutations {
    arr: Vec<usize>,
    start: bool
}

impl Permutations {
    pub fn new(n: usize) -> Self {
        Permutations{arr: (0..n).collect::<Vec<usize>>(), start: true}
    }
}

impl Iterator for Permutations {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Vec<usize>> {
        if self.start {
            self.start = false;
            return Some(self.arr.clone());
        }
        for i in (1 .. self.arr.len()).rev() {
            if self.arr[i - 1] < self.arr[i] {
                for j in (i .. self.arr.len()).rev() {
                    if self.arr[j] > self.arr[i - 1] {
                        self.arr.swap(i - 1, j);
                        self.arr[i .. ].reverse();
                        return Some(self.arr.clone());
                    }
                }
            }
        }
        None
    }
}

#[test]
fn test() {
    let per = Permutations::new(3);
    let arr = vec![[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut counter = 0;
    for (i, value) in per.into_iter().enumerate() {
        counter += 1;
        assert_eq!(vec![value[0], value[1], value[2]], arr[i]);
    }
    assert_eq!(counter, arr.len());
}
