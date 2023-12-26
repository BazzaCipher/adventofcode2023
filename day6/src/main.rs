fn main() {
    let input = input();

    // From funny maths, we find that time of release is half of time max
    let out: Vec<f32> = input.map(|(time, dist)| {
        let (left, right) = quadratic_roots(-1., time as f32, -(dist as f32));

        (right - 1.).ceil() - (left + 1.).floor() + 1.
    }).collect();

    println!("{:?}", out.iter().product::<f32>());
}

fn input() -> impl Iterator<Item = (u32, u32)> {
    let input = include_str!("../input");

    let [times, dists] = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .collect::<Vec<_>>()[..2] else { unreachable!() };
    times.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .zip(dists.split_whitespace()
             .map(str::parse)
             .map(Result::unwrap))
}

fn quadratic_roots(a: f32, b: f32, c: f32) -> (f32, f32) {
    let doublea = 2. * a;
    let discr = b.powi(2) - 4. * a * c;

    assert!(discr > 0., "Discriminant less than zero");

    let mid = discr.sqrt() / doublea.abs();

    (-b/doublea - mid, -b/doublea + mid)
}
