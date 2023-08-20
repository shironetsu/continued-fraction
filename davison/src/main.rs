/* Rust translation of http://www.numbertheory.org/gnubc/davison */
use nalgebra::Matrix2;
use davison::{FromPartialQuotients};
use rug::Rational;
trait IsNonNegative {
    fn is_non_negative(&self) -> bool;
}

impl IsNonNegative for Matrix2<i128> {
    fn is_non_negative(&self) -> bool {
        self.iter().all(|&x| x >= 0)
    }
}

fn nprod(l: i128, m: i128) -> (Matrix2<i128>, i128) {
    let mut a = Matrix2::identity();
    let mut k = 0;
    loop {
        let t = (2 * k + 1) * m;
        a *= Matrix2::new(t + l, t, t, t - l);
        a.reduce();

        if a.is_non_negative() {
            break;
        }
        k += 1
    }
    (a, k)
}

fn gcd(m: i128, n: i128) -> i128 {
    let mut a = m.abs();
    if n == 0 {
        return a;
    }
    let mut b = n.abs();
    let mut c = a % b;
    while c > 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

trait Reduce {
    fn reduce(&mut self);
}

impl Reduce for Matrix2<i128> {
    fn reduce(&mut self) {
        let g = self.iter().cloned().reduce(|acc, x| gcd(acc, x)).unwrap();
        *self /= g;
    }
}

fn raney(
    a: &mut Matrix2<i128>,
    partial_quotients: &mut Vec<i128>,
    r: &mut i128,
    l: &mut i128,
    flagl: &mut bool,
    flagr: &mut bool,
    count: &mut i128,
) {
    let mut k = 0;
    let mut b = a.clone();
    loop {
        let mut i = 0;
        while b.index((0,0)) >= b.index((1, 0)) && b.index((0, 1)) >= b.index((1, 1)) {
            b = Matrix2::new(1, -1, 0, 1) * b;
            i = i + 1;
        }
        if i > 0 {
            *r += i;
            *flagr = true;
            if *flagl && *flagr {
                *flagl = false;
                partial_quotients.push(*l);
                *l = 0;
                *count += 1;
            }
            k += 1;
        }
        let mut j = 0;
        while b.index((1, 0)) >= b.index((0, 0)) && b.index((1, 1)) >= b.index((0, 1)) {
            b = Matrix2::new(1, 0, -1, 1) * b;
            j += 1;
        }
        if j > 0 {
            *l += j;
            *flagl = true;
            if *flagr && *flagl {
                *flagr = false;
                partial_quotients.push(*r);
                *r = 0;
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
    count: i128,
    partial_quotients: Vec<i128>,
}

fn davison(p: i128, q: i128, n: i128) -> Davison {
    let mut partial_quotients = Vec::<i128>::new();
    let mut count = 0;
    let mut flagr = false;
    let mut flagl = false;
    let mut r = 0;
    let mut l = 0;

    let (mut a, k) = nprod(p, q);
    let mut i = k;
    let j = k + n;
    while i <= j {
        if i > k {
            let t = (2 * i + 1) * q;
            a *= Matrix2::new(t + p, t, t, t - p);
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
    let p = 3;
    let q = 2;
    let n = 100;
    let d = davison(p, q, n);
    println!("e^({}/{})", p, q);
    println!("{:?}", d.partial_quotients);
    //println!("{}", d.count);
    for n in 1..=10{
        let r = Rational::from_partial_quotients(d.partial_quotients.iter().cloned().take(n).collect()).recip();
        println!("{} {} {}", n, r, r.to_f64());
    }
}
