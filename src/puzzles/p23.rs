use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Path {
    Start,
    Empty,
    Wall,
    End,
    Left,
    Right,
    Up,
    Down,
}

fn populate_graph(
    grid: &HashMap<(i64, i64), Path>,
    paths: &mut HashMap<(i64, i64), HashMap<(i64, i64), usize>>,
    start: (i64, i64),
    first_dir: (i64, i64),
    end: (i64, i64),
) {
    let mut pos = (start.0 + first_dir.0, start.1 + first_dir.1);
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut possible = vec![pos];
    while possible.len() < 2 {
        pos = possible[0];
        seen.insert(pos);
        possible = match &grid[&pos] {
            Path::End => {
                if !paths.contains_key(&start) {
                    paths.insert(start, HashMap::new());
                }
                if !paths.contains_key(&end) {
                    paths.insert(end, HashMap::new());
                }
                paths.get_mut(&start).unwrap().insert(end, seen.len() - 1);
                paths.get_mut(&end).unwrap().insert(start, seen.len() - 1);
                return;
            }
            Path::Empty | Path::Start => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            Path::Left => vec![(0, -1)],
            Path::Down => vec![(1, 0)],
            Path::Right => vec![(0, 1)],
            Path::Up => vec![(-1, 0)],
            Path::Wall => vec![],
        }
        .into_iter()
        .filter_map(|p| {
            let next = (pos.0 + p.0, pos.1 + p.1);
            if seen.contains(&next) {
                return None;
            }
            let Some(path) = grid.get(&next) else {
                return None;
            };
            if let Path::Wall = path {
                return None;
            }
            Some(next)
        })
        .collect::<Vec<_>>();
        if possible.len() == 0 {
            return;
        }
    }
    if !paths.contains_key(&start) {
        paths.insert(start, HashMap::new());
    }
    if paths[&start].contains_key(&pos) {
        return;
    }
    paths.get_mut(&start).unwrap().insert(pos, seen.len() - 1);
    for dir in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        populate_graph(grid, paths, pos, dir, end);
    }
}

fn longest_salesman(
    paths: &HashMap<(i64, i64), HashMap<(i64, i64), usize>>,
    seen: HashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
) -> usize {
    let mut seen = seen;
    seen.insert(start);
    let mut longest = 0;
    for (&route, &len) in &paths[&start] {
        if route == end {
            longest = longest.max(len);
            continue;
        }
        if seen.contains(&route) {
            continue;
        }
        let cont_len = longest_salesman(paths, seen.clone(), route, end);
        longest = longest.max(len + cont_len);
    }
    longest
}

fn path_from(c: char) -> Path {
    match c {
        '.' => Path::Empty,
        '#' => Path::Wall,
        '<' => Path::Left,
        '>' => Path::Right,
        '^' => Path::Up,
        'v' => Path::Down,
        _ => panic!(),
    }
}

fn no_slopes(c: char) -> Path {
    match c {
        '#' => Path::Wall,
        _ => Path::Empty,
    }
}

fn parse(data: &str, path_parse: impl Fn(char) -> Path) -> HashMap<(i64, i64), Path> {
    let rows = data.lines().count();
    data.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    let path = match path_parse(c) {
                        Path::Empty => {
                            if row == 0 {
                                Path::Start
                            } else if row == rows - 1 {
                                Path::End
                            } else {
                                Path::Empty
                            }
                        }
                        p => p,
                    };
                    ((row as i64, col as i64), path)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn run_one(data: &str) -> String {
    let grid = parse(data, path_from);
    let start_col = data.find('.').unwrap() as i64;
    let end_row = data.lines().count() as i64;
    let end_col = data.lines().last().unwrap().find('.').unwrap() as i64;
    let mut paths = HashMap::new();
    populate_graph(
        &grid,
        &mut paths,
        (0, start_col),
        (1, 0),
        (end_row, end_col),
    );
    longest_salesman(&paths, HashSet::new(), (0, start_col), (end_row, end_col)).to_string()
}

pub fn run_two(data: &str) -> String {
    let grid = parse(data, no_slopes);
    let start_col = data.find('.').unwrap() as i64;
    let end_row = data.lines().count() as i64;
    let end_col = data.lines().last().unwrap().find('.').unwrap() as i64;
    let mut paths = HashMap::new();
    populate_graph(
        &grid,
        &mut paths,
        (0, start_col),
        (1, 0),
        (end_row, end_col),
    );
    longest_salesman(&paths, HashSet::new(), (0, start_col), (end_row, end_col)).to_string()
}
