/* Rust translation of http://www.numbertheory.org/gnubc/davison */

fn positivity(a: i128, b: i128, c: i128, d: i128) -> bool {
    a >= 0 && b >= 0 && c >= 0 && d >= 0
}

fn nprod(
    globala: &mut i128,
    globalb: &mut i128,
    globalc: &mut i128,
    globald: &mut i128,
    l: i128,
    m: i128,
) -> i128 {
    *globala = m + l;
    *globalb = m;
    *globalc = m;
    *globald = m - l;
    let mut k = 1;
    for i in 1.. {
        k = i;
        let s = positivity(*globala, *globalb, *globalc, *globald);
        if s {
            break;
        }
        let t = (2 * k + 1) * m;
        let t1 = *globala + *globalb;
        let t2 = *globalc + *globald;
        let temp1 = t * t1;
        let temp2 = t * t2;
        let a1 = temp1 + *globala * l;
        let b1 = temp1 - *globalb * l;
        let c1 = temp2 + *globalc * l;
        let d1 = temp2 - *globald * l;
        *globala = a1;
        *globalb = b1;
        *globalc = c1;
        *globald = d1;
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

fn reduce(a: i128, b: i128, c: i128, d: i128) -> i128 {
    let mut g = gcd(a, b);
    g = gcd(g, c);
    g = gcd(g, d);
    g
}

fn raney(
    partial_quotientds: &mut Vec<i128>,
    globalr: &mut i128,
    globall: &mut i128,
    flagl: &mut bool,
    flagr: &mut bool,
    count: &mut i128,
    globala: &mut i128,
    globalb: &mut i128,
    globalc: &mut i128,
    globald: &mut i128,
) -> i128 {
    let mut k = 0;
    let mut p = *globala;
    let mut q = *globalb;
    let mut r = *globalc;
    let mut s = *globald;
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
                //print!("{},", *globall);
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
                //print!("{},", globalr);
                *globalr = 0;
                *count += 1;
            }
            k = k + 1;
        }
        if (p < r && q > s) || (p > r && s > q) {
            break;
        }
    }
    *globala = p;
    *globalb = q;
    *globalc = r;
    *globald = s;
    k
}

struct Davison {
    count: i128,
    partial_quotients: Vec::<i128>
}

fn davison(l: i128, m: i128, n: i128) -> Davison {
    let mut partial_quotients = Vec::<i128>::new();
    let mut count = 0;
    let mut flagr = false;
    let mut flagl = false;
    let mut globalr = 0;
    let mut globall = 0;

    let mut globala = 0;
    let mut globalb = 0;
    let mut globalc = 0;
    let mut globald = 0;
    let k = nprod(&mut globala, &mut globalb, &mut globalc, &mut globald, l, m);
    let mut g = reduce(globala, globalb, globalc, globald);
    let mut t = 0;
    if g > 1 {
        globala = globala / g;
        globalb = globalb / g;
        globalc = globalc / g;
        globald = globald / g;
    }
    let mut i = k;
    let j = k + n;
    while i <= j {
        if i > k {
            t = (2 * i + 1) * m;
            let t1 = globala + globalb;
            let t2 = globalc + globald;
            let temp1 = t * t1;
            let temp2 = t * t2;
            let a1 = temp1 + globala * l;
            let b1 = temp1 - globalb * l;
            let c1 = temp2 + globalc * l;
            let d1 = temp2 - globald * l;
            globala = a1;
            globalb = b1;
            globalc = c1;
            globald = d1;
        }
        i += 1;
        t = raney(
            &mut partial_quotients,
            &mut globalr,
            &mut globall,
            &mut flagl,
            &mut flagr,
            &mut count,
            &mut globala,
            &mut globalb,
            &mut globalc,
            &mut globald,
        );
        g = reduce(globala, globalb, globalc, globald);
        if g > 1 {
            globala = globala / g;
            globalb = globalb / g;
            globalc = globalc / g;
            globald = globald / g;
        }
    }

    Davison {
        count,
        partial_quotients
    }
}

fn main() {
    let l = 3;
    let m = 2;
    let n = 100;
    let d = davison(l, m, n);
    println!("e^({}/{})", l , m);
    println!("{:?}", d.partial_quotients);
    println!("{}", d.count);
}
