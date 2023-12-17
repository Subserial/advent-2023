use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

fn opposite(dir: Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Down,
        Dir::Left => Dir::Right,
        Dir::Down => Dir::Up,
        Dir::Right => Dir::Left,
    }
}

fn add(coords: (usize, usize), dir: Dir, max_coords: (usize, usize)) -> Option<(usize, usize)> {
    let next = match dir {
        Dir::Up => (coords.0.checked_sub(1)?, coords.1),
        Dir::Left => (coords.0, coords.1.checked_sub(1)?),
        Dir::Down => (coords.0 + 1, coords.1),
        Dir::Right => (coords.0, coords.1 + 1),
    };
    if next.0 >= max_coords.0 || next.1 >= max_coords.1 {
        return None;
    }
    return Some(next);
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Cursor {
    pos: (usize, usize),
    last_dir: Dir,
    last_count: usize,
}

fn run(data: &str, next_validate: impl Fn(Cursor, Dir) -> Option<Cursor>) -> usize {
    let data: Vec<Vec<usize>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let size = (data.len(), data[0].len());
    let final_pos = (size.0 - 1, size.1 - 1);
    let mut seen: HashMap<Cursor, usize> = HashMap::new();
    let mut pos: Vec<(Cursor, usize)> = Vec::new();
    let start_a = Cursor {
        pos: (0, 0),
        last_dir: Dir::Down,
        last_count: 0,
    };
    let start_b = Cursor {
        pos: (0, 0),
        last_dir: Dir::Right,
        last_count: 0,
    };
    pos.push((start_a, 0));
    seen.insert(start_a, 0);
    pos.push((start_b, 0));
    seen.insert(start_b, 0);
    let mut best_final = (size.0 + size.1) * 9;
    while pos.len() > 0 {
        let search = pos;
        pos = Vec::new();
        for (cursor, total) in search.into_iter() {
            for dir in [Dir::Up, Dir::Left, Dir::Down, Dir::Right] {
                let Some(next) = next_validate(cursor, dir) else {
                    continue;
                };
                let next_total = total + data[next.pos.0][next.pos.1];
                if next.pos == final_pos {
                    best_final = best_final.min(next_total);
                    continue;
                }
                if let Some(&prev_total) = seen.get(&next) {
                    if prev_total <= next_total {
                        continue;
                    }
                }
                seen.insert(next, next_total);
                pos.push((next, next_total));
            }
        }
    }
    best_final
}

pub fn run_one(data: &str) -> String {
    let max_coords = (data.lines().count(), data.lines().next().unwrap().len());
    run(data, |cursor, dir| {
        if dir == opposite(cursor.last_dir) {
            return None;
        }
        if cursor.last_count == 3 && cursor.last_dir == dir {
            return None;
        }
        let next_count = if dir == cursor.last_dir {
            cursor.last_count + 1
        } else {
            1
        };
        Some(Cursor {
            pos: add(cursor.pos, dir, max_coords)?,
            last_dir: dir,
            last_count: next_count,
        })
    })
    .to_string()
}

pub fn run_two(data: &str) -> String {
    let max_coords = (data.lines().count(), data.lines().next().unwrap().len());
    run(data, |cursor, dir| {
        if dir == opposite(cursor.last_dir) {
            return None;
        }
        if cursor.last_count <= 3 && cursor.last_dir != dir {
            return None;
        }
        if cursor.last_count >= 10 && cursor.last_dir == dir {
            return None;
        }
        let next_count = if dir == cursor.last_dir {
            cursor.last_count + 1
        } else {
            1
        };
        Some(Cursor {
            pos: add(cursor.pos, dir, max_coords)?,
            last_dir: dir,
            last_count: next_count,
        })
    })
    .to_string()
}
