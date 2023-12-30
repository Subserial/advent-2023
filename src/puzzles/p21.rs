use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> ((i64, i64), HashMap<(i64, i64), char>) {
    let mut start = (0, 0);
    let field: HashMap<(i64, i64), char> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| {
                    let real_char = match char {
                        'S' => {
                            start = (row as i64, col as i64);
                            '.'
                        }
                        c => c,
                    };
                    ((row as i64, col as i64), real_char)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    (start, field)
}

fn run(field: &HashMap<(i64, i64), char>, start: (i64, i64), iters: usize) -> usize {
    let mut locations = HashSet::new();
    locations.insert(start);
    for _ in 0..iters {
        let mut next_set = HashSet::new();
        for location in locations {
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next = (location.0 + dir.0, location.1 + dir.1);
                if field.get(&next) == Some(&'.') {
                    next_set.insert(next);
                }
            }
        }
        locations = next_set;
    }
    locations.len()
}

pub fn run_one(data: &str) -> String {
    let (start, field) = parse(data);
    run(&field, start, 64).to_string()
}

const STEPS: usize = 26501365;

// we will assume "start" is in the exact center of an odd-length grid,
// the grid is square, STEPS is larger than the length of the grid,
// AND that there are lines of sight from S to all edges.
pub fn run_two(data: &str) -> String {
    let (start, field) = parse(data);
    let size = data.lines().count();
    let end_inside = (STEPS % 2) + size * 2;
    let end_inside_alt = ((STEPS + 1) % 2) + size * 2;
    let end_corner = (STEPS - size / 2 - 1) % size;
    let end_corner_before = end_corner + size;
    let end_edge_outer = (STEPS - 1) % size;
    let end_edge_inner = end_edge_outer + size;

    let radius = (STEPS - 1) / size;
    let edges_outer = radius;
    let edges_inner = radius - 1;
    let (end_count, end_alt_count) = {
        let large_square = radius.pow(2) - if end_corner > size / 2 { 4 } else { 0 };
        let small_square = (radius - 1).pow(2);
        if radius % 2 == 0 {
            (small_square, large_square)
        } else {
            (large_square, small_square)
        }
    };
    let far_end = size as i64 - 1;
    let mut total = run(&field, start, end_inside) * end_count
        + run(&field, start, end_inside_alt) * end_alt_count;
    for start in [
        (start.0, 0),
        (start.0, far_end),
        (0, start.1),
        (far_end, start.1),
    ] {
        total += run(&field, start, end_corner);
        total += run(&field, start, end_corner_before);
    }
    for start in [(0, 0), (0, far_end), (far_end, 0), (far_end, far_end)] {
        total += run(&field, start, end_edge_outer) * edges_outer;
        total += run(&field, start, end_edge_inner) * edges_inner;
    }
    total.to_string()
}
