use aho_corasick::AhoCorasick;

const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const NUM: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn main() {
    let inpt = include_str!("../input");
    let ac = AhoCorasick::new(&[NUMBERS, NUM].concat()).unwrap();

    let out: u64 = inpt.split_whitespace()
        .map(|line| {
            let line: Vec<usize> = ac.find_overlapping_iter(line).map(|m| m.pattern().as_usize()).collect();

            let a = if line[0] >= 10 { line[0] - 10 } else { line[0] };
            let b = if line[line.len()-1] >= 10 { line[line.len()-1] - 10 } else { line[line.len()-1] };

            (a * 10 + b) as u64

        })
    .sum();
        
    println!("{out}");
}
// fn main() {
//     let inpt = include_str!("../input");
//
//     let out = inpt.split_whitespace()
//         .map(|line| -> u8 {
//             let mut m = line.chars();
//             let a = m.find(|x| x.is_numeric()).expect(line).to_digit(10).unwrap();
//             let b = m.rfind(|x| x.is_numeric()).map(|s| s.to_digit(10).unwrap()).unwrap_or(a);
//             (a * 10 + b) as u8
//         })
//         .inspect(|x| println!("{x}"))
//         .fold(0, |a, x| a + (x as u64));
//
//     println!("{:?}", out);
// }
