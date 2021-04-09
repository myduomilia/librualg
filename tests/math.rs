extern crate librualg;

use librualg::math::{gcd, is_simple};

#[test]
fn test_gcd() {
    assert_eq!(gcd(24, 60), 12);
    assert_eq!(gcd(0, 7), 7);
    assert_eq!(gcd(3, 0), 3);
    assert_eq!(gcd(11, 11), 11);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn test_is_simple() {
    assert_eq!(is_simple(157), true);
    assert_eq!(is_simple(39916800), false);
}