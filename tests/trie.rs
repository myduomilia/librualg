extern crate librualg;

use librualg::trie::Trie;

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
}