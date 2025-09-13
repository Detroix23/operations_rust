// exponents.rs
// Compute a to the power of n.

/// Compute a exponent n.
pub fn power(a: i64, n: i32) -> i64 {
    fast(a, n)
}

/// Using the fast exponentiation, recursion.
fn fast(a: i64, n: i32) -> i64 {
    if a == 0 && n == 0 {
        panic!("(X) - Undefined result (0 ^ 0).")
    } else if a == 0 {
        return 0;
    } else if n == 0 {
        return 1;
    } 

    if n % 2 == 0 {
        let tmp: i64 = fast(a, n / 2);
        return tmp * tmp;
    } else {
        let tmp: i64 = fast(a, (n - 1) / 2);
        return a * tmp * tmp;
    }
}