use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

fn next(dir: Dir, pipe: char) -> Option<Dir> {
    Some(match (dir, pipe) {
        (Dir::Up, '|') => Dir::Up,
        (Dir::Down, '|') => Dir::Down,
        (Dir::Left, '-') => Dir::Left,
        (Dir::Right, '-') => Dir::Right,
        (Dir::Up, '7') => Dir::Left,
        (Dir::Right, '7') => Dir::Down,
        (Dir::Down, 'J') => Dir::Left,
        (Dir::Right, 'J') => Dir::Up,
        (Dir::Down, 'L') => Dir::Right,
        (Dir::Left, 'L') => Dir::Up,
        (Dir::Up, 'F') => Dir::Right,
        (Dir::Left, 'F') => Dir::Down,
        (_, 'S') => Dir::Up,
        _ => return None,
    })
}

fn add_to((row, col): (i64, i64), dir: Dir) -> (i64, i64) {
    match dir {
        Dir::Up => (row - 1, col),
        Dir::Left => (row, col - 1),
        Dir::Right => (row, col + 1),
        Dir::Down => (row + 1, col),
    }
}

fn loop_coords(data: &str) -> (HashSet<(i64, i64)>, char) {
    let pipemap: HashMap<(i64, i64), char> = data
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| ((i as i64, j as i64), c))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let start = *pipemap.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let mut curr = start.clone();
    let mut next_dir = Dir::Right;
    for dir in vec![Dir::Left, Dir::Up].into_iter() {
        if let Some(_) = next(dir, *pipemap.get(&add_to(curr, dir)).unwrap_or(&'.')) {
            next_dir = dir;
            break;
        }
    }

    let first_dir = next_dir;
    let mut loop_pipes = HashSet::new();
    loop_pipes.insert(curr);
    loop {
        curr = add_to(curr, next_dir);
        let next_pipe = pipemap[&curr];
        if next_pipe == 'S' {
            break;
        }
        next_dir = next(next_dir, next_pipe).unwrap();
        loop_pipes.insert(curr);
    }
    let s_char = match (first_dir, next_dir) {
        (Dir::Left, Dir::Up) => '7',
        (Dir::Left, Dir::Left) => '-',
        (Dir::Left, Dir::Down) => 'J',
        (Dir::Up, Dir::Up) => '|',
        (Dir::Up, Dir::Left) => 'L',
        (Dir::Up, Dir::Right) => 'J',
        (Dir::Right, Dir::Up) => 'F',
        (Dir::Right, Dir::Right) => '-',
        (Dir::Right, Dir::Down) => 'L',
        _ => panic!(),
    };
    (loop_pipes, s_char)
}

pub fn run_one(data: &str) -> String {
    let (loop_pipes, _) = loop_coords(data);
    (loop_pipes.len() / 2).to_string()
}

pub fn run_two(data: &str) -> String {
    let (loop_pipes, s_char) = loop_coords(data);
    let mut sum = 0;
    for (i, line) in data.lines().enumerate() {
        let mut in_loop = false;
        let mut last_vert = '.';
        for (j, c) in line.chars().enumerate() {
            let coords = (i as i64, j as i64);
            if loop_pipes.contains(&coords) {
                let c = if c == 'S' { s_char } else { c };
                match c {
                    '|' => {
                        in_loop = !in_loop;
                    }
                    'F' | 'L' => {
                        last_vert = c;
                    }
                    'J' => {
                        if last_vert == 'F' {
                            in_loop = !in_loop;
                        }
                    }
                    '7' => {
                        if last_vert == 'L' {
                            in_loop = !in_loop;
                        }
                    }
                    _ => (),
                }
            } else if in_loop {
                sum += 1;
            }
        }
    }
    sum.to_string()
}
