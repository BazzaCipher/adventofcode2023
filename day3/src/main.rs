type CharPosition = (usize, usize, char);

fn main() {
    let input = include_str!("../input");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();

    for (i, row) in input.split_whitespace().enumerate() {
        map.push(Vec::new());
        for (j, char) in row.chars().enumerate() {
            map[i].push(char);
            if !char.is_ascii_digit() && char != '.' {
                symbols.push((i, j));
            }
        }
    }

    let out: u64 = symbols.into_iter()
        .filter(|symbol| count_surrounding_unique(&map, symbol) == 2)
        .map(|symbol| {
            unique_leads(&map, &symbol)
                .into_iter()
                .inspect(|x| println!("{:?}", x))
                .map(|lead|
                    find_contiguous_string(&map, &lead)
                        .iter()
                        .map(|(_, _, c)| c)
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap()
                )
                .inspect(|x| println!("Words: {:?}", x))
                .reduce(|acc, e| e * acc)
                .unwrap()
        })
        .inspect(|x| println!("{:?}", x))
        .sum();

    // for symbol in symbols {
    //     leads.append(&mut surrounding_leads(&map, &symbol));
    // }
    //
    // let mut numbers: Vec<u64>      = Vec::new();
    // let mut nummap: BTreeSet<(usize, usize)> = BTreeSet::new();
    //
    // for (i, j) in leads {
    //     if nummap.contains(&(i, j)) {
    //         continue;
    //     }
    //     let found_number = find_contiguous_string(&map, &(i, j))
    //         .iter()
    //         .map(|(i, j, char)| {
    //             nummap.insert((*i, *j));
    //             char
    //         })
    //         .inspect(|x| println!("{x}"))
    //         .collect::<String>()
    //         .parse::<u64>()
    //         .unwrap();
    //     numbers.push(found_number);
    // }

    println!("{}", out);
}

fn find_contiguous_string(map: &[Vec<char>], start: &(usize, usize)) -> Vec<CharPosition> {
    let mut exit = false;
    let row = &map[start.0]; // Row major
    let max_pos = row.len() - 1;

    let mut lpos = start.1.saturating_sub(1);
    let mut rpos = (start.1 + 1).min(max_pos);

    while !exit {
        if (!row[lpos].is_ascii_digit() || lpos == 0) &&
            (!row[rpos].is_ascii_digit() || rpos == max_pos)
        {
            exit = true;
        }
        if row[lpos].is_ascii_digit() {
            lpos = usize::saturating_sub(lpos, 1);
        }
        if row[rpos].is_ascii_digit() {
            rpos = usize::min(rpos + 1, max_pos);
        }
    }

    let left = lpos + if row[lpos].is_ascii_digit() { 0 } else { 1 };
    let right = rpos - if row[rpos].is_ascii_digit() { 0 } else { 1 };
    (left..=right) // Might be without =
        .map(|j| (start.0, j, row[j]))
        .collect()
}

fn unique_leads(map: &[Vec<char>], &(ii, ij): &(usize, usize)) -> Vec<(usize, usize)> {
    let mut out = Vec::new();

    if map.get(ii.saturating_sub(1)).is_some() {
        out.append(&mut unique_row_lead_finder(map, &(ii-1, ij)))
    }

    if map.get((map.len()-1).min(ij + 1)).is_some() {
        out.append(&mut unique_row_lead_finder(map, &(ii+1, ij)))
    }

    let row = &map[ii];
    if let Some(c) = row.get(ij.saturating_sub(1)) {
        if c.is_ascii_digit() {
            out.push((ii, ij - 1))
        }
    }
    if let Some(c) = row.get((map[ii].len()-1).min(ij+1)) {
        if c.is_ascii_digit() {
            out.push((ii, ij + 1))
        }
    }

    out
}

fn unique_row_lead_finder(map: &[Vec<char>], &(ii, ij): &(usize, usize)) -> Vec<(usize, usize)> {
    let mut out = Vec::new();

    let (a, b, c) = (
        get_pos(map, &(ii as i32, ij as i32 - 1)).unwrap_or('.'),
        get_pos(map, &(ii as i32, ij as i32)).unwrap_or('.'),
        get_pos(map, &(ii as i32, ij as i32 + 1)).unwrap_or('.'),
    );

    if a.is_ascii_digit() && !b.is_ascii_digit() && c.is_ascii_digit() {
        out.push((ii, ij - 1));
        out.push((ii, ij + 1));
    } else if !a.is_ascii_digit() && !b.is_ascii_digit() && !c.is_ascii_digit() {
    } else if a.is_ascii_digit() {
        out.push((ii, ij - 1));
    } else if c.is_ascii_digit() {
        out.push((ii, ij + 1));
    } else {
        out.push((ii, ij));
    }

    out
}

fn count_surrounding_unique(map: &[Vec<char>], symbol: &(usize, usize)) -> usize {
    let mut tot = 0;
    let (a, b, c) = (
        get_pos(map, &(symbol.0 as i32 - 1, symbol.1 as i32 - 1)).unwrap_or('.'),
        get_pos(map, &(symbol.0 as i32 - 1, symbol.1 as i32)).unwrap_or('.'),
        get_pos(map, &(symbol.0 as i32 - 1, symbol.1 as i32 + 1)).unwrap_or('.'),
    );
    if a.is_ascii_digit() && !b.is_ascii_digit() && c.is_ascii_digit() {
        tot += 2;
    } else if !a.is_ascii_digit() && !b.is_ascii_digit() && !c.is_ascii_digit() {
        tot += 0;
    } else {
        tot += 1;
    }

    let (a, b, c) = (
        get_pos(map, &(symbol.0 as i32 + 1, symbol.1 as i32 - 1)).unwrap_or('.'),
        get_pos(map, &(symbol.0 as i32 + 1, symbol.1 as i32)).unwrap_or('.'),
        get_pos(map, &(symbol.0 as i32 + 1, symbol.1 as i32 + 1)).unwrap_or('.'),
    );
    if a.is_ascii_digit() && !b.is_ascii_digit() && c.is_ascii_digit() {
        tot += 2;
    } else if !a.is_ascii_digit() && !b.is_ascii_digit() && !c.is_ascii_digit() {
        tot += 0;
    } else {
        tot += 1;
    }

    if get_pos(map, &(symbol.0 as i32, symbol.1 as i32 - 1)).is_some_and(|x| char::is_ascii_digit(&x)) {
        tot += 1;
    }
    if get_pos(map, &(symbol.0 as i32, symbol.1 as i32 + 1)).is_some_and(|x| char::is_ascii_digit(&x)) {
        tot += 1;
    }

    tot
}

fn get_pos(map: &[Vec<char>], pos: &(i32, i32)) -> Option<char> {
    if pos.0 < 0 || pos.0 > map.len() as i32-1 || pos.1 < 0 || pos.1 > map[pos.0 as usize].len() as i32-1 {
        return None
    }
    Some(map[pos.0 as usize][pos.1 as usize])
}

