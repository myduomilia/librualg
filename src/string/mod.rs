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

/// Search for the minimum string period
///```
/// use librualg::string::minimum_string_period;
///
/// assert_eq!(minimum_string_period("abcabcabca"), "abc");
/// assert_eq!(minimum_string_period("abcdefg"), "abcdefg");
/// ```

pub fn minimum_string_period(src: &str) ->&str {
    for (idx, value) in z_function(src).iter().enumerate() {
        if value + idx == src.len() && src.len() % idx == 0 {
            return &src[..idx];
        } else if value + idx == src.len() {
            let k = src.len() % idx;
            if src[..k] == src[src.len() - k..] {
                return &src[..idx];
            }
        }
    }
    src
}

#[test]
fn test_minimum_string_period() {
    assert_eq!(minimum_string_period("abcabcabca"), "abc");
    assert_eq!(minimum_string_period("abcdefg"), "abcdefg");
    assert_eq!(minimum_string_period("abcabcabcd"), "abcabcabcd");
    assert_eq!(minimum_string_period(""), "");
}

/// Search for distinct substring
///```
/// use librualg::string::distinct_substrings;
///
/// assert_eq!(distinct_substrings("a"), vec!["a"]);
/// assert_eq!(distinct_substrings("aaaa"), vec!["a", "aa", "aaa", "aaaa"]);
/// let mut values = distinct_substrings("abaaba");
/// values.sort();
/// assert_eq!(values, vec!["a", "aa", "aab", "aaba", "ab", "aba", "abaa", "abaab", "abaaba", "b", "ba", "baa", "baab", "baaba"]);
/// ```
pub fn distinct_substrings(s: &str)->Vec<&str> {
    let mut seq = vec![];
    for i in (0..s.len()).rev() {
        let pr = z_function(&s[i ..s.len()]);
        let res = s.len() - i - pr.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        for j in 0..res {
            seq.push(&s[i..s.len() - j]);
        }
    }
    seq
}

#[test]
fn test_distinct_substrings(){
    assert_eq!(distinct_substrings("a"), vec!["a"]);
    assert_eq!(distinct_substrings("aaaa"), vec!["a", "aa", "aaa", "aaaa"]);
    assert_eq!(distinct_substrings(""), Vec::<&str>::new());
    let mut values = distinct_substrings("abaaba");
    values.sort();
    assert_eq!(values, vec!["a", "aa", "aab", "aaba", "ab", "aba", "abaa", "abaab", "abaaba", "b", "ba", "baa", "baab", "baaba"]);
    assert_eq!(distinct_substrings("abacabadabacaba").len(), 85);
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

pub fn z_function(src: &str) -> Vec<usize> {
    let mut z = vec![0; src.len()];
    let mut l = 0;
    let mut r = 0;

    let arr = src.as_bytes();
    for i in 1..src.len() {
        if i <= r {
            z[i] = min(r - i + 1, z[i - l]);
        }
        while i + z[i] < arr.len() && arr[z[i]] == arr[i + z[i]]{
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}

#[test]
fn test_z_function_ascii() {
    assert_eq!(z_function("abacaba"), [0, 0, 1, 0, 3, 0, 1]);
}