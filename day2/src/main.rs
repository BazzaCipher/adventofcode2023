use std::collections::HashMap;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Clone, Hash, PartialEq, Eq, EnumString)]
enum Colour {
    #[strum(serialize="red")]
    Red,
    #[strum(serialize="blue")]
    Blue,
    #[strum(serialize="green")]
    Green
}

fn main() {
    let input = include_str!("../input");
    let max: HashMap<Colour, u64> = HashMap::from([
        (Colour::Red, 12),
        (Colour::Blue, 14),
        (Colour::Green, 13),
    ]);

    let out: u64 = input.lines()
        .map(|line| {
            let game = line.split(':').nth(1).unwrap();

            game.split_terminator(&[';', ','])
                .map(num_colour)
                .fold(HashMap::new(), |mut acc, e| {
                    // Too lazy for complicated efficiency
                    acc.insert(e.1.clone(), u64::max(e.0, acc.get(&e.1).map(|e| *e).unwrap_or_default()));
                    acc
                })
                .values()
                .copied()
                .collect()
        })
        .map(|values: Vec<_>| values.into_iter().reduce(|acc, e| acc * e).unwrap())
        .sum();

    println!("{out}");
}

// fn read_gameid(fixed_str: &str) -> u32 {
//     fixed_str.strip_prefix("Game ").unwrap().parse().expect("Was valid")
// }

// fn is_possible(all_sets: &str, max: &HashMap<Colour, u32>) -> bool {
//     all_sets.split(';')
//         .all(|set| {
//             set.split(',')
//                 .map(num_colour)
//                 .all(|(count, colour)| max.get(&colour).unwrap() >= &count)
//         })
//
// }

fn num_colour(input: &str) -> (u64, Colour) {
    let words: Vec<_> = input.split_whitespace().take(2).collect();
    (words[0].parse().unwrap(), Colour::from_str(words[1]).unwrap())
}
