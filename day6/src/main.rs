use num_traits::{Float, Zero, Signed, cast::FromPrimitive};
use std::ops::Mul;

fn main() {
    let input = input();

    // From funny maths, we find that time of release is half of time max
    let (left, right) = quadratic_roots::<f64>(-1., input.0 as f64, -(input.1 as f64));
    let out: f64 = (right - 1.).ceil() - (left + 1.).floor() + 1.;

    println!("{:?}", out);
}

fn input() -> (u32, u32) {
    let input = include_str!("../input");

    let [times, dists] = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|nums| nums.split_whitespace().map(str::parse::<u32>).map(Result::unwrap).product())
        .collect::<Vec<_>>()[..2] else { unreachable!() };

    (times, dists)
}

fn quadratic_roots<A>(a: A, b: A, c: A) -> (A, A)
where A: Float + Mul<A, Output = A> + Zero + Signed + FromPrimitive
{
    let doublea = a * A::from_i8(2).unwrap();
    let discr = b * b - a * c * A::from_i8(4).unwrap();

    assert!(discr > A::zero(), "Discriminant less than zero");

    let mid = discr.sqrt() / doublea.abs();

    (-b/doublea - mid, -b/doublea + mid)
}

