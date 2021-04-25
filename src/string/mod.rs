use std::cmp::{min, max};
use crate::segment_tree::{RmqMin, SegmentTreeMin, SegmentTreeMax};
use std::collections::{BTreeMap, VecDeque};

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

/// String hashing function
///```
/// use librualg::string::hash;
///
/// assert_eq!(hash("abcdabcd"), 2842022591228);
/// ```
pub fn hash(s: &str) -> u64 {
    let mut k = 1;
    let mut res = 0u64;
    for ch in s.as_bytes() {
        res = res.wrapping_add((*ch as u64).wrapping_mul(k));
        k =  k.wrapping_mul(31);
    }
    res
}

#[test]
fn test_hash() {
    hash("");
    hash("a");
    hash("abc");
}

/// Search for a common substring
///```
/// use librualg::string::common_substring;
///
/// assert_eq!(common_substring("aba", "cabdd"), Some("ab"));
/// ```

pub fn common_substring<'a> (a: &'a str, b: &'a str) -> Option<&'a str> {
    if a.is_empty() || b.is_empty() {
        return None;
    }
    let mut p: Vec<u64> = vec![1; max(a.len(), b.len())];
    let mut h1: Vec<u64> = vec![0; a.len()];
    let mut h2: Vec<u64> = vec![0; b.len()];
    for idx in 1..p.len() {
        p[idx] = p[idx - 1].wrapping_mul(31);
    }
    for (idx, ch) in a.as_bytes().iter().enumerate() {
        h1[idx] = (*ch as u64).wrapping_mul(p[idx]);
        if idx != 0 {
            h1[idx] = h1[idx].wrapping_add(h1[idx - 1]);
        }
    }
    for (idx, ch) in b.as_bytes().iter().enumerate() {
        h2[idx] = (*ch as u64).wrapping_mul(p[idx]);
        if idx != 0 {
            h2[idx] = h2[idx].wrapping_add(h2[idx - 1]);
        }
    }
    let mut res = None;
    let mut l = 0;
    let mut r = min(a.len(), b.len()) - 1;
    while l < r {
        let mid = r - (r - l) / 2;
        let mut map = BTreeMap::new();
        for i in 0..a.len() - mid + 1 {
            let mut hash = h1[i + mid - 1];
            if i != 0 {
                hash = hash.wrapping_sub(h1[i - 1]);
            }
            hash = hash.wrapping_mul(p[p.len() - i - 1]);
            map.insert(hash, i);
        }
        let mut f = false;
        for i in 0..b.len() - mid + 1 {
            let mut hash = h2[i + mid - 1];
            if i != 0 {
                hash = hash.wrapping_sub(h2[i - 1]);
            }
            hash = hash.wrapping_mul(p[p.len() - i - 1]);
            if let Some(idx) = map.get(&hash) {
                if &b[i..i + mid] == &a[*idx..*idx + mid] {
                    res = Some(&b[i..i + mid]);
                    f = true;
                    break;
                }
            }
        }
        if f {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    let mut map = BTreeMap::new();
    for i in 0..a.len() - l + 1 {
        let mut hash = h1[i + l - 1];
        if i != 0 {
            hash = hash.wrapping_sub(h1[i - 1]);
        }
        hash = hash.wrapping_mul(p[p.len() - i - 1]);
        map.insert(hash, i);
    }
    for i in 0..b.len() - l + 1 {
        let mut hash = h2[i + l - 1];
        if i != 0 {
            hash = hash.wrapping_sub(h2[i - 1]);
        }
        hash = hash.wrapping_mul(p[p.len() - i - 1]);
        if let Some(idx) = map.get(&hash) {
            if &b[i..i + l] == &a[*idx..*idx + l] {
                res = Some(&b[i..i + l]);
                break;
            }
        }
    }
    res
}

#[test]
fn test_common_substring() {
    assert_eq!(common_substring("VOTEFORTHEGREATALBANIAFORYOU", "CHOOSETHEGREATALBANIANFUTURE"), Some("THEGREATALBANIA"));
    assert_eq!(common_substring("aba", "cabdd"), Some("ab"));
    assert_eq!(common_substring("aaaaa", "bbaaa"), Some("aaa"));
    assert_eq!(common_substring("", "bbaaa"), None);
    assert_eq!(common_substring("abcde", "abcde"), Some("abcde"));
    assert_eq!(common_substring("aaaaaaaaaaaaaaaaaaaaaaaaab", "aaaaaaaaaaaaaaaaaaaaaaaaac"), Some("aaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[derive(Clone)]
struct VertexAhoCorasick {
    children: BTreeMap<i32, i32>,
    link: i32,
    good_link: i32,
    pat_num: i32,
    pch: i32,
    parent: i32,
}

struct TrieAhoCorasick {
    arr: Vec<VertexAhoCorasick>,
    sz: i32,
}

impl TrieAhoCorasick {
    fn new() -> TrieAhoCorasick {
        TrieAhoCorasick { arr: vec![VertexAhoCorasick { children: BTreeMap::new(), link: -1, good_link: -1, pat_num: -1, pch: -1, parent: -1 }; 1], sz: 1 }
    }
    fn insert(&mut self, s: &str, num: i32) {
        let mut v = 0;
        for ch in s.as_bytes() {
            let idx = *ch as i32;
            if self.arr[v].children.get(&idx).is_none() {
                self.arr.push(VertexAhoCorasick { children: BTreeMap::new(), link: 0, good_link: -1, pat_num: -1, pch: idx, parent: v as i32 });
                self.arr[v].children.insert(idx, self.sz);
                self.sz += 1;
            }
            v = *self.arr[v].children.get(&idx).unwrap() as usize;
        }
        self.arr[v].pat_num = num;
    }
}

/// Algorithm Aho Corasick. Search for a set of substring from the dictionary in the given string.
///```
/// use librualg::string::aho_corasick;
/// use std::collections::BTreeMap;
///
/// let dict = ["aba", "baba", "cc"];
/// let t = "ababababa";
/// let res = aho_corasick(&dict, t);
///
/// let mut m = BTreeMap::new();
/// m.insert(0, vec![0, 2, 4, 6]);
/// m.insert(1, vec![1, 3, 5]);
/// assert_eq!(m, res);
/// ```

pub fn aho_corasick(dict: &[&str], t: &str) -> BTreeMap<i32, Vec<usize>> {
    let mut res: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    let mut trie = TrieAhoCorasick::new();
    for (idx, s) in dict.iter().enumerate() {
        trie.insert(*s, idx as i32);
    }
    let mut q = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();

        for (_, value) in &trie.arr[curr as usize].children {
            q.push_back(*value);
        }
        if curr == 0 {
            continue
        }
        let parent = trie.arr[curr as usize].parent;
        let mut next_link = trie.arr[parent as usize].link;
        let pch = trie.arr[curr as usize].pch;
        while next_link >= 0 && trie.arr[next_link as usize].children.get(&pch).is_none() {
            next_link = trie.arr[next_link as usize].link;
        }
        if next_link >= 0 {
            let link = *trie.arr[next_link as usize].children.get(&pch).unwrap();
            let good_link;
            if trie.arr[link as usize].pat_num != -1 {
                good_link = link;
            } else {
                good_link = trie.arr[link as usize].good_link;
            }
            let r = &mut trie.arr[curr as usize];
            r.link = link;
            r.good_link = good_link;
        }
    }
    let mut v = 0i32;
    for (i, ch) in t.as_bytes().iter().enumerate() {
        let idx = *ch as i32;
        while v >= 0 && trie.arr[v as usize].children.get(&idx).is_none() {
            v = trie.arr[v as usize].link;
        }
        if v == -1 {
            v = 0;
        } else {
            v = *trie.arr[v as usize].children.get(&idx).unwrap();
        }
        if trie.arr[v as usize].pat_num != -1 {
            if res.contains_key(&trie.arr[v as usize].pat_num) {
                res.get_mut(&trie.arr[v as usize].pat_num).unwrap().push(i + 1 - dict[trie.arr[v as usize].pat_num as usize].len());
            } else {
                res.insert(trie.arr[v as usize].pat_num, vec![i + 1 - dict[trie.arr[v as usize].pat_num as usize].len()]);
            }
        }
        let mut good_link = trie.arr[v as usize].good_link;
        while good_link > 0 {
            if res.contains_key(&trie.arr[good_link as usize].pat_num) {
                res.get_mut(&trie.arr[good_link as usize].pat_num).unwrap().push(i + 1 - dict[trie.arr[good_link as usize].pat_num as usize].len());
            } else {
                res.insert(trie.arr[good_link as usize].pat_num, vec![i + 1 - dict[trie.arr[good_link as usize].pat_num as usize].len()]);
            }
            good_link = trie.arr[good_link as usize].good_link;
        }
    }
    res
}

#[test]
fn test_aho_corasick() {
    let mut dict = ["aba", "abb", "bbca"];
    let t = "abaabbbbca";
    let mut res = aho_corasick(&dict, t);

    let mut m = BTreeMap::new();
    m.insert(0, vec![0]);
    m.insert(1, vec![3]);
    m.insert(2, vec![6]);
    assert_eq!(m, res);

    let t = "abaabbbbcaaba";
    res = aho_corasick(&dict, t);

    m = BTreeMap::new();
    m.insert(0, vec![0, 10]);
    m.insert(1, vec![3]);
    m.insert(2, vec![6]);
    assert_eq!(m, res);

    let t = "abaabbbbcaba";
    res = aho_corasick(&dict, t);

    m = BTreeMap::new();
    m.insert(0, vec![0, 9]);
    m.insert(1, vec![3]);
    m.insert(2, vec![6]);
    assert_eq!(m, res);

    dict = ["abba", "bb", "cc"];
    let t = "abba";
    res = aho_corasick(&dict, t);

    m = BTreeMap::new();
    m.insert(0, vec![0]);
    m.insert(1, vec![1]);
    assert_eq!(m, res);

    dict = ["aba", "baba", "cc"];
    let t = "ababababa";
    res = aho_corasick(&dict, t);

    m = BTreeMap::new();
    m.insert(0, vec![0, 2, 4, 6]);
    m.insert(1, vec![1, 3, 5]);
    assert_eq!(m, res);

}
