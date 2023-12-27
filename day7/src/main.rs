use std::collections::{HashMap, BTreeMap};
use std::cmp::Ordering;
use lazy_static::lazy_static;

const CARDS: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

lazy_static! {
    static ref CARDORDER: BTreeMap<char, usize> = {
        let mut cardorder = BTreeMap::new();
        for (i, card) in CARDS.iter().enumerate() {
            cardorder.insert(*card, i);
        }
        cardorder
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Kind {
    FiveofaKind = 0,
    FourofaKind,
    FullHouse,
    ThreeofaKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Debug)]
struct Hand {
    inner: [char; 5],
    kind: Option<Kind>,
}

impl Hand {
    fn new(input: &[char; 5]) -> Hand {
        Hand {
            inner: *input,
            kind: None
        }
    }
}

fn main() {
    let mut input = input();

    input.sort_unstable_by(|(carda, _), (cardb, _)| carda.partial_cmp(cardb).unwrap());
    let out = input.into_iter()
        .enumerate()
        .fold(0_u64, |acc, (ind, (_, bid))| acc + (ind + 1) as u64 * bid);

    println!("{:?}", out);
}

fn input() -> Vec<(Hand, u64)> {
    let input = include_str!("../input");

    input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(i, bid)| (str_to_card(i), bid.parse::<u64>().unwrap()))
        .collect()
}

fn compare_card_letters(a: &Hand, b: &Hand) -> Ordering {
    fn get_order(c: &char) -> usize {
        *CARDORDER.get(c).unwrap()
    }

    (0..a.inner.len())
        .map(|i| get_order(&a.inner[i]) as isize - get_order(&b.inner[i]) as isize)
        .find(|diff| diff != &0)
        .map(|diff| 
             if diff > 0 {
                 Ordering::Less
            } else {
                Ordering::Greater
            })
        .unwrap_or(Ordering::Equal)
}

impl Hand {
    fn kind(&self) -> Kind {
        *self.kind.as_ref().unwrap_or({
            let (jacks, mut matches) = occurrences(self);
            matches.sort_unstable_by(|a, b| b.cmp(a));
            match matches.get_mut(0) {
                Some(a) => *a += jacks,
                _ => matches.push(jacks) // Equivalent; For decoupling purposes
            }
            match matches[..] {
                [_] => &Kind::FiveofaKind,
                [a, b] if a == 4 || b == 4 => &Kind::FourofaKind,
                [_, _] => &Kind::FullHouse,
                [a, b, c] if a == 3 || b == 3 || c == 3 => &Kind::ThreeofaKind,
                [_, _, _] => &Kind::TwoPair,
                [_, _, _, _] => &Kind::OnePair,
                _ => &Kind::HighCard
            }
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let skind = self.kind() as isize;
        let okind = other.kind() as isize;

        match okind - skind {
            a if a < 0 => Some(Ordering::Less),
            a if a > 0 => Some(Ordering::Greater),
            _ => Some(compare_card_letters(self, other))
        }
    }
}

fn occurrences(v: &Hand) -> (usize, Vec<usize>) {
    let mut m: HashMap<&char, usize> = HashMap::new();
    let mut jacks = 0;
    let cards = &v.inner;

    for x in cards {
        if *x == 'J' {
            jacks += 1;
            continue
        }
        *m.entry(x).or_default() += 1;
    }

    (jacks, m.into_values().collect())
}

fn str_to_card(input: &str) -> Hand {
    let mut out = [' '; 5];

    input.chars()
        .zip(out.iter_mut())
        .for_each(|(b, ptr)| *ptr = b);

    Hand::new(&out)
}
