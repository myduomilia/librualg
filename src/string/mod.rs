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
    let t_bytes = t.as_bytes();
    let p_bytes = p.as_bytes();
    for i in 0..t.len() {
        while idx > 0  && p_bytes[idx] != t_bytes[i]{
            idx = pr[idx - 1];
        }
        if p_bytes[idx] == t_bytes[i] {
            idx += 1;
        }
        if idx == p.len() {
            res.push(i + 1 - idx);
            idx = pr[idx - 1];
        }
    }
    res
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
    let t_bytes = t.as_bytes();
    let p_bytes = p.as_bytes();
    for i in 0..t.len() {
        while idx > 0  && p_bytes[idx] != t_bytes[i]{
            idx = pr[idx - 1];
        }
        if p_bytes[idx] == t_bytes[i] {
            idx += 1;
        }
        if idx == p.len() {
            return Some(i + 1 - idx);
        }
    }
    None
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