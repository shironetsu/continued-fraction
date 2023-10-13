use davison::ToDecimal;
use rug::{ops::Pow, Rational};

fn main() {
    let p: Rational = 1 - Rational::from((1, 200));
    let e: Rational = p.pow(300);
    println!("{}", e);
    println!("{}", e.to_decimal(30));
}
