fn main() {
    let inpt = input();

    let mut latergalaxy = Vec::new();

    for row in inpt.iter() {
        if row.iter().all(|s| s == &'.') { latergalaxy.push(row.clone()) }
        latergalaxy.push(row.to_vec())
    }

    println!("{:?}", latergalaxy);
    let mut ins = vec![];
    // cannot be arsed for part 1
    (0..inpt[0].len()).for_each(|colind| {
        if inpt.iter().map(|x| x[colind]).all(|s| s == '.') { ins.push(colind) }
    });

    for (i, ind) in ins.iter().enumerate() {
        latergalaxy.iter_mut().for_each(|row|
            row.insert(i + ind, '.')
        );
    }
    println!("{:?}", latergalaxy);

    let galax = find_galaxies(&latergalaxy);
    let mut idx = 0;

    let mut tot: u64 = 0;

    while idx != galax.len() {
        for i in idx..galax.len() {
            tot += dist(&galax[idx], &galax[i]);
        }
        idx += 1;
    }
    println!("{tot}");
}

fn input() -> Vec<Vec<char>> {
    let input = include_str!("../input");

    input.lines()
        .map(|x| x.chars().collect())
        .collect()
}

fn find_galaxies(inpt: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut out = vec![];
    inpt.iter()
        .enumerate()
        .for_each(|(i, row)|
             row.iter()
                .enumerate()
                .for_each(|(j, c)|
                    if c == &'#' {
                        out.push((i, j))
                    }
                )
        );

    out
}

fn dist(&(i, j): &(usize, usize), &(k, l): &(usize, usize)) -> u64 {
    (i as isize - k as isize).abs() as u64 + (j as isize - l as isize).abs() as u64
}
