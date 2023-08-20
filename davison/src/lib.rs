use rug::{Rational, Integer};

pub trait FromPartialQuotients<T> {
    fn from_partial_quotients(partial_quotients: Vec<T>) -> Rational;
}
pub trait ToPartialQuotients<T> {
    fn to_partial_quotients(self) -> Vec<T>;
}

impl ToPartialQuotients<Integer> for Rational {
    fn to_partial_quotients(self) -> Vec<Integer> {
        let mut partial_quotients = Vec::<Integer>::new();
        let mut r = self;
        loop {
            let a = r.clone().floor().numer().clone();
            r -= a.clone();
            partial_quotients.push(a);
            if r == 0 {
                break;
            }
            r = r.recip();
        }
        partial_quotients
    }
}

macro_rules! impl_from_partial_quotients {
    ($t: ty) => {
        impl FromPartialQuotients<$t> for Rational {
            fn from_partial_quotients(partial_quotients: Vec<$t>) -> Rational {
                let mut p = Integer::from(1);
                let mut q = Integer::from(0);
                let mut r = Integer::from(0);
                let mut s = Integer::from(1);
                for a in partial_quotients.iter() {
                    let p1 = a * p.clone() + q;
                    let q1 = p.clone();
                    let r1 = a * r.clone() + s;
                    let s1 = r.clone();
                    p = p1;
                    q = q1;
                    r = r1;
                    s = s1;
                }
                Rational::from((p, r))
            }
        }
    };
}

impl_from_partial_quotients!(Integer);
impl_from_partial_quotients!(i32);
impl_from_partial_quotients!(i64);
impl_from_partial_quotients!(i128);

#[cfg(test)]
mod tests {
    use super::*;
    use rug::{Rational, Integer};

    #[test]
    fn from_partial_quotients_1() {
        let r = Rational::from_partial_quotients(vec![1]);
        assert_eq!(r, Rational::from(1));
    }

    #[test]
    fn from_partial_quotients_2() {
        let r = Rational::from_partial_quotients(vec![3,7,15,1]);
        assert_eq!(r, Rational::from((355, 113)));
    }

    #[test]
    fn to_partial_quotients_1() {
        let p = Rational::from((34, 21)).to_partial_quotients();
        assert_eq!(p,
        vec![
            Integer::from(1),
            Integer::from(1),
            Integer::from(1),
            Integer::from(1),
            Integer::from(1),
            Integer::from(1),
            Integer::from(2),
        ])
    }
}