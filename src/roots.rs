// roots.rs
// Compute n-roots, using dichotomy.

/// Compute the n-root of x.
pub fn root(x: f64, n: f64) -> f64 {
    let epsilon: usize = 16;
    root_wide(x, n, epsilon)
}

/// Use basic dichotomy, and takes as limits 0 and x.
fn root_wide(x: f64, n: f64, epsilon: usize) -> f64 {
    if x < 0.0 {
        return f64::NAN;
    } if x == 0.0 {
        return 0.0;
    } if x == 1.0 {
        return 1.0;
    }
    
    let mut limit_upper: f64 = if x > 4.0 {x / 2.0} else {x};
    let mut limit_lower: f64 = 0.0_f64;
    let mut found: bool = false;
    let mut i: usize = 0;

    while i < epsilon && !found {
        let guess: f64 = (limit_lower + limit_upper) / 2.0;
        if guess.powf(n) > x {
            limit_upper = guess;
        } else if guess.powf(n) < x {
            limit_lower = guess;
        } else {
            found = true;
        }
        i += 1;
    }
    
    (limit_lower + limit_upper) / 2.0
}