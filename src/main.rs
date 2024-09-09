mod curve;
mod group;
mod prime;
use curve::*;
use group::*;

fn main() -> Result<(), String> {
    let prime = 5;
    let curve = EllipticCurve::new(1, 1, prime)?;
    println!("Elliptic curve: {}", curve);

    let gene = curve.represent(Point::Affine(73, 24))?;
    let cycle = curve.cyclic_group(gene)?;
    println!("order of curve: {}", curve.order().unwrap());
    println!("             g: {}", gene);
    println!("    order of g: {}", cycle.len());
    for (i, pt) in cycle.iter().enumerate() {
        println!("{:>2}: {}", i, pt);
    }

    Ok(())
}
