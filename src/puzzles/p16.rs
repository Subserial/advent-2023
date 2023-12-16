use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn next(
    dir: Dir,
    (row, col): (usize, usize),
    (max_row, max_col): (usize, usize),
) -> Option<(usize, usize)> {
    let (row, col) = match dir {
        Dir::Up => (row.checked_sub(1)?, col),
        Dir::Down => (row + 1, col),
        Dir::Left => (row, col.checked_sub(1)?),
        Dir::Right => (row, col + 1),
    };
    if row >= max_row || col >= max_col {
        return None;
    }
    return Some((row, col));
}

fn interact(tile: char, dir: Dir) -> Vec<Dir> {
    match (tile, dir) {
        ('.', _) => vec![dir],
        ('|', Dir::Left | Dir::Right) => vec![Dir::Up, Dir::Down],
        ('|', Dir::Up | Dir::Down) => vec![dir],
        ('-', Dir::Up | Dir::Down) => vec![Dir::Left, Dir::Right],
        ('-', Dir::Left | Dir::Right) => vec![dir],
        ('/', Dir::Left) => vec![Dir::Down],
        ('/', Dir::Up) => vec![Dir::Right],
        ('/', Dir::Right) => vec![Dir::Up],
        ('/', Dir::Down) => vec![Dir::Left],
        ('\\', Dir::Left) => vec![Dir::Up],
        ('\\', Dir::Up) => vec![Dir::Left],
        ('\\', Dir::Right) => vec![Dir::Down],
        ('\\', Dir::Down) => vec![Dir::Right],
        _ => panic!(),
    }
}

fn run(grid: &Vec<Vec<char>>, start_pos: (usize, usize), start_dir: Dir) -> usize {
    let grid_max = (grid.len(), grid[0].len());
    let mut seen: HashMap<(usize, usize), HashSet<Dir>> = HashMap::new();
    let mut follows: Vec<((usize, usize), Dir)> = Vec::new();
    follows.push((start_pos, start_dir));
    while follows.len() > 0 {
        let moves = follows;
        follows = Vec::new();
        for (coords, dir) in moves.into_iter() {
            if let Some(dirs) = seen.get_mut(&coords) {
                if dirs.contains(&dir) {
                    continue;
                }
                dirs.insert(dir);
            } else {
                seen.insert(coords, HashSet::from([dir]));
            }
            let next_dirs = interact(grid[coords.0][coords.1], dir);
            for dir in next_dirs.into_iter() {
                if let Some(next_coords) = next(dir, coords, grid_max) {
                    follows.push((next_coords, dir));
                }
            }
        }
    }
    seen.len()
}

pub fn run_one(data: &str) -> String {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    run(&grid, (0, 0), Dir::Right).to_string()
}

pub fn run_two(data: &str) -> String {
    let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (grid.len(), grid[0].len());
    let top = (0..cols)
        .map(|col| run(&grid, (0, col), Dir::Down))
        .max()
        .unwrap();
    let left = (0..rows)
        .map(|row| run(&grid, (row, 0), Dir::Right))
        .max()
        .unwrap();
    let right = (0..rows)
        .map(|row| run(&grid, (row, cols - 1), Dir::Left))
        .max()
        .unwrap();
    let bottom = (0..cols)
        .map(|col| run(&grid, (rows - 1, col), Dir::Up))
        .max()
        .unwrap();
    [top, left, right, bottom].iter().max().unwrap().to_string()
}
