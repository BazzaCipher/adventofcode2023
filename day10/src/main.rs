#![feature(iter_array_chunks)]

use std::ops::Not;

// I think I should've built a tree...
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

#[derive(Debug, PartialEq)]
enum GridCell {
    Strength(usize),
    Pipe(char),
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
            Pipe('S') => vec![North, South, East, West],
            _ => vec![]
        }
    }
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
            r >= self.dim.1 as isize && *dir == Direction::South ||
            c >= self.dim.0 as isize && *dir == Direction::East {
            return None
        }

        let (rd, cd) = dir.clone().into();
        let (r, d) = (r + rd, c + cd);

        let holes = self.holes(&(r, d));
        if holes.is_some() && holes.unwrap().contains(&!dir.clone()) { Some((r, d)) } else { None }
    }
    fn find_rodent(&self) -> Option<Tile> {
        for i in 0..self.inner.len() {
            for j in 0..self.inner[0].len() {
                if self.inner[i][j] == GridCell::Pipe('S') {
                    return Some((i as isize, j as isize));
                }
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
    let mut stack: Vec<(Tile, Option<Direction>)> = Vec::new();
    let mut loopmap = vec![vec![false; map.inner[0].len()]; map.inner.len()];

    let Some((i, j)) = map.find_rodent() else { unreachable!() };
    
    let mut currstren = 0;
    stack.push(((i, j), None));

    while let Some(((r, c), d)) = stack.pop() {
        // Nested because compound statements are not supported yet
        let cell = &map.inner[r as usize][c as usize];
        if let GridCell::Strength(s) = cell {
            println!("So the total strength was {:?}", currstren/2);
            currstren = 0;
            continue
        } else if let GridCell::Pipe(_) = cell {
            let alldir: Vec<Direction> = cell.into();

            let mut addlen = alldir.into_iter()
                .filter_map(|pipedir| {
                    if let Some(dir) = d.clone() { if pipedir == !dir { return None } }
                    map.next(&(r, c), &pipedir).map(|c| (c, Some(pipedir)))
                })
                .collect::<Vec<_>>();

            // Need this in case the first time search doesn't work
            // if addlen.is_empty() {
            //     loopmap = vec![vec![false; map.inner[0].len()]; map.inner.len()];
            // }

            stack.append(&mut addlen);
            println!("{:?}, {:?}", r, c);

            map.inner[r as usize][c as usize] = GridCell::Strength(currstren);
            loopmap[r as usize][c as usize] = true;
            currstren += 1
        }
    }

    println!("Loop map: {:?}", loopmap);
    let out: usize = loopmap.into_iter().map(|row| {
        let m = row.iter()
            .enumerate()
            // True if searching for next match
            .filter(|(i, &s)| s)
            .array_chunks()
            .map(|[(i, _), (j, _)]| j - i - 1)
            .sum::<usize>();
        println!("{m}");
        m
    }).sum();
    println!("{out}");
}

fn input() -> Map {
    // let input = include_str!("../input");
    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    let inner: Vec<Vec<_>> = input.lines()
        .map(|s| s.chars().map(GridCell::Pipe).collect())
        .collect();
    let dim = (inner.len(), inner[0].len());

    Map { inner, dim }
}


