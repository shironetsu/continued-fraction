/* Rust translation of http://www.numbertheory.org/gnubc/davison */
use nalgebra::Matrix2;

trait IsPositive {
    fn is_positive(&self) -> bool;
}

impl IsPositive for Matrix2<i128> {
    fn is_positive(&self) -> bool {
        self.iter().all(|&x| x >= 0)
    }
}

fn nprod(a: &mut Matrix2<i128>, l: i128, m: i128) -> i128 {
    *a = Matrix2::new(m + l, m, m, m - l);
    let mut k = 1;
    for i in 1.. {
        k = i;
        if a.is_positive() {
            break;
        }
        let t = (2 * k + 1) * m;
        *a *= Matrix2::new(t + l, t, t, t - l);
    }
    k - 1
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
    partial_quotientds: &mut Vec<i128>,
    globalr: &mut i128,
    globall: &mut i128,
    flagl: &mut bool,
    flagr: &mut bool,
    count: &mut i128,
    a: &mut Matrix2<i128>,
) {
    let mut k = 0;
    let mut p = *a.index((0, 0));
    let mut q = *a.index((0, 1));
    let mut r = *a.index((1, 0));
    let mut s = *a.index((1, 1));
    loop {
        let mut i = 0;
        while p >= r && q >= s {
            p = p - r;
            q = q - s;
            i = i + 1;
        }
        if i > 0 {
            *globalr += i;
            *flagr = true;
            if *flagl && *flagr {
                *flagl = false;
                partial_quotientds.push(*globall);
                *globall = 0;
                *count += 1;
            }
            k += 1;
        }
        let mut j = 0;
        while r >= p && s >= q {
            r = r - p;
            s = s - q;
            j += 1;
        }
        if j > 0 {
            *globall += j;
            *flagl = true;
            if *flagr && *flagl {
                *flagr = false;
                partial_quotientds.push(*globalr);
                *globalr = 0;
                *count += 1;
            }
            k = k + 1;
        }
        if (p < r && q > s) || (p > r && s > q) {
            break;
        }
    }
    *a = Matrix2::new(p, q, r, s);
}

struct Davison {
    count: i128,
    partial_quotients: Vec<i128>,
}

fn davison(l: i128, m: i128, n: i128) -> Davison {
    let mut partial_quotients = Vec::<i128>::new();
    let mut count = 0;
    let mut flagr = false;
    let mut flagl = false;
    let mut globalr = 0;
    let mut globall = 0;

    let mut a = Matrix2::zeros();
    let k = nprod(&mut a, l, m);
    a.reduce();
    let mut i = k;
    let j = k + n;
    while i <= j {
        if i > k {
            let t = (2 * i + 1) * m;
            a *= Matrix2::new(t + l, t, t, t - l);
        }
        i += 1;
        raney(
            &mut partial_quotients,
            &mut globalr,
            &mut globall,
            &mut flagl,
            &mut flagr,
            &mut count,
            &mut a,
        );
        a.reduce();
    }

    Davison {
        count,
        partial_quotients,
    }
}

fn main() {
    let l = 1;
    let m = 2;
    let n = 100;
    let d = davison(l, m, n);
    println!("e^({}/{})", l, m);
    println!("{:?}", d.partial_quotients);
    println!("{}", d.count);
}
