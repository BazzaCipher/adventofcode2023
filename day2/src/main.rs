fn main() {
    let input = include_str!("../input");

    let out: u32 = input.split("\r\n")
        .map(|line| {
            let gameid_sets: Vec<_> = line.split(":").take(2).collect();

            if is_possible(gameid_sets[1]) { read_gameid(gameid_sets[0]) } else { 0 }
        })
        .sum();

    println!("{out}");
}

fn read_gameid(fixed_str: &str) -> u32 {
    fixed_str.strip_prefix("Game ").unwrap().parse().expect("Was valid")
}

fn is_possible(all_sets: &str) -> bool {
    all_sets.split(";")
        .all(|set| set.

}
