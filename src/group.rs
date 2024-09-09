use crate::curve::*;
use crate::prime::*;

pub trait Group {
    fn order(&self) -> Result<Num, String>;
    fn cyclic_group(&self, generator: Point) -> Result<Vec<Point>, String>;
}

impl Group for EllipticCurve {
    fn order(&self) -> Result<Num, String> {
        let mut ord = 1 + self.prime;
        for i in 0..self.prime {
            ord += self.rhs(i)?.legendre(self.prime)?;
        }
        Ok(ord)
    }
    fn cyclic_group(&self, generator: Point) -> Result<Vec<Point>, String> {
        if !self.is_on(generator) {
            return Err(NOT_ON_THE_CURVE.to_string());
        }
        if generator.is_inf() {
            return Ok(vec![Point::Inf]);
        }
        let mut cycle = vec![self.represent(generator)?];
        loop {
            cycle.push(self.sum(generator, *cycle.last().unwrap())?);
            if cycle.last().unwrap().is_inf() {
                break;
            }
        }
        Ok(cycle)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_group {
        use super::*;

        #[test]
        fn test_order() {
            let curve = EllipticCurve::new(-1, 0, 71).unwrap();
            assert_eq!(72, curve.order().unwrap());
            let curve = EllipticCurve::new(1, 6, 11).unwrap();
            assert_eq!(13, curve.order().unwrap());
            let curve = EllipticCurve::new(57, 97, 199).unwrap();
            assert_eq!(220, curve.order().unwrap());
        }
        #[test]
        fn test_cyclic_group() {
            let curve = EllipticCurve::new(3, 11, 53).unwrap();
            assert_eq!(57, curve.cyclic_group(Point::Affine(9, 5)).unwrap().len());
            assert_eq!(19, curve.cyclic_group(Point::Affine(38, 47)).unwrap().len());
        }
    }
}
