pub const NOT_AN_NON_NEG: &str = "not an non-negative integer...";
pub const NOT_A_POS: &str = "not a positive integer...";
pub const NOT_A_PRIME: &str = "not a prime number...";

pub type Num = i32;

pub trait Prime<T> {
    fn is_prime(&self) -> Result<bool, &str>;
    fn prime_factors(&self) -> Result<Vec<(T, T)>, &str>;
    #[allow(dead_code)]
    fn is_prime_pow(&self) -> Result<bool, &str>;
    fn inner_gcd(&self, max: Self) -> Self;
    fn gcd(&self, other: Self) -> Self;
}
pub trait Field<T> {
    fn mod_pow(&self, exp: Self, modulo: Self) -> Result<T, &str>;
    fn qr_mod_prime(&self) -> Result<Vec<T>, String>;
    fn legendre(&self, prime: Self) -> Result<T, String>;
}

impl Prime<Num> for Num {
    fn is_prime(&self) -> Result<bool, &str> {
        if *self <= 0 {
            return Err(NOT_A_POS);
        }
        if *self == 1 {
            return Ok(false);
        }
        if *self == 2 {
            return Ok(true);
        }
        if self.rem_euclid(2) == 0 {
            return Ok(false);
        }
        let mut odd = 3;
        while odd * odd <= *self {
            if self.rem_euclid(odd) == 0 {
                return Ok(false);
            }
            odd += 2;
        }
        Ok(true)
    }
    fn prime_factors(&self) -> Result<Vec<(Self, Self)>, &str> {
        if self.is_prime()? {
            return Ok(vec![(*self, 1)]);
        }
        if *self == 1 {
            return Ok(vec![]);
        }
        let mut res = Vec::new();
        let mut n = *self;
        let mut p = 2;
        while p * p <= *self {
            let mut e = 0;
            if n.rem_euclid(p) == 0 {
                while n.rem_euclid(p) == 0 {
                    n /= p;
                    e += 1;
                }
                res.push((p, e));
            }
            p += 1;
        }
        if n != 1 {
            res.push((n, 1));
        }
        Ok(res)
    }
    fn is_prime_pow(&self) -> Result<bool, &str> {
        Ok(self.prime_factors()?.len() == 1)
    }
    fn inner_gcd(&self, max: Self) -> Self {
        if *self == 0 {
            return max;
        }
        max.rem_euclid(*self).inner_gcd(*self)
    }
    fn gcd(&self, other: Self) -> Self {
        std::cmp::min(self.abs(), other.abs()).inner_gcd(std::cmp::max(self.abs(), other.abs()))
    }
}
impl Field<Num> for Num {
    fn mod_pow(&self, exp: Self, modulo: Self) -> Result<Self, &str> {
        if modulo < 1 {
            return Err(NOT_A_POS);
        }
        if exp < 0 {
            return Err(NOT_AN_NON_NEG);
        }
        Ok(match (self, exp, modulo) {
            (_, _, 1) => 0,
            (_, 0, _) => 1,
            (0, _, _) => 0,
            _ => {
                let mut res = self.rem_euclid(modulo);
                for _ in 1..exp {
                    res *= self;
                    res = res.rem_euclid(modulo);
                }
                res
            }
        })
    }
    fn qr_mod_prime(&self) -> Result<Vec<Self>, String> {
        if !self.is_prime()? {
            return Err(NOT_A_PRIME.to_string());
        }
        let mut qrs = vec![0, 1];
        for i in 2..*self {
            if i.mod_pow((self - 1) / 2, *self)? == 1 {
                qrs.push(i);
            }
        }
        Ok(qrs)
    }
    fn legendre(&self, prime: Self) -> Result<Self, String> {
        let qrs = prime.qr_mod_prime()?;
        if self.gcd(prime) != 1 {
            return Ok(0);
        }
        Ok(match qrs.contains(self) {
            true => 1,
            false => -1,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_prime {
        use super::*;

        #[test]
        fn test_is_prime() {
            assert!((-5).is_prime().is_err());
            assert!(0.is_prime().is_err());
            assert!(!1.is_prime().unwrap());
            assert!(2.is_prime().unwrap());
            assert!(97.is_prime().unwrap());
        }
        #[test]
        fn test_prime_factors() {
            assert_eq!(vec![(2, 2), (5, 1), (23, 1)], 460.prime_factors().unwrap());
            assert!(0.prime_factors().is_err());
            assert!((-3).prime_factors().is_err());
        }
        #[test]
        fn test_is_prime_pow() {
            assert!((-1).is_prime_pow().is_err());
            assert!(0.is_prime_pow().is_err());
            assert!(!1.is_prime_pow().unwrap());
            assert!(2.is_prime_pow().unwrap());
            assert!(3.is_prime_pow().unwrap());
            assert!(4.is_prime_pow().unwrap());
            assert!(5.is_prime_pow().unwrap());
            assert!(!6.is_prime_pow().unwrap());
            assert!(7.is_prime_pow().unwrap());
            assert!(8.is_prime_pow().unwrap());
            assert!(9.is_prime_pow().unwrap());
            assert!(!10.is_prime_pow().unwrap());
        }
        #[test]
        fn test_gcd() {
            assert_eq!(1, 9.gcd(7));
            assert_eq!(1, (-9).gcd(7));
            assert_eq!(1, 9.gcd(-7));
            assert_eq!(1, (-9).gcd(-7));
            assert_eq!(1, 9.gcd(97));
            assert_eq!(20, 40.gcd(20));
            assert_eq!(20, 20.gcd(40));
            assert_eq!(20, 20.gcd(20));
            assert_eq!(7, 0.gcd(7));
            assert_eq!(0, 0.gcd(0));
            assert_eq!(1, 1.gcd(0));
            assert_eq!(1, 0.gcd(1));
            assert_eq!(1, 1.gcd(1));
            assert_eq!(1, 2.gcd(1));
            assert_eq!(1, 2.gcd(3));
            assert_eq!(2, 2.gcd(4));
        }
    }
    mod test_field {
        use super::*;

        #[test]
        fn test_mod_pow() {
            assert_eq!(1, 2.mod_pow(4, 5).unwrap());
            assert_eq!(4, (-2).mod_pow(5, 6).unwrap());
            assert_eq!(1, 1.mod_pow(1, 2).unwrap());
            assert!(2.mod_pow(10, 0).is_err());
            assert!(3.mod_pow(-2, 9).is_err());
            assert_eq!(1, 0.mod_pow(0, 10).unwrap());
            assert_eq!(0, 0.mod_pow(9, 10).unwrap());
        }
        #[test]
        fn test_qr_mod_prime() {
            assert!((-1).qr_mod_prime().is_err());
            assert!(0.qr_mod_prime().is_err());
            assert!(1.qr_mod_prime().is_err());
            assert_eq!(vec![0, 1], 2.qr_mod_prime().unwrap());
            assert_eq!(vec![0, 1], 3.qr_mod_prime().unwrap());
            assert!(4.qr_mod_prime().is_err());
            assert_eq!(vec![0, 1, 4], 5.qr_mod_prime().unwrap());
            assert!(6.qr_mod_prime().is_err());
            assert_eq!(vec![0, 1, 2, 4], 7.qr_mod_prime().unwrap());
            assert_eq!(
                vec![
                    0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 19, 20, 24, 25, 27, 29, 30, 32,
                    36, 37, 38, 40, 43, 45, 48, 49, 50, 54, 57, 58, 60, 64
                ],
                71.qr_mod_prime().unwrap()
            );
        }
        #[test]
        fn test_legendre() {
            assert!(4.legendre(6).is_err());
            assert_eq!(1, 4.legendre(5).unwrap());
            assert_eq!(-1, 2.legendre(5).unwrap());
            assert_eq!(1, 60.legendre(71).unwrap());
            assert_eq!(-1, 63.legendre(71).unwrap());
        }
    }
}
