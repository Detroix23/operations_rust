// lib.rs

mod roots;
mod logarithms;
mod exponents;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        println!("{}", roots::root(25.0, 2.0));
        println!("{}", roots::root(1.0, 2.0));
        println!("{}", roots::root(0.0, 2.0));
        println!("{}", roots::root(36.0, 2.0));
        println!("{}", roots::root(16.0, 2.0));
    }
}
