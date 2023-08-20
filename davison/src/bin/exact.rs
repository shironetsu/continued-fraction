use rug::{Rational, Integer, ops::Pow};
use davison::{ToPartialQuotients};

fn main(){
    let p: Rational = 1 - Rational::from((1, 200));
    let e: Rational = p.pow(300);
    println!("{}", e.to_f64());
    let q = e.to_partial_quotients();
    for (i, a) in q.iter().enumerate().take(100){
        println!("{} {}", i ,a);
    }
}