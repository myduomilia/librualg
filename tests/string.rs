extern crate librualg;

use librualg::*;

#[test]
fn kmp(){
    assert_eq!(string::kmp("ababcxabdabcxabcxabcde", "abcxabcde"), vec![13]);
    assert_eq!(string::kmp("a", "ab"), vec![]);
    assert_eq!(string::kmp("aaaaa", "a"), vec![0, 1, 2, 3, 4]);
    assert_eq!(string::kmp("abcdabcd", "abc"), vec![0, 4]);
}

#[test]
fn kmp_first(){
    assert_eq!(string::kmp_first("ababcxabdabcxabcxabcde", "abcxabcde"), Some(13));
    assert_eq!(string::kmp_first("a", "ab"), None);
    assert_eq!(string::kmp_first("aaaaa", "a"), Some(0));
    assert_eq!(string::kmp_first("ebcdabcd", "abc"), Some(4));
}