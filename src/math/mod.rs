/// The Greatest Common Divisor (GCD)
///```
/// use librualg::math::gcd;
///
/// assert_eq!(gcd(24, 60), 12);
///
/// ```

pub fn gcd(a: u64, b: u64) -> u64{
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) if a > b => {
            gcd(a % b, b)
        }
        (a, b) => {
            gcd(a, b % a)
        }
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(24, 60), 12);
    assert_eq!(gcd(0, 7), 7);
    assert_eq!(gcd(3, 0), 3);
    assert_eq!(gcd(11, 11), 11);
    assert_eq!(gcd(0, 0), 0);
}