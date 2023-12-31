fn main() {
    let mut inpt = input();
    let m = inpt.next().unwrap();

    println!("{:?}", greedy_guess(m.0, m.1));
}

fn greedy_guess(input: &str, mut numbers: impl Iterator<Item = usize>) -> u32 {
    // Early exit condition
    let Some(n) = numbers.next() else { return 1 };
    if n > input.len() { return 0 }

    let numbers = numbers.collect::<Vec<usize>>();
    println!("recursion: input: {:?}, {:?} + numbers: {:?}", input, n, numbers);
    let input = input.trim_start_matches('.');

    let mut out = 0;

    // Rewrite cond. Should be if input slice exists & slice doesn't contain a dot
    for i in 0..input.len()-n+1 {
        if input.get(i..i+n)
            .and_then(|s| (!s.contains('.')).then_some(()))
            .is_some() {
                if input.len() > i + n { // Going for more
                    out += greedy_guess(&input[i+n+1..], numbers.clone().into_iter())
                } else {
                    out += if numbers.is_empty() { 1 } else { 0 } // Still matches
                }
            }
    }

    println!("Ascending: {:?}", out);
    out
}

fn input() -> impl Iterator<Item = (&'static str, impl Iterator<Item = usize>)> {
    // let input = include_str!("../input");
    let input = "?###???????? 3,2,1";

    input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(s, m)| (s, m.split(',').map(str::parse::<usize>).map(Result::unwrap)))
}
