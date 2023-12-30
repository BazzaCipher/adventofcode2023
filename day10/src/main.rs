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

impl Not for &Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        use Direction::*;
        match &self {
            North => South,
            South => North,
            East  => West,
            West  => East,
        }
    }
}

impl From<&Direction> for (isize, isize) {
    // Assume column major, from top left
    fn from(dir: &Direction) -> Self {
        match &dir {
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
    fn holes(&self, &(r, c): &Tile) -> Vec<Direction> {
        if self.tile_exceeds_dim(&(r, c)) { return vec![] }

        if let Some(row) = self.inner.get(r as usize) {
            if let Some(cell) = row.get(c as usize) {
                return Vec::from(cell)
            }
        }
        vec![]
    }
    fn is_invalid(&self, &(r, c): &Tile, dir: &Direction) -> bool {
            r <= 0 && *dir == Direction::North ||
            c <= 0 && *dir == Direction::West  ||
            r >= self.dim.0 as isize && *dir == Direction::South ||
            c >= self.dim.1 as isize && *dir == Direction::East
    }
    // Tile & going direction
    fn next(&self, &(r, c): &Tile, dir: &Direction) -> Option<Tile> {
        if self.is_invalid(&(r, c), dir) { return None }

        let (rd, cd) = dir.into();
        let (r, d) = (r + rd, c + cd);

        let holes = self.holes(&(r, d));
        if holes.contains(&!dir) { Some((r, d)) } else { None }
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
    let mut extmap = vec![vec![false; map.inner[0].len() * 2 - 1]; map.inner.len() * 2 - 1];

    let Some((i, j)) = map.find_rodent() else { unreachable!() };
    
    let mut currstren = 0;
    stack.push(((i, j), None));

    while let Some(((r, c), d)) = stack.pop() {
        // Nested because compound statements are not supported yet
        if currstren == 1 { map.inner[i as usize][j as usize] = GridCell::Pipe('S') }
        let cell = &map.inner[r as usize][c as usize];
        if let GridCell::Strength(_) = cell {
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
            // println!("{:?}, {:?}", r, c);

            map.inner[r as usize][c as usize] = GridCell::Strength(currstren);
            loopmap[r as usize][c as usize] = true;
            if let Some(dir) = d {
                let (dr, dc) = (&!dir).into();
                extmap[(r * 2 + dr) as usize][(c * 2 + dc) as usize] = true;
            }
            extmap[r as usize * 2][c as usize * 2] = true;

            currstren += 1
        }
    }

    printsquare(&loopmap);
    printsquare(&extmap);

    println!("Flood filled: {:?}", floodfill(&mut extmap, &(i, j)));

    // Build the extended loopmap

    // let out: usize = loopmap.into_iter().map(|row| {
    //     let m = row.iter()
    //         .enumerate()
    //         // True if searching for next match
    //         .filter(|(i, &s)| s)
    //         .array_chunks()
    //         .map(|[(i, _), (j, _)]| j - i - 1)
    //         .sum::<usize>();
    //     println!("{m}");
    //     m
    // }).sum();
    // println!("{out}");
}

fn input() -> Map {
    let input = include_str!("../input");

    let inner: Vec<Vec<_>> = input.lines()
        .map(|s| s.chars().map(GridCell::Pipe).collect())
        .collect();
    let dim = (inner.len(), inner[0].len());

    Map { inner, dim }
}

fn floodfill(input: &mut Vec<Vec<bool>>, root: &(isize, isize)) -> Vec<u32> {
    // A (1) list of (2) odd flood plain (3) rows
    let mut out = Vec::new();
    let mut map = input.clone();
    let dim = (input.len(), input[0].len());
    let leads = [(root.0 - 1, root.1 - 1), (root.0 + 1, root.1 - 1), (root.0 - 1, root.1 + 1), (root.0 + 1, root.1 + 1)];
    let mut m = leads.iter();

    // while let Some((i, j)) = (0..dim.0*dim.1).map(|i| (i / dim.1, i % dim.0)).find(|(i, j)| !map[*i][*j]) {
    while let Some(&(i, j)) = m.next() {
        let mut out1 = vec![0; input.len() / 2 + 1];
        let mut q: Vec<Tile> = vec![(i as isize, j as isize)];

        while let Some(tile) = q.pop() {
            if map[tile.0 as usize][tile.1 as usize] { continue }
            if tile.0 < input.len() as isize && map.get(tile.0 as usize + 1).map(|m| m.get(tile.1 as usize)) == Some(Some(&false)) {
                q.push((tile.0 + 1, tile.1));
            }
            if tile.0 > 0 && map.get(tile.0 as usize - 1).map(|m| m.get(tile.1 as usize)) == Some(Some(&false)) {
                q.push((tile.0 - 1, tile.1));
            }
            if tile.1 < input[0].len() as isize && map.get(tile.0 as usize).map(|m| m.get(tile.1 as usize + 1)) == Some(Some(&false)) {
                q.push((tile.0, tile.1 + 1));
            }
            if tile.1 > 0 && map.get(tile.0 as usize).map(|m| m.get(tile.1 as usize - 1)) == Some(Some(&false)) {
                q.push((tile.0, tile.1 - 1));
            }
            map[tile.0 as usize][tile.1 as usize] = true;

            if tile.0 % 2 == 0 && tile.1 % 2 == 0 {
                out1[(tile.0/2) as usize] += 1;
            }
        }

        println!();
        printsquare(&map);
        println!("{:?}", out1);
        out.push(out1.iter().sum());
    }
    out
}

fn printsquare(map: &[Vec<bool>]) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            print!("{}", if map[i][j] { 1 } else {0 });
        }
        println!();
    }
}
