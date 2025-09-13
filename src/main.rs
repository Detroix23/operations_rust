// main.rs
// Realtime tests.

mod roots;
mod logarithms;
mod exponents;

fn main() {
    println!("# Operations.");

    println!("\n## Roots.");
    println!("{}", roots::root(25.0, 2.0)); 
    println!("{}", roots::root(1.0, 2.0));  
    println!("{}", roots::root(0.0, 2.0));  
    println!("{}", roots::root(36.0, 2.0)); 
    println!("{}", roots::root(27.0, 3.0)); 
    println!("{}", roots::root(10.0, 3.0)); 
    println!("{}", roots::root(2.0, 2.0)); 
    println!("{}", roots::root(1.5, 2.0));
    println!("{}", roots::root(-27.0, 2.2));


    println!("\n## Logarithms.");
    println!("{}", logarithms::logarithm(100.0, 10.0));
    println!("{}", logarithms::logarithm(10.0, 10.0));
    println!("{}", logarithms::logarithm(1.0, 10.0));
    println!("{}", logarithms::logarithm(2.0, 10.0));
    println!("{}", logarithms::logarithm(64.0, 2.0));
    println!("{}", logarithms::logarithm(32.0, 2.0));
    println!("{}", logarithms::logarithm(48.0, 2.0));

    println!("\n## Powers.");
    println!("{}", exponents::power(10.0, 2));
    println!("{}", exponents::power(100.0, 2));
    println!("{}", exponents::power(322.0, 3));
    println!("{}", exponents::power(2.0, -2));
    println!("{}", exponents::power(45.0, -3));
    println!("{}", exponents::power(10.0, 1));
    println!("{}", exponents::power(0.0, 2));
    println!("{}", exponents::power(roots::root(25.0, 2.0), 2));
    println!("{}", exponents::power(roots::root(1.5, 2.0), 2));


}