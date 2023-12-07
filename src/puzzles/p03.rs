use std::collections::{HashMap, HashSet};

fn collect(data: &str) -> (HashSet<(u32, u32)>, HashMap<(u32, u32), (u32, u32)>) {
    let symbols = data
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_ascii_digit() && !c.eq(&'.'))
                .map(move |(j, _)| (i as u32, j as u32))
        })
        .collect::<HashSet<(u32, u32)>>();

    let numbers = data
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .chain(vec!['.'])
                .enumerate()
                .fold(
                    (-1, 0, HashMap::new()),
                    |(mut start, mut accum, mut data), (j, char)| {
                        if let Some(digit) = char.to_digit(10) {
                            if start == -1 {
                                start = j as i32
                            }
                            accum = 10 * accum + digit;
                        } else {
                            if start != -1 {
                                data.insert((i as u32, start as u32), ((j - 1) as u32, accum));
                                start = -1;
                                accum = 0;
                            }
                        }
                        (start, accum, data)
                    },
                )
                .2
        })
        .collect::<HashMap<(u32, u32), (u32, u32)>>();

    (symbols, numbers)
}
pub fn run_one(data: &str) -> String {
    let (symbols, numbers) = collect(data);
    numbers
        .into_iter()
        .filter_map(|((row, start), (end, val))| {
            let rowm = std::cmp::max(1, row);
            let startm = std::cmp::max(1, start);
            for r in (rowm - 1)..=(row + 1) {
                for c in (startm - 1)..=(end + 1) {
                    if symbols.contains(&(r, c)) {
                        return Some(val);
                    }
                }
            }
            None
        })
        .sum::<u32>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    let charmap = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (symbols, numbers) = collect(data);
    symbols
        .into_iter()
        .filter_map(|(row, col)| {
            let rowl = std::cmp::max(1, row);
            let rowh = std::cmp::min(charmap.len() as u32 - 2, row);
            let coll = std::cmp::max(1, col);
            let colh = std::cmp::min(charmap[0].len() as u32 - 2, col);
            let mut found = Vec::new();
            for r in (rowl - 1)..=(rowh + 1) {
                let mut cont = false;
                for c in (coll - 1)..=(colh + 1) {
                    if charmap[r as usize][c as usize].is_ascii_digit() {
                        if !cont {
                            found.push((r, c));
                        }
                        cont = true;
                    } else {
                        cont = false;
                    }
                }
            }
            if found.len() != 2 {
                return None;
            }
            let (first, second) = (found.pop().unwrap(), found.pop().unwrap());
            let mut first_left = first.1 as i32;
            while first_left >= 0 && charmap[first.0 as usize][first_left as usize].is_ascii_digit()
            {
                first_left -= 1;
            }
            first_left += 1;
            let mut second_left = second.1 as i32;
            while second_left >= 0
                && charmap[second.0 as usize][second_left as usize].is_ascii_digit()
            {
                second_left -= 1;
            }
            second_left += 1;
            let first_val = numbers[&(first.0, first_left as u32)].1;
            let second_val = numbers[&(second.0, second_left as u32)].1;
            Some(first_val * second_val)
        })
        .sum::<u32>()
        .to_string()
}
