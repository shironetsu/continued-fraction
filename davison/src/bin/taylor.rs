use rug::{ops::Pow, Complete, Integer, Rational};

fn taylor(x: Rational, order: u32) -> Rational {
    let mut r = Rational::from(0);
    for n in 0..=order {
        r += x.clone().pow(n) / Integer::factorial(n).complete();
    }
    r
}

fn i(n: u32, a: Rational) -> Rational {
    let mut x = Rational::from(0);
    for k in 0..=n {
        x += (-1).pow(n - k) * a.clone().pow(k) / Integer::factorial(k).complete()
    }
    x
}

fn ii(order: u32, a: Rational) -> Rational {
    let mut x = Rational::from(0);
    for k in 0..=order {
        x += (-1).pow(k) * i(k, a.clone());
    }
    x
}

fn main() {
    let x = Rational::from(1);
    let order = 11;
    let q = taylor(x.clone(), order);
    println!("{}", q);

    for order in 0..10 {
        let x = Rational::from(1);
        let r = taylor(-x, order);
        println!("{} {}", order, r);
    }
    //taylor(-1, 10)^{-1} = 44800/16481
    //\int_{0}^1 (1-x+x^2/2+...+x^8/40320) e^x - 1 dx = 44800/16481

    for order in 0..10 {
        let x = ii(order, Rational::from(1));
        println!("{} {}", order, x);
    }
}
