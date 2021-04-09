use rand::Rng;

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

/// The function returns the value of x to the power of y.
///```
/// use librualg::math::pow;
///
/// assert_eq!(pow(2, 20), 1048576);
///
/// ```
pub fn pow(mut value: u64, mut n: u64) -> u64 {
    if n == 0 {
        return 1_u64;
    }
    let mut res = 1;
    while n > 0 {
        if n % 2 != 0 {
            res *= value;
            n -= 1;
        } else {
            value = value * value;
            n >>= 1;
        }
    }
    res
}

#[test]
fn test_pow() {
    assert_eq!(pow(1, 3), 1);
    assert_eq!(pow(0, 3), 0);
    assert_eq!(pow(2, 20), 1048576);
}

/// The function returns the value of x to the power of y by module.
///```
/// use librualg::math::pow_mod;
///
/// assert_eq!(pow_mod(5, 100, 7), 2);
///
/// ```
pub fn pow_mod(mut value: u64, mut n: u64, m: u64) -> u64 {
    if n == 0 {
        return 1_u64;
    }
    let mut res = 1;
    while n > 0 {
        if n % 2 != 0 {
            res = (res * value) % m;
            n -= 1;
        } else {
            value = (value * value) % m;
            n >>= 1;
        }
    }
    res
}

#[test]
fn test_fast_pow_mod() {
    assert_eq!(pow_mod(5, 100, 7), 2);
    assert_eq!(pow_mod(3, 90, 5), 4);
}

/// Checking a Number for Simplicity (Fermat's test)
///```
/// use librualg::math::is_simple;
///
/// assert_eq!(is_simple(157), true);
/// assert_eq!(is_simple(8505), false);
///
/// ```
pub fn is_simple(value: u64) -> bool {
    if value == 2 {
        return true;
    }
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let a = rng.gen_range(1, u64::MAX) % (value - 2) + 2;
        if gcd(a, value) != 1 {
            return false;
        }
        if pow_mod(a, value - 1, value) != 1 {
            return false;
        }
    }
    true
}

#[test]
fn test_is_simple() {

    fn generate_simple_numbers(max_value: u64) -> Vec<u64> {
        let mut src = vec![true; max_value as usize + 1];
        let mut dst = Vec::new();
        for i in 2 .. max_value as usize + 1 {
            if src[i] {
                let mut ind = i * i;
                while ind <= max_value as usize {
                    src[ind] = false;
                    ind += i;
                }
                dst.push(i as u64);
            }
        }
        dst
    }

    for value in generate_simple_numbers(1000000) {
        assert_eq!(is_simple(value), true);
    }
    assert_eq!(is_simple(10), false);
    assert_eq!(is_simple(169), false);
    assert_eq!(is_simple(8505), false);
    assert_eq!(is_simple(83521), false);
    assert_eq!(is_simple(34012224), false);
    assert_eq!(is_simple(39916800), false);
}