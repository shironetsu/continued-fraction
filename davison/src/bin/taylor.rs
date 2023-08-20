use rug::{Rational, Integer, ops::Pow, Complete};



fn main(){
    let mut r = Rational::from(0);
    let x = Rational::from((-3,2));
    for n in 0..=10 {
        r += x.clone().pow(n) / Integer::factorial(n).complete();
        println!("{} {} {}", n, r, r.to_f64());
    }
}