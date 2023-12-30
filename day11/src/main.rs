use std::iter::once;
use std::collections::BTreeSet;

const SCALE: u64 = 1_000_000 - 1;

fn main() {
    let inpt = input();

    let emptyrows = BTreeSet::from_iter(inpt.iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|s| s == &'.'))
        .map(|(i, _)| i));
    let emptycols = BTreeSet::from_iter((0..inpt[0].len()) // Assuming squareness
        .filter(|c|
            (0..inpt.len())
                .enumerate()
                .zip(once(c).cycle())
                .all(|((_, r), c)| inpt[r][*c] == '.')
        ));

    let galaxies = find_galaxies(&inpt);
    let mut idx = 0;
    let mut tot: u64 = 0;

    while idx != galaxies.len() {
        for i in idx..galaxies.len() {
            tot += dist(&galaxies[idx], &galaxies[i]);

            let min = galaxies[idx].0.min(galaxies[i].0);
            let max = galaxies[idx].0.max(galaxies[i].0);
            tot += emptyrows.range(min..=max).count() as u64 * SCALE;

            let min = galaxies[idx].1.min(galaxies[i].1);
            let max = galaxies[idx].1.max(galaxies[i].1);
            tot += emptycols.range(min..=max).count() as u64 * SCALE;
        }
        idx += 1;
    }

    println!("{tot}");
}

fn input() -> Vec<Vec<char>> {
    let input = include_str!("../input");
//     let input = "...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....";

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
