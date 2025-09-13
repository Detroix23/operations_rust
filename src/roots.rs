// roots.rs
// Compute n-roots, using dichotomy.

/// Compute the n-root of x.
pub fn root(x: f64, n: f64) -> f64 {
    let epsilon: usize = 16;
    root_wide(x, n, epsilon)
}

/// Use basic dichotomy, and takes as limits 0 and x.
fn root_wide(x: f64, n: f64, epsilon: usize) -> f64 {
    if x < 0.0 && n % 2.0 == 0.0 {
        panic!("(X) - Can't handle a pair root of a negative number ({n}-root({x}))");
    } if x == 0.0 {
        return 0.0;
    } if x == 1.0 {
        return 1.0;
    }
    
    // Apply the sign to the upper limit, and then to the result.
    let sign_x: i8 = (x / x.abs()) as i8;
    let signed_x: f64 = sign_x as f64 * x;
    let mut limit_upper: f64 = if x > 4.0 {signed_x / 2.0} else {signed_x};
    let mut limit_lower: f64 = 0.0_f64;
    let mut found: bool = false;
    let mut i: usize = 0;

    while i < epsilon && !found {
        let guess: f64 = (limit_lower + limit_upper) / 2.0;
        if guess.powf(n) > signed_x {
            limit_upper = guess;
        } else if guess.powf(n) < signed_x {
            limit_lower = guess;
        } else {
            found = true;
        }
        i += 1;
    }
    
    sign_x as f64 * (limit_lower + limit_upper) / 2.0
}