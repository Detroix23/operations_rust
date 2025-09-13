// logarithms.rs
// Find the log-n of any x.

/// Compute the log-n of x.
pub fn logarithm(x: f64, n: f64) -> f64 {
    let epsilon: usize = 16;
    logarithm_wide(x, n, epsilon)
}

/// Use basic dichotomy, and takes as limits 0 and x.
fn logarithm_wide(x: f64, n: f64, epsilon: usize) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    } if x == n {
        return 1.0;
    } if x == 1.0 {
        return 0.0;
    }
    
    let mut limit_upper: f64 = x;
    let mut limit_lower: f64 = 0.0_f64;
    let mut found: bool = false;
    let mut i: usize = 0;

    while i < epsilon && !found {
        let guess: f64 = (limit_lower + limit_upper) / 2.0;
        if n.powf(guess) > x {
            limit_upper = guess;
        } else if n.powf(guess) < x {
            limit_lower = guess;
        } else {
            found = true;
        }
        i += 1;
    }
    
    (limit_lower + limit_upper) / 2.0
}