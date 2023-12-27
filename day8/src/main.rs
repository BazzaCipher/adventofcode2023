use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

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

    fn get_key(&self, key: &'static str) -> Weak<&'static str> {
        Rc::downgrade(&self.states.get_key_value(&key).map(|(k, _)| k).unwrap())
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
    
    fn step(&self, states: &[Weak<&str>], stepdir: Direction) -> Vec<Weak<&str>> {
        states.iter().map(move |state| {
            let (left, right) = self.states
                .get(&state.upgrade().unwrap())
                .unwrap();

            match stepdir {
                Direction::Left => left.clone(),
                Direction::Right => right.clone(),
            }
        }).collect()
    }
}

fn main() {
    let (dirs, maps) = input();
    let auton = Automaton::new(maps);
    let initial = auton.inital_states.clone();
    let length = initial.len();

    let out = dirs.chars()
        .cycle()
        .map(|dir| {
            match dir {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!()
            }
        })
        .scan(initial, |acc, dir| {
            *acc = auton.step(acc, dir);
            Some(acc.iter().fold(length, |acc, x| acc - if x.upgrade().unwrap().ends_with('Z') { 1 } else { 0 }))
        })
        // .inspect(|s| println!("{:?}", s))
        .position(|x| x == 0);

    println!("{:?}", out.unwrap() + 1);
}

fn input() -> (&'static str, &'static str) {
    let input = include_str!("../input");

    input.split_once("\n\n").unwrap()
}



