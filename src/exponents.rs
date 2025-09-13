// exponents.rs
// Compute a to the power of n.

/// Compute a exponent n.
pub fn power(a: f64, n: i32) -> f64 {
    fast_handle(a, n)
}

/// Handle negative powers.
fn fast_handle(a: f64, n: i32) -> f64 {
    if n > 0 {
        return fast(a, n);
    } else if n < 0 {
        return 1.0 / fast(a, -n);
    } else {
        return 0.0;
    }

}

/// Using the fast exponentiation, recursion.
fn fast(a: f64, n: i32) -> f64 {
    if a == 0.0 && n == 0 {
        // panic!("(X) - Undefined result (0 ^ 0).")
    } else if a == 0.0 {
        return 0.0;
    } else if n == 0 {
        return 1.0;
    } 

    if n % 2 == 0 {
        let tmp: f64 = fast(a, n / 2);
        return tmp * tmp;
    } else {
        let tmp: f64 = fast(a, (n - 1) / 2);
        return a * tmp * tmp;
    }
}