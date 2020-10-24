/// Binary search algorithm.
/// Returns the leftmost position of the item to find.
/// It is necessary that the container is pre-sorted
///```
/// use librualg::binary_search::lower_bound;
///
/// let seq = vec![1, 2, 3, 4, 5, 8, 8, 8, 9, 20];
///
/// assert_eq!(lower_bound(&seq, &8).unwrap(), 5);
/// assert_eq!(lower_bound(&seq, &7), None);
/// ```

pub fn lower_bound<T>(container: &[T], key: &T) -> Option<usize>
    where T: std::cmp::Ord {
    if container.is_empty() {
        return None;
    }
    let mut l = 0;
    let mut r = container.len() - 1;
    while l < r {
        let idx = l + (r - l) / 2;
        if container[idx] < *key {
            l = idx + 1;
        } else {
            r = idx;
        }
    }
    match container[l] == *key {
        true => Some(l),
        _ => None
    }
}

/// Binary search algorithm.
/// Returns the rightmost position of the item to find.
/// It is necessary that the container is pre-sorted
///```
/// use librualg::binary_search::upper_bound;
///
/// let seq = vec![1, 2, 3, 4, 5, 8, 8, 8, 9, 20];
///
/// assert_eq!(upper_bound(&seq, &8).unwrap(), 7);
/// assert_eq!(upper_bound(&seq, &7), None);
/// ```
pub fn upper_bound<T>(container: &[T], key: &T) -> Option<usize>
    where T: std::cmp::Ord {
    if container.is_empty() {
        return None;
    }
    let mut l = 0;
    let mut r = container.len() - 1;
    while l < r {
        let idx = r - (r - l) / 2;
        if container[idx] <= *key {
            l = idx;
        } else {
            r = idx - 1;
        }
    }
    match container[l] == *key {
        true => Some(l),
        _ => None
    }
}

#[test]
fn test_lower_bound(){
    let seq = vec![1, 2, 3, 4, 5, 8, 8, 8, 9, 20];
    assert_eq!(lower_bound(&seq, &8).unwrap(), 5);
    assert_eq!(lower_bound(&seq, &1).unwrap(), 0);
    assert_eq!(lower_bound(&seq, &7), None);
    assert_eq!(lower_bound(&seq, &21), None);
}

#[test]
fn test_upper_bound(){
    let seq = vec![1, 2, 3, 4, 5, 8, 8, 8, 9, 20];
    assert_eq!(upper_bound(&seq, &8).unwrap(), 7);
    assert_eq!(upper_bound(&seq, &1).unwrap(), 0);
    assert_eq!(upper_bound(&seq, &7), None);
    assert_eq!(upper_bound(&seq, &21), None);
}

#[test]
fn test_empty_container(){
    let seq = vec![];
    assert_eq!(upper_bound(&seq, &1), None);
    assert_eq!(lower_bound(&seq, &1), None);
}