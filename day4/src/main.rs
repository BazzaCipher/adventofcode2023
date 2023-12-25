use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    // Card 1 will be at index 0, etc.
    let out: u64 = input.lines()
        .map(|x| x.split_once(':')
             .expect("Can strip prefix 'Card'").1
             .split_once('|')
             .unwrap()
        )
        .map(|(winners, numbers)| {
            let mut win_num: isize = -1;
            let winners: Vec<&str> = winners.split_whitespace().collect();
            for num in numbers.split_whitespace() {
                if winners.contains(&num) { win_num += 1 }
            }
            if win_num >= 0 { 2_u64.pow(win_num as u32) } else { 0 }
        })
        .sum();

    println!("{:?}", out);
}
