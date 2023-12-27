#![feature(iter_map_windows)]

use itertools::multipeek;

fn main() {
    let input = input();

    let out = input.into_iter()
        .map(|x| build(&x))
        .map(|x| x.into_iter().map(|y| {
            *y.last().unwrap()
        }).sum::<i64>()).sum::<i64>();

    println!("{:?}", out);
}

fn input() -> Vec<Vec<i64>> {
    let input = include_str!("../input");

    input.lines()
        .map(|line|
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
            )
        .collect()
}

fn build(set: &[i64]) -> Vec<Vec<i64>> {
    let mut out: Vec<Vec<i64>> = Vec::new();
    out.push(set.iter().rev().copied().collect());

    let mut last = &out[0];
    while last.iter().any(|&s| s != 0) {
        out.push(last.iter().map_windows(|[a, b]| *b - *a).collect());
        last = out.last().unwrap();
    }

    out
}

