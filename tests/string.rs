extern crate librualg;

use librualg::*;
use librualg::string::levenshtein_distance;

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

#[test]
fn test_levenshtein_distance(){
    assert_eq!(levenshtein_distance("POLYNOMIAL", "EXPONENTIAL", 1, 1, 1), 6);
    assert_eq!(levenshtein_distance("abcdasdasd", "cddabcd", 1, 1, 1), 6);
    assert_eq!(levenshtein_distance("", "", 1, 1, 1), 0);
    assert_eq!(levenshtein_distance("aaa", "aaa", 1, 1, 1), 0);
    assert_eq!(levenshtein_distance("", "aaa", 1, 1, 1), 3);
}

#[test]
fn test_minimum_string_period() {
    assert_eq!(string::minimum_string_period("abcabcabca"), "abc");
    assert_eq!(string::minimum_string_period("abcdefg"), "abcdefg");
}

#[test]
fn test_distinct_substrings(){
    assert_eq!(string::distinct_substrings("a"), vec!["a"]);
    assert_eq!(string::distinct_substrings("aaaa"), vec!["a", "aa", "aaa", "aaaa"]);
    assert_eq!(string::distinct_substrings(""), Vec::<&str>::new());
    let mut values = string::distinct_substrings("abaaba");
    values.sort();
    assert_eq!(values, vec!["a", "aa", "aab", "aaba", "ab", "aba", "abaa", "abaab", "abaaba", "b", "ba", "baa", "baab", "baaba"]);
}

#[test]
fn test_suffix_array() {
    assert_eq!(string::suffix_array("ababba$").0, vec![6, 5, 0, 2, 4, 1, 3]);
    assert_eq!(string::suffix_array("bababa$").0, vec![6, 5, 3, 1, 4, 2, 0]);
}