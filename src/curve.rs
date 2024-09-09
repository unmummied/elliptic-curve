use crate::prime::*;

pub const NOT_AN_NON_SINGULAR: &str = "not an non-singular...";
pub const NOT_ON_THE_CURVE: &str = "not on the curve...";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    Inf,
    Affine(Num, Num),
}

impl Point {
    pub fn is_inf(&self) -> bool {
        match self {
            Point::Inf => true,
            Point::Affine(_, _) => false,
        }
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Point::Inf => write!(f, "Inf"),
            Point::Affine(x, y) => write!(f, "({}, {})", x, y),
        }
    }
}

pub trait Op {
    fn lhs(&self, y: Num) -> Result<Num, String>;
    fn rhs(&self, x: Num) -> Result<Num, String>;
    fn is_on(&self, point: Point) -> bool;
    fn represent(&self, point: Point) -> Result<Point, &str>;
    fn inv(&self, point: Point) -> Result<Point, &str>;
    fn sum(&self, pt0: Point, pt1: Point) -> Result<Point, String>;
}

pub struct EllipticCurve {
    pub coef1: Num,
    pub coef0: Num,
    pub prime: Num,
}

impl EllipticCurve {
    pub fn new(coef1: Num, coef0: Num, prime: Num) -> Result<Self, String> {
        if !prime.is_prime()? {
            return Err(NOT_A_PRIME.to_string());
        }
        if coef1 == 0 && coef0 == 0 {
            return Err(NOT_AN_NON_SINGULAR.to_string());
        }
        Ok(EllipticCurve {
            coef1: coef1.rem_euclid(prime),
            coef0: coef0.rem_euclid(prime),
            prime,
        })
    }
}
impl std::fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "y^2 = x^3 + {} * x + {} (mod {})",
            self.coef1, self.coef0, self.prime
        )
    }
}
impl Op for EllipticCurve {
    fn lhs(&self, y: Num) -> Result<Num, String> {
        Ok(y.mod_pow(2, self.prime)?)
    }
    fn rhs(&self, x: Num) -> Result<Num, String> {
        Ok(
            (x.mod_pow(3, self.prime)? + (self.coef1 * x).rem_euclid(self.prime) + self.coef0)
                .rem_euclid(self.prime),
        )
    }
    fn is_on(&self, point: Point) -> bool {
        match point {
            Point::Inf => true,
            Point::Affine(x, y) => self.lhs(y) == self.rhs(x),
        }
    }
    fn represent(&self, point: Point) -> Result<Point, &str> {
        if !self.is_on(point) {
            return Err(NOT_ON_THE_CURVE);
        }
        Ok(match point {
            Point::Inf => Point::Inf,
            Point::Affine(x, y) => {
                Point::Affine(x.rem_euclid(self.prime), y.rem_euclid(self.prime))
            }
        })
    }
    fn inv(&self, point: Point) -> Result<Point, &str> {
        if !self.is_on(point) {
            return Err(NOT_ON_THE_CURVE);
        }
        Ok(match point {
            Point::Inf => Point::Inf,
            Point::Affine(x, y) => self.represent(Point::Affine(x, -y))?,
        })
    }
    fn sum(&self, pt0: Point, pt1: Point) -> Result<Point, String> {
        if !self.is_on(pt0) | !self.is_on(pt1) {
            return Err(NOT_ON_THE_CURVE.to_string());
        }
        match (pt0, pt1) {
            (Point::Inf, _) => Ok(self.represent(pt1)?),
            (_, Point::Inf) => Ok(self.represent(pt0)?),
            (Point::Affine(x0, y0), Point::Affine(x1, y1)) => {
                if (x0 - x1).rem_euclid(self.prime) != 0 {
                    let diff = ((y1 - y0) * (x1 - x0).mod_pow(self.prime - 2, self.prime)?)
                        .rem_euclid(self.prime);
                    let x2 = diff.mod_pow(2, self.prime)? - x0 - x1;
                    let y2 = diff * (x2 - x0) + y0;
                    return Ok(self.inv(Point::Affine(x2, y2))?);
                }
                if (y0 + y1).rem_euclid(self.prime) == 0 {
                    return Ok(Point::Inf);
                }
                let diff = ((3 * x0.mod_pow(2, self.prime)? + self.coef1)
                    * (2 * y0).mod_pow(self.prime - 2, self.prime)?)
                .rem_euclid(self.prime);
                let x2 = diff.mod_pow(2, self.prime)? - (2 * x0).rem_euclid(self.prime);
                let y2 = (diff * (x2 - x0)).rem_euclid(self.prime) + y0;
                Ok(self.inv(Point::Affine(x2, y2))?)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_op {
        use super::*;

        #[test]
        fn test_lhs() {
            let curve = EllipticCurve::new(9, 20, 7).unwrap();
            assert_eq!(4, curve.lhs(2).unwrap());
        }
        #[test]
        fn test_rhs() {
            let curve = EllipticCurve::new(9, 20, 7).unwrap();
            assert_eq!(4, curve.rhs(2).unwrap());
        }
        #[test]
        fn test_is_on() {
            let curve = EllipticCurve::new(7, 5, 13).unwrap();
            assert!(curve.is_on(Point::Inf));
            assert!(curve.is_on(Point::Affine(5, 10)));
            assert!(curve.is_on(Point::Affine(9, 2)));
            assert!(curve.is_on(Point::Affine(8, 1)));
            assert!(curve.is_on(Point::Affine(8 + 13, 1 + 13)));
            assert!(!curve.is_on(Point::Affine(9, 1)));

            let curve = EllipticCurve::new(77, 42, 97).unwrap();
            assert!(curve.is_on(Point::Inf));
            assert!(curve.is_on(Point::Affine(22, 68)));
            assert!(curve.is_on(Point::Affine(64, 48)));
            assert!(!curve.is_on(Point::Affine(35, 54)));
            assert!(!curve.is_on(Point::Affine(35, 65)));
            assert!(!curve.is_on(Point::Affine(10, 10)));
        }
        #[test]
        fn test_represent() {
            let curve = EllipticCurve::new(7, 5, 13).unwrap();
            let pt = Point::Affine(3, 1);
            assert_eq!(pt, curve.represent(Point::Affine(3 + 13, 1 + 13)).unwrap());
            assert_eq!(
                pt,
                curve
                    .represent(Point::Affine(3 + 13 * 2, 1 + 13 * 2))
                    .unwrap()
            );
            assert_eq!(
                pt,
                curve
                    .represent(Point::Affine(3 + 13 * -72, 1 + 13 * -25))
                    .unwrap()
            );
        }
        #[test]
        fn test_inv() {
            let curve = EllipticCurve::new(11, 3, 67).unwrap();
            assert_eq!(Point::Inf, curve.inv(Point::Inf).unwrap());
            assert_eq!(
                Point::Affine(22, 46),
                curve.inv(Point::Affine(22, 21)).unwrap()
            );
            assert_eq!(
                Point::Affine(55, 32),
                curve.inv(Point::Affine(55, 35)).unwrap()
            );
            assert_eq!(
                Point::Affine(2, 10),
                curve.inv(Point::Affine(2, 57)).unwrap()
            );
            assert_eq!(
                curve.inv(Point::Affine(2, 57)).unwrap(),
                curve.inv(Point::Affine(-65, -10)).unwrap()
            );
        }
        #[test]
        fn test_sum() {
            let curve = EllipticCurve::new(23, 9, 47).unwrap();
            assert_eq!(
                Point::Affine(15, 43),
                curve
                    .sum(Point::Affine(13, 22), Point::Affine(6, 38))
                    .unwrap()
            );
        }
    }
}
