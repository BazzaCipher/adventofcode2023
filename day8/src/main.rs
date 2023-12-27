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
    current_state: Weak<&'static str>,
}

impl Automaton {
    fn new(map: &'static str) -> Automaton {
        let mut states = BTreeMap::new();
        
        map.lines()
            .for_each(|line| {
                let [s, a, b] = line.split(|c: char| !c.is_alphabetic())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()[..3] else { unreachable!() };
                let a = Automaton::get_key_stub(&mut states, a);
                let b = Automaton::get_key_stub(&mut states, b);
                states.insert(Rc::new(s), (a, b));
            });

        let current_state = Automaton::get_key_stub(&mut states, "AAA");
        Automaton { states, current_state }
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
    
    fn step(&mut self, stepdir: Direction) -> Weak<&str> {
        let (left, right) = self.states
            .get(&self.current_state.upgrade().unwrap())
            .unwrap();

        self.current_state = match stepdir {
            Direction::Left => left.clone(),
            Direction::Right => right.clone(),
        };
        self.current_state.clone()
    }
}

fn main() {
    let (dirs, maps) = input();
    let mut auton = Automaton::new(maps);

    let out = dirs.chars()
        .cycle()
        .map(|dir| {
            let dir = match dir {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!()
            };
            *auton.step(dir).upgrade().unwrap() == "ZZZ"
        })
        .position(|s| s);

    println!("{:?}", out.unwrap() + 1);
}

fn input() -> (&'static str, &'static str) {
    let input = include_str!("../input");

    input.split_once("\n\n").unwrap()
}
