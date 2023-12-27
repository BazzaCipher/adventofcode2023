use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

#[derive(Clone)]
enum Direction {
    Left,
    Right
}

///
/// A FSA is much more efficient, however I cannot find a proper implementation
/// that is well supported and documented. Therefore I will use a standard
/// BTreeMap implementation
///
/// Rc was used for maximum compatability if the world map was to be allowed
/// to change/decisions removed or changed
///
#[derive(Debug)]
struct Automaton {
    states: BTreeMap<Rc<&'static str>, (Weak<&'static str>, Weak<&'static str>)>,
    inital_states: Vec<Weak<&'static str>>,
}

impl Automaton {
    fn new(map: &'static str) -> Automaton {
        let mut states = BTreeMap::new();
        let mut inital_states = Vec::new();
        
        map.lines()
            .for_each(|line| {
                let [s, a, b] = line.split(&['=', ' ', ',', '(', ')'])
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()[..3] else { unreachable!() };
                let a = Automaton::get_key_stub(&mut states, a);
                let b = Automaton::get_key_stub(&mut states, b);
                
                let rcs = Rc::new(s);
                let refrcs = Rc::downgrade(&rcs);
                
                states.insert(rcs, (a, b));
                if s.ends_with('A') {
                    inital_states.push(refrcs)
                }
            });

        Automaton { states, inital_states }
    }

    fn initial_states(&self) -> &Vec<Weak<&'static str>> {
        &self.inital_states
    }

    // Gets a Weak to the key in the states, but otherwise inserts a stub
    fn get_key_stub(
        states: &mut BTreeMap<Rc<&'static str>, (Weak<&'static str>, Weak<&'static str>)>,
        key: &'static str
    ) -> Weak<&'static str> {
        match states.get_key_value(&key) {
            None => {
                let newkey = Rc::new(key);
                let refkey = Rc::downgrade(&newkey);
                states.insert(newkey, (Weak::new(), Weak::new()));
                refkey
            },
            Some((k, _)) => Rc::downgrade(k)
        }
    }
    
    fn step_all(&self, states: impl Iterator<Item = Weak<&'static str>>, stepdir: Direction)
        -> impl Iterator<Item = Weak<&str>> {
        states.map(move |state| self.step(&state, &stepdir))
    }

    fn step(&self, state: &Weak<&str>, stepdir: &Direction) -> Weak<&str> {
        let (left, right) = self.states
            .get(&state.upgrade().unwrap())
            .unwrap();

        match stepdir {
            Direction::Left => left.clone(),
            Direction::Right => right.clone(),
        }
    }
}

fn main() {
    let (dirs, maps) = input();
    let auton = Automaton::new(maps);
    let initial = auton.initial_states().into_iter();

    let directions = dirs.chars()
        .cycle()
        .map(|dir| {
            match dir {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!()
            }
        });

    let lengths = initial.map(|mut start| {
        let mut tot = 0;
        let mut dirs = directions.clone();
        let mut start = start.clone();

        while !start.upgrade().unwrap().ends_with('Z') {
            tot += 1;
            start = auton.step(&start, &dirs.next().unwrap());
        }

        tot
    }).collect::<Vec<_>>();
    
    println!("{:?}", lcmm(lengths));
}

fn input() -> (&'static str, &'static str) {
    let input = include_str!("../input");

    input.split_once("\n\n").unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcmm(inpt: Vec<u64>) -> u64 {
    inpt.into_iter().reduce(|a, x| lcm(a, x)).unwrap()
}

