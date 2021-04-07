use std::cmp::min;

/// Knuth–Morris–Pratt string-searching algorithm (or KMP algorithm).
/// Return all occurrences of a substring.
///```
/// use librualg::string::kmp;
///
/// assert_eq!(kmp("abcdabcd", "abc"), vec![0, 4]);
/// ```

pub fn kmp(t: &str, p: &str) -> Vec<usize> {
    let mut res = vec![];
    let pr = prefix_function(p);
    let mut idx = 0;
    let pattern = p.as_bytes();
    for (i, value) in t.as_bytes().iter().enumerate() {
        while idx > 0  && pattern[idx] != *value{
            idx = pr[idx - 1];
        }
        if pattern[idx] == *value {
            idx += 1;
        }
        if idx == p.len() {
            res.push(i + 1 - idx);
            idx = pr[idx - 1];
        }
    }
    res
}

#[test]
fn test_kmp(){
    assert_eq!(kmp("ababcxabdabcxabcxabcde", "abcxabcde"), vec![13]);
    assert_eq!(kmp("a", "ab"), vec![]);
    assert_eq!(kmp("aaaaa", "a"), vec![0, 1, 2, 3, 4]);
    assert_eq!(kmp("abcdabcd", "abc"), vec![0, 4]);
}

/// Knuth–Morris–Pratt string-searching algorithm (or KMP algorithm).
/// Return first occurrence of a substring.
///```
/// use librualg::string::kmp_first;
///
/// assert_eq!(kmp_first("cbcdabcd", "abc"), Some(4));
/// assert_eq!(kmp_first("cbcdabcd", "ebc"), None);
/// ```

pub fn kmp_first(t: &str, p: &str) -> Option<usize> {
    let pr = prefix_function(p);
    let mut idx = 0;
    let pattern = p.as_bytes();
    for (i, value) in t.as_bytes().iter().enumerate() {
        while idx > 0  && pattern[idx] != *value{
            idx = pr[idx - 1];
        }
        if pattern[idx] == *value {
            idx += 1;
        }
        if idx == p.len() {
            return Some(i + 1 - idx);
        }
    }
    None
}



#[test]
fn test_kmp_first(){
    assert_eq!(kmp_first("ababcxabdabcxabcxabcde", "abcxabcde"), Some(13));
    assert_eq!(kmp_first("a", "ab"), None);
    assert_eq!(kmp_first("aaaaa", "a"), Some(0));
    assert_eq!(kmp_first("ebcdabcd", "abc"), Some(4));
}

/// Levenshtein distance (Metric of the difference between two symbol sequences).
///```
/// use librualg::string::levenshtein_distance;
///
/// assert_eq!(levenshtein_distance("POLYNOMIAL", "EXPONENTIAL", 1, 1, 1), 6);
/// assert_eq!(levenshtein_distance("aaa", "aaa", 1, 1, 1), 0);
/// ```
pub fn levenshtein_distance(first: &str, second: &str, delete_cost: u32, insert_cost: u32, replace_cost: u32) -> u32 {
    let first = first.as_bytes();
    let second = second.as_bytes();
    let mut dist_old = vec![0; first.len() + 1];
    for j in 1..(first.len() + 1) {
        dist_old[j] = dist_old[j - 1] + insert_cost;
    }
    for i in 1..second.len() + 1 {
        let mut dist = vec![0; first.len() + 1];
        dist[0] = dist_old[0] + delete_cost;
        for j in 1..(first.len() + 1) {
            if second[i - 1] != first[j - 1] {
                dist[j] = min(min(dist_old[j] + delete_cost, dist_old[j - 1] + insert_cost), dist[j - 1] + replace_cost);
            } else {
                dist[j] = dist_old[j - 1];
            }
        }
        dist_old = dist;
    }
    dist_old[first.len()]
}

#[test]
fn test_levenshtein_distance(){
    assert_eq!(levenshtein_distance("POLYNOMIAL", "EXPONENTIAL", 1, 1, 1), 6);
    assert_eq!(levenshtein_distance("abcdasdasd", "cddabcd", 1, 1, 1), 6);
    assert_eq!(levenshtein_distance("", "", 1, 1, 1), 0);
    assert_eq!(levenshtein_distance("aaa", "aaa", 1, 1, 1), 0);
    assert_eq!(levenshtein_distance("", "aaa", 1, 1, 1), 3);
}

fn prefix_function(src: &str) -> Vec<usize> {
    let mut pi = vec![0; src.len()];
    let arr = src.as_bytes();
    for i in 1 .. arr.len() {
        let mut j = pi[i - 1];
        while j > 0 && arr[i] != arr[j] {
            j = pi[j - 1];
        }
        if arr[i] == arr[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

#[test]
fn test_prefix_function() {
    assert_eq!(prefix_function("abacaba"), [0, 0, 1, 0, 1, 2, 3]);
    assert_eq!(prefix_function("b"), [0]);
    assert_eq!(prefix_function("aaaaa"), [0, 1, 2, 3, 4]);
    assert_eq!(prefix_function(""), []);
}