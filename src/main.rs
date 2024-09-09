mod curve;
mod group;
mod prime;
use curve::*;
use group::*;
use prime::Prime;

fn main() -> Result<(), String> {
    let prime = 47;
    let curve = EllipticCurve::new(-1, 0, prime)?;
    println!("Elliptic curve: {}", curve);

    let gene = curve.represent(Point::Affine(0, 0))?;
    let cycle = curve.cyclic_group(gene)?;
    println!("order of curve: {}", curve.order().unwrap());
    println!("             g: {}", gene);
    println!("    order of g: {}", cycle.len());
    for (i, pt) in cycle.iter().enumerate() {
        println!("{:>2}: {}", i, pt);
    }

    for p in 2..100 {
        if p.is_prime()? {
            let curve = EllipticCurve::new(-1, 0, p).unwrap();
            println!(
                "{:>2}, {:>2}, {:?}",
                p,
                curve.order()?,
                curve.decomposition()?
            );
        }
    }

    Ok(())
}
