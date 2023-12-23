use std::collections::HashMap;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, EnumString)]
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
    let max: HashMap<Colour, u32> = HashMap::from([
        (Colour::Red, 12),
        (Colour::Blue, 14),
        (Colour::Green, 13),
    ]);

    let out: u32 = input.lines()
        .map(|line| {
            let gameid_sets: Vec<_> = line.split(':').take(2).collect();

            if is_possible(gameid_sets[1], &max) { read_gameid(gameid_sets[0]) } else { 0 }
        })
        .sum();

    println!("{out}");
}

fn read_gameid(fixed_str: &str) -> u32 {
    fixed_str.strip_prefix("Game ").unwrap().parse().expect("Was valid")
}

fn is_possible(all_sets: &str, max: &HashMap<Colour, u32>) -> bool {
    all_sets.split(';')
        .all(|set| {
            set.split(',')
                .map(num_colour)
                .all(|(count, colour)| max.get(&colour).unwrap() >= &count)
        })

}

fn num_colour(input: &str) -> (u32, Colour) {
    let words: Vec<_> = input.split_whitespace().take(2).collect();
    (words[0].parse().unwrap(), Colour::from_str(words[1]).unwrap())
}
