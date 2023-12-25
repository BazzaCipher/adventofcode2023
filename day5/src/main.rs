#![feature(iter_array_chunks)]

use std::collections::BTreeMap;
use std::time::Instant;
use std::rc::{Weak, Rc};

type Range = (u64, u64);

#[derive(Debug)]
struct BijectiveMap {
    // Source, destination ranges
    range_maps: Vec<Rc<(Range, Range)>>,
    search_tree: BTreeMap<u64, Weak<(Range, Range)>>
}

impl BijectiveMap {
    fn new(input: &str) -> BijectiveMap {
        let maps = input.lines()
            .map(|line| match line
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<u64>>()[..3] {
                    [dest, source, range] => 
                        Rc::new(((source, source+range-1), (dest, dest+range-1))),
                    _ => unreachable!()
            })
            .collect();

        let bt = BijectiveMap::build_btree(&maps);

        BijectiveMap { 
            range_maps: maps,
            search_tree: bt,
        }
    }

    fn get(&self, key: u64) -> u64 {
        let lower = self.search_tree
            .range(..=key)
            .next_back()
            .map(|x| x.1.upgrade().expect("Cannot get reference to value?"));
        let upper = self.search_tree
            .range(key..)
            .next()
            .map(|x| x.1.upgrade().expect("Cannot get reference to value?"));

        if let (Some(u), Some(l)) = (upper, lower) {
            if u == l {
                return l.1.0 + key - l.0.0
            }
        }
        key
    }

    fn build_btree(map: &Vec<Rc<(Range, Range)>>) -> BTreeMap<u64, Weak<(Range, Range)>> {
        let mut tree = BTreeMap::new();

        for pair in map {
            let ((srclow, srchigh), _) = **pair;
            tree.insert(srclow, Rc::downgrade(pair));
            tree.insert(srchigh, Rc::downgrade(pair));
        }

        tree
    }
}

fn input() -> impl Iterator<Item = (&'static str, &'static str)> {
    let input = include_str!("../input");

    input.split_terminator("\n\n")
        .map(|x| x.split_once(':').unwrap())
}

fn main() {
    let mut input = input();

    let initseeds = input
        .next()
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .array_chunks::<2>()
        .flat_map(|[left, right]| left..left+right);
    
    let searchchain: Vec<BijectiveMap> = input
            .map(|(_, block)| BijectiveMap::new(block.strip_prefix('\n').unwrap()))
            .collect();

    let before = Instant::now();
    println!("before");
    let mut out: u64 = initseeds.into_iter()
        .map(|mut seed| {
             for chain in searchchain.iter() {
                 seed = chain.get(seed)
             }
             seed
        })
        .min()
        .unwrap();
    println!("This is how long it took to create and collect the search chains: {:?}", before.elapsed());

    println!("{:?}", out);
}
