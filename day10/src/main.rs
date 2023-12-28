use std::ops::Not;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East  => Self::West,
            Self::West  => Self::East,
        }
    }
}

impl From<Direction> for (isize, isize) {
    // Assume column major, from top left
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => (-1, 0),
            Direction::South => (1 , 0),
            Direction::East  => (0 , 1),
            Direction::West  => (0 ,-1),
        }
    }
}

impl From<&GridCell> for Vec<Direction> {
    fn from(cell: &GridCell) -> Self {
        use Direction::*;
        use GridCell::*;

        match cell {
            Pipe('|') => vec![North, South],
            Pipe('-') => vec![East, West],
            Pipe('L') => vec![North, East],
            Pipe('J') => vec![North, West],
            Pipe('7') => vec![South, West],
            Pipe('F') => vec![South, East],
            Pipe('.') => vec![North, South, East, West],
            _ => vec![]
        }
    }
}


#[derive(Debug, PartialEq)]
enum GridCell {
    Strength(usize),
    Pipe(char),
}

type Tile = (isize, isize);
#[derive(Debug)]
struct Map {
    inner: Vec<Vec<GridCell>>,
    dim: (usize, usize),
}

impl Map {
    fn holes(&self, &(r, c): &Tile) -> Option<Vec<Direction>> {
        if self.tile_exceeds_dim(&(r, c)) { return None }

        if let Some(row) = self.inner.get(r as usize) {
            if let Some(cell) = row.get(c as usize) {
                return Some(Vec::from(cell))
            }
        }
        None
    }
    // Tile & going direction
    fn next(&self, &(r, c): &Tile, dir: &Direction) -> Option<Tile> {
        if  r <= 0 && *dir == Direction::North ||
            c <= 0 && *dir == Direction::West  ||
            r >= self.dim.1 as isize - 1 && *dir == Direction::South || // TODO: Check if correct
            c >= self.dim.0 as isize - 1 && *dir == Direction::East {
            return None
        }

        let (rd, cd) = dir.clone().into();
        let (r, d) = (r + rd, c + cd);

        let holes = self.holes(&(r, d));
        if holes.is_some() && holes.unwrap().contains(&!dir.clone()) { Some((r, d)) } else { None }
    }
    fn find_rodent(&self) -> Option<Tile> {
        for (i, j) in (0..self.inner.len()).map(|s| 0..self.inner[0].len()) {
            println!("{:?}, {:?}, {:?}", i, j, self.inner[i][j]);
            if self.inner[i][j] == GridCell::Pipe('S') {
                return Some((i as isize, j as isize));
            }
        }
        None
    }
    fn tile_exceeds_dim(&self, &(r, c): &Tile) -> bool {
        r < 0 || r > self.dim.0 as isize - 1 || c < 0 || c > self.dim.1 as isize - 1
    }
}

fn main() {
    let mut map = input();
    let mut stack: Vec<(Tile, Option<Direction>)> = Vec::new(); // Want to support as many things
                                                                // as possible, so allow for the
                                                                // initial position
    println!("{:?}", map);

    let Some((i, j)) = map.find_rodent() else { unreachable!() };
    map.inner[i as usize][j as usize] = GridCell::Strength(0);
    stack.push(((i, j), None));

    let mut currstren = 1;

    while let Some(((r, c), d)) = stack.pop() {
        // Nested because compound statements are not supported yet
        let cell = &map.inner[r as usize][c as usize];
        if let GridCell::Strength(s) = cell {
            if *s == 0 {
                println!("Found: ({:?}, {:?}", r, c);
                println!("At strength: {:?}", currstren);
            }
            continue
        } else if let GridCell::Pipe(_) = cell {
            let alldir: Vec<Direction> = cell.into();

            let stacklen = stack.len();
            alldir.into_iter()
                .filter(move |pipedir| pipedir != &d.clone().unwrap())
                .for_each(|dir| {
                    let Some(cell) = map.next(&(r, c), &dir) else { return };
                    stack.push((cell, Some(!dir)))
                });
            if stacklen == stack.len() {
                // Have to reset because only root nodes are considered
                println!("{:?}", stack);
                stack = vec![]
            }

            currstren += 1;
            map.inner[r as usize][c as usize] = GridCell::Strength(currstren)
        }
    }
}

fn input() -> Map {
    // let input = include_str!("../input");
    let input = ".....
.F-7.
.|.|.
.L-J.
.....";

    let inner: Vec<Vec<_>> = input.lines()
        .map(|s| s.chars().map(GridCell::Pipe).collect())
        .collect();
    let dim = (inner.len(), inner[0].len());

    Map { inner, dim }
}


