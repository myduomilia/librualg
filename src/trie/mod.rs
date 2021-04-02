use std::collections::BTreeMap;

/// Trie or prefix tree
///```
/// use librualg::trie::Trie;
///
/// let mut trie = Trie::new();
/// trie.insert("abab");
/// trie.insert("abcc");
/// trie.insert("ddvbn");
///
/// assert_eq!(trie.contains("abab"), true);
/// assert_eq!(trie.contains("ababa"), false);
/// assert_eq!(trie.contains("abcc"), true);
/// assert_eq!(trie.contains("abc"), false);
/// ```

pub struct Trie {
    children: BTreeMap<u8, Trie>,
    leaf: bool,
}


impl Trie {
    pub fn new() -> Self {
        Trie{ children: BTreeMap::new(), leaf: false}
    }

    pub fn insert(&mut self, s: &str) {
        let mut node = self;
        for ch in s.as_bytes() {
            if node.children.get(ch).is_none() {
                node.children.insert(*ch, Trie{ children: BTreeMap::new(), leaf: false});
            }
            node = node.children.get_mut(ch).unwrap();
        }
        node.leaf = true;
    }

    pub fn contains(&self, p: &str) -> bool {
        let mut node = self;
        for ch in p.as_bytes() {
            if node.children.get(ch).is_none() {
                return false;
            }
            node = node.children.get(ch).unwrap();
        }
        node.leaf
    }

    pub fn remove(&mut self, p: &str) {
        if self.contains(p) {
            let mut node = self;
            for ch in p.as_bytes() {
                if node.children.get(ch).unwrap().children.is_empty() {
                    node.children.remove(ch);
                    return;
                }
                node = node.children.get_mut(ch).unwrap();
            }
            node.leaf = false;
        }
    }
}

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("abab");
    trie.insert("abc");
    trie.insert("abccc");
    trie.insert("ddvbn");

    assert_eq!(trie.contains("abab"), true);
    assert_eq!(trie.contains("ababa"), false);
    assert_eq!(trie.contains("abccc"), true);
    assert_eq!(trie.contains("abcc"), false);
    assert_eq!(trie.contains("abc"), true);

    trie.remove("ab");
    trie.remove("abc");
    assert_eq!(trie.contains("abc"), false);

    trie = Trie::new();
    trie.insert("abc");
    trie.insert("abccc");

    assert_eq!(trie.contains("abccc"), true);
    assert_eq!(trie.contains("abc"), true);

    trie.remove("abccc");
    assert_eq!(trie.contains("abccc"), false);
    assert_eq!(trie.contains("abc"), true);
}
