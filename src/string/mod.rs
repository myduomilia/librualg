use std::cmp::{min, max};
use crate::segment_tree::{RmqMin, SegmentTreeMin, SegmentTreeMax};
use std::collections::BTreeMap;

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

/// Sufix Array
///```
/// use librualg::string::suffix_array;
///
/// assert_eq!(suffix_array("ababba$").0, vec![6, 5, 0, 2, 4, 1, 3]);
/// assert_eq!(suffix_array("bababa$").0, vec![6, 5, 3, 1, 4, 2, 0]);
/// ```
pub fn suffix_array(src: &str) -> (Vec<usize>, Vec<usize>) {

    use std::cmp::Ordering;

    #[derive(Eq, Copy, Clone, Default)]
    struct Pair<T>
        where T: std::cmp::Ord {
        first: T,
        second: usize,
    }

    impl<T> Ord for Pair<T>
        where T: std::cmp::Ord {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.first.eq(&other.first){
                self.second.cmp(&other.second)
            }else {
                self.first.cmp(&other.first)
            }
        }
    }

    impl <T> PartialOrd for Pair<T>
        where T: std::cmp::Ord {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl <T> PartialEq for Pair<T>
        where T: std::cmp::Ord {
        fn eq(&self, other: &Self) -> bool {
            self.first == other.first && self.second == other.second
        }
    }

    fn radix_sort(mut arr: Vec<Pair<Pair<usize>>>) -> Vec<Pair<Pair<usize>>> {
        let length = arr.len();

        {
            let mut cnt = vec![0; length];
            for value in &arr {
                cnt[value.first.second] += 1;
            }
            let mut arr_new: Vec<Pair<Pair<usize>>> = vec![Default::default(); length];
            let mut pos = vec![0; length];
            for i in 1..length {
                pos[i] = pos[i -1] + cnt[i - 1];
            }
            for value in &arr {
                let idx = value.first.second;
                arr_new[pos[idx]] = *value;
                pos[idx] += 1;
            }
            arr = arr_new;
        }
        {
            let mut cnt = vec![0; length];
            for value in &arr {
                cnt[value.first.first] += 1;
            }
            let mut arr_new = vec![Default::default(); length];
            let mut pos = vec![0; length];
            for i in 1..length {
                pos[i] = pos[i -1] + cnt[i - 1];
            }
            for value in arr {
                let idx = value.first.first;
                arr_new[pos[idx]] = value;
                pos[idx] += 1;
            }
            arr = arr_new;
        }
        arr
    }

    let bytes = src.as_bytes();
    let length = src.len();
    let mut suffix_array = vec![0; length];
    let mut classes = vec![0; length];
    {
        let mut a: Vec<Pair<u8>> = vec![Default::default(); length];
        for (i, ch) in bytes.iter().enumerate() {
            a[i] = Pair{first: *ch, second: i};
        }
        a.sort();
        for i in 0..length {
            suffix_array[i] = a[i].second;
        }
        classes[suffix_array[0]] = 0;
        for i in 1..length {
            if a[i].first == a[i - 1].first {
                classes[suffix_array[i]] = classes[suffix_array[i - 1]];
            }else {
                classes[suffix_array[i]] = classes[suffix_array[i - 1]] + 1;
            }
        }
    }
    let mut k = 0;
    while (1 << k) < length {
        let mut a = vec![Default::default(); length];
        for i in 0..length {
            a[i] = Pair{first: Pair {first: classes[i], second: classes[(i + (1 << k)) % length]}, second: i};
        }
        a = radix_sort(a);
        for i in 0..length {
            suffix_array[i] = a[i].second;
        }
        classes[suffix_array[0]] = 0;
        for i in 1..length {
            if a[i].first == a[i - 1].first {
                classes[suffix_array[i]] = classes[suffix_array[i - 1]];
            }else {
                classes[suffix_array[i]] = classes[suffix_array[i - 1]] + 1;
            }
        }
        k += 1;
    }
    (suffix_array, classes)
}

#[test]
fn test_suffix_array() {
    assert_eq!(suffix_array("ababba$").0, vec![6, 5, 0, 2, 4, 1, 3]);
    assert_eq!(suffix_array("bababa$").0, vec![6, 5, 3, 1, 4, 2, 0]);
}

/// Longest Common Prefix
///```
/// use librualg::string::{Lcp, suffix_array};
///
/// let (p, c) = suffix_array("ababba$");
/// let data = Lcp::build(&p, &c, "ababba$");
/// assert_eq!(data.lcp(0, 5), Some(1));
/// assert_eq!(data.lcp(1, 4), Some(2));
/// ```
pub struct Lcp<'a> {
    data: RmqMin<usize>,
    suffix_array: (&'a [usize], &'a [usize]),
    pos_array: BTreeMap<usize, usize>
}

#[allow(clippy::many_single_char_names)]
impl<'a> Lcp<'a> {
    pub fn build(suffix_array: &'a[usize], classes: &'a[usize], text: &str) -> Self {
        let mut lcp = vec![0; text.len()];
        let bytes = text.as_bytes();
        let mut k = 0;
        for i in 0.. text.len() - 1 {
            let pi = classes[i];
            let j = suffix_array[pi - 1];
            while bytes[i + k] == bytes[j + k] {
                k += 1;
            }
            lcp[pi] = k;
            if k > 0 {
                k = max(k - 1, 0);
            }
        }
        let mut pos = BTreeMap::new();
        for (i, item) in suffix_array.iter().enumerate() {
            pos.insert(*item, i);
        }
        Lcp {data: RmqMin::new(&lcp), suffix_array: (suffix_array, classes), pos_array: pos }
    }

    pub fn lcp(&self, i: usize, j: usize) -> Option<usize> {
        impl SegmentTreeMin for usize {
            fn minimal() -> usize {
                usize::MIN
            }
        }

        impl SegmentTreeMax for usize {
            fn maximal() -> usize {
                usize::MAX
            }
        }
        if let Some(pos_i) = self.pos_array.get(&i) {
            if let Some(pos_j) = self.pos_array.get(&j) {
                if *pos_i == *pos_j {
                    return Some(self.suffix_array.0.len() - i - 1);
                }
                return self.data.query(min(*pos_i, *pos_j) + 1, max(*pos_i, *pos_j));
            }
        }
        None
    }
}

#[test]
fn test_lcp() {
    let (p, c) = suffix_array("ababba$");
    let data = Lcp::build(&p, &c, "ababba$");
    assert_eq!(data.lcp(0, 5), Some(1));
    assert_eq!(data.lcp(0, 1), Some(0));
    assert_eq!(data.lcp(1, 4), Some(2));
    assert_eq!(data.lcp(3, 4), Some(1));
    assert_eq!(data.lcp(4, 1), Some(2));
    assert_eq!(data.lcp(1, 11), None);

    assert_eq!(data.lcp(3, 3), Some(3));
    assert_eq!(data.lcp(0, 0), Some(6));

}