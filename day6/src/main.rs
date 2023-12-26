use num_traits::Float;
use num_bigfloat::BigFloat;

fn main() {
    let (time, dist) = input();
    println!("{:?}, {:?}", time, dist);

    let bigone = - BigFloat::from_f32(1.);
    // From funny maths, we find that time of release is half of time max
    let (left, right) = quadratic_roots(bigone, BigFloat::from_u64(time), BigFloat::from_f64(-(dist as f64)));
    let out = (right - bigone).ceil() - (left + bigone).floor() + bigone;

    println!("{}", out);
}

fn input() -> (u64, u64) {
    let input = include_str!("../input");
    let input = "Time:      7  15   30
Distance:  9  40  200";

    let [times, dists] = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.chars().filter(|s| s != &' '))
        .map(|num| num.collect::<String>().parse::<u64>().unwrap())
        .collect::<Vec<_>>()[..2] else { unreachable!() };

    (times, dists)
}

fn quadratic_roots(a: BigFloat, b: BigFloat, c: BigFloat) -> (BigFloat, BigFloat) {
    let doublea = BigFloat::from_f32(2.) * a;
    let discr = b.pow(&BigFloat::from_f32(2.)) - BigFloat::from_f32(4.) * a * c;

    assert!(discr > BigFloat::from_f32(0.), "Discriminant less than zero");

    let mid = discr.sqrt() / doublea.abs();

    (-b/doublea - mid, -b/doublea + mid)
}
