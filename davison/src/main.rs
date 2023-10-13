/* Rust re-implementation of http://www.numbertheory.org/gnubc/davison */
use davison::{FromPartialQuotients, ToDecimal};
use nalgebra::Matrix2;
use rug::{Integer, Rational};
trait IsNonNegative {
    fn is_non_negative(&self) -> bool;
}

impl IsNonNegative for Matrix2<Integer> {
    fn is_non_negative(&self) -> bool {
        self.iter().all(|x| *x >= 0)
    }
}

fn nprod(l: Integer, m: Integer) -> (Matrix2<Integer>, Integer) {
    let mut a = Matrix2::identity();
    let mut k = Integer::ZERO;
    loop {
        let t: Integer = (2 * k.clone() + 1) * m.clone();
        a *= Matrix2::new(
            t.clone() + l.clone(),
            t.clone(),
            t.clone(),
            t.clone() - l.clone(),
        );
        a.reduce();

        if a.is_non_negative() {
            break;
        }
        k += 1
    }
    (a, k)
}

trait Reduce {
    fn reduce(&mut self);
}

impl Reduce for Matrix2<Integer> {
    fn reduce(&mut self) {
        let g = self.iter().cloned().reduce(|acc, x| acc.gcd(&x)).unwrap();
        *self /= g;
    }
}

fn raney(
    a: &mut Matrix2<Integer>,
    partial_quotients: &mut Vec<Integer>,
    r: &mut Integer,
    l: &mut Integer,
    flagl: &mut bool,
    flagr: &mut bool,
    count: &mut Integer,
) {
    let mut k = 0;
    let mut b = a.clone();
    loop {
        let mut i = 0;
        while b.index((0, 0)) >= b.index((1, 0)) && b.index((0, 1)) >= b.index((1, 1)) {
            b = Matrix2::new(
                Integer::from(1),
                Integer::from(-1),
                Integer::from(0),
                Integer::from(1),
            ) * b;
            i = i + 1;
        }
        if i > 0 {
            *r += i;
            *flagr = true;
            if *flagl && *flagr {
                *flagl = false;
                partial_quotients.push(l.clone());
                *l = Integer::ZERO;
                *count += 1;
            }
            k += 1;
        }
        let mut j = 0;
        while b.index((1, 0)) >= b.index((0, 0)) && b.index((1, 1)) >= b.index((0, 1)) {
            b = Matrix2::new(
                Integer::from(1),
                Integer::from(0),
                Integer::from(-1),
                Integer::from(1),
            ) * b;
            j += 1;
        }
        if j > 0 {
            *l += j;
            *flagl = true;
            if *flagr && *flagl {
                *flagr = false;
                partial_quotients.push(r.clone());
                *r = Integer::ZERO;
                *count += 1;
            }
            k = k + 1;
        }
        if (b.index((0, 0)) < b.index((1, 0)) && b.index((0, 1)) > b.index((1, 1)))
            || (b.index((0, 0)) > b.index((1, 0)) && b.index((1, 1)) > b.index((0, 1)))
        {
            break;
        }
    }
    *a = b;
    a.reduce();
}

struct Davison {
    count: Integer,
    partial_quotients: Vec<Integer>,
}

fn davison(p: Integer, q: Integer, n: Integer) -> Davison {
    let mut partial_quotients = Vec::<Integer>::new();
    assert!(p != 0 && q != 0, "p and q should be non-zero integers");
    let (p, q) = {
        if (p > 0 && q < 0) || (p < 0 && q > 0) {
            partial_quotients.push(Integer::ZERO);
        }
        (p.abs(), q.abs())
    };
    let mut count = Integer::ZERO;
    let mut flagr = false;
    let mut flagl = false;
    let mut r = Integer::ZERO;
    let mut l = Integer::ZERO;

    let (mut a, k) = nprod(p.clone(), q.clone());
    let mut i = k.clone();
    let j = k.clone() + n;
    while i <= j {
        if i > k {
            let t: Integer = (2 * i.clone() + 1) * q.clone();
            a *= Matrix2::new(t.clone() + p.clone(), t.clone(), t.clone(), t - p.clone());
            a.reduce();
        }
        i += 1;
        raney(
            &mut a,
            &mut partial_quotients,
            &mut r,
            &mut l,
            &mut flagl,
            &mut flagr,
            &mut count,
        );
    }

    Davison {
        count,
        partial_quotients,
    }
}

fn main() {
    let p = -3;
    let q = 2;
    let n = 32;
    let d = davison(Integer::from(p), Integer::from(q), Integer::from(n));
    println!("e^({}/{})", p, q);
    println!(
        "{}",
        d.partial_quotients
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    println!("{}", d.count);

    for n in 1..=10 {
        let r =
            Rational::from_partial_quotients(d.partial_quotients.iter().cloned().take(n).collect());
        let d = r.to_decimal(30);
        println!("{} {} {}", n, r, d);
    }
}
