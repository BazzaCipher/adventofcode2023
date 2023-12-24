use std::collections::BTreeSet;

type CharPosition = (usize, usize, char);

fn main() {
    let input = include_str!("../input");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();
    let mut leads: Vec<(usize, usize)> = Vec::new();

    for (i, row) in input.split_whitespace().enumerate() {
        map.push(Vec::new());
        for (j, char) in row.chars().enumerate() {
            map[i].push(char);
            if !char.is_ascii_digit() && char != '.' {
                symbols.push((i, j));
            }
        }
    }
    
    for symbol in symbols {
        leads.append(&mut surrounding_leads(&map, &symbol));
    }

    let mut numbers: Vec<u64>      = Vec::new();
    let mut nummap: BTreeSet<(usize, usize)> = BTreeSet::new();

    for (i, j) in leads {
        if nummap.contains(&(i, j)) {
            continue;
        }
        let found_number = find_contiguous_string(&map, &(i, j))
            .iter()
            .map(|(i, j, char)| {
                nummap.insert((*i, *j));
                char
            })
            .inspect(|x| println!("{x}"))
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        numbers.push(found_number);
    }

    println!("{}", numbers.iter().sum::<u64>());
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

fn surrounding_leads(map: &[Vec<char>], (ii, ij): &(usize, usize)) -> Vec<(usize, usize)> {
    let mut out = Vec::new();

    for i in 0..=2 {
        for j in 0..=2 {
            if i == 1 && j == 1 {
                continue
            }
            let checkedleft = usize::saturating_sub(ii + i, 1);
            let checkedright = usize::saturating_sub(ij + j, 1);

            if let Some(row) = map.get(checkedleft) {
                if row.get(checkedright).unwrap().is_ascii_digit() {
                    out.push((checkedleft, checkedright));
                }
            }
        }
    }

    out
}
