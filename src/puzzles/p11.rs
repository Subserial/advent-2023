use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn run(data: &str, offset: usize) -> String {
    let galaxies_init: HashSet<(usize, usize)> = data
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let (rows, cols) = (data.lines().count(), data.lines().next().unwrap().len());
    let mut galaxies_rfix = HashSet::new();
    let mut row_offset = 0;
    for row in 0..rows {
        let row_g = galaxies_init
            .iter()
            .filter(|g| (**g).0 == row)
            .collect::<Vec<_>>();
        if row_g.len() == 0 {
            row_offset += offset;
        }
        for &g in row_g {
            galaxies_rfix.insert((g.0 + row_offset, g.1));
        }
    }
    let mut galaxies_cfix = Vec::new();
    let mut col_offset = 0;
    for col in 0..cols {
        let col_g = galaxies_rfix
            .iter()
            .filter(|g| (**g).1 == col)
            .collect::<Vec<_>>();
        if col_g.len() == 0 {
            col_offset += offset;
        }
        for &g in col_g {
            galaxies_cfix.push((g.0, g.1 + col_offset));
        }
    }
    let galaxies = galaxies_cfix;
    let mut dist = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (g1, g2) = (galaxies[i], galaxies[j]);
            dist += max(g1.0, g2.0) - min(g1.0, g2.0) + max(g1.1, g2.1) - min(g1.1, g2.1);
        }
    }
    dist.to_string()
}

pub fn run_one(data: &str) -> String {
    run(data, 2 - 1)
}

pub fn run_two(data: &str) -> String {
    run(data, 1000000 - 1)
}
