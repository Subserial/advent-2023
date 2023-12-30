use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> (HashMap<(i64, i64, i64), usize>, HashSet<(i64, i64, i64)>) {
    data.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            let (left, right) = line.split_once('~').unwrap();
            let mut left_iter = left.split(',').map(|p| p.parse::<i64>().unwrap());
            let left = (
                left_iter.next().unwrap(),
                left_iter.next().unwrap(),
                left_iter.next().unwrap(),
            );
            let mut right_iter = right.split(',').map(|p| p.parse::<i64>().unwrap());
            let right = (
                right_iter.next().unwrap(),
                right_iter.next().unwrap(),
                right_iter.next().unwrap(),
            );
            let mut spaces = Vec::new();

            for x in left.0..=right.0 {
                for y in left.1..=right.1 {
                    for z in left.2..=right.2 {
                        spaces.push((((x, y, z), i), left));
                    }
                }
            }
            spaces
        })
        .unzip()
}

fn find_neighbors(
    grid: &HashMap<(i64, i64, i64), usize>,
    pos: (i64, i64, i64),
) -> Vec<(i64, i64, i64)> {
    let block = grid[&pos];
    let mut found = HashSet::from([pos]);
    let mut search = vec![pos];
    while search.len() > 0 {
        let mut next = vec![];
        for pos in search {
            for (x, y, z) in [
                (1, 0, 0),
                (0, 1, 0),
                (0, 0, 1),
                (-1, 0, 0),
                (0, -1, 0),
                (0, 0, -1),
            ] {
                let new_pos = (pos.0 + x, pos.1 + y, pos.2 + z);
                if let Some(&b) = grid.get(&new_pos) {
                    if b == block && !found.contains(&new_pos) {
                        found.insert(new_pos);
                        next.push(new_pos);
                    }
                }
            }
        }
        search = next;
    }
    found.into_iter().collect()
}

fn descend_check(grid: &HashMap<(i64, i64, i64), usize>, all_pos: &[(i64, i64, i64)]) -> i64 {
    let min_z = all_pos.iter().min_by(|a, b| a.2.cmp(&b.2)).unwrap();
    let bottom = min_z.2;
    let lowest = all_pos.iter().filter(|v| v.2 == bottom).collect::<Vec<_>>();
    for next in (1..bottom).rev() {
        for low in &lowest {
            let check = (low.0, low.1, next);
            if grid.contains_key(&check) {
                return next + 1;
            }
        }
    }
    1
}

fn descend(grid: &mut HashMap<(i64, i64, i64), usize>, pos: (i64, i64, i64)) -> (i64, i64, i64) {
    let block = grid[&pos];
    let neighbors = find_neighbors(grid, pos);
    let new_height = descend_check(grid, &neighbors);
    if pos.2 == new_height {
        return pos;
    }
    for ngh in &neighbors {
        grid.remove(ngh);
    }
    let diff = new_height - pos.2;
    for ngh in &neighbors {
        grid.insert((ngh.0, ngh.1, ngh.2 + diff), block);
    }
    (pos.0, pos.1, new_height)
}

pub fn run_one(data: &str) -> String {
    let (mut grid, blocks) = parse(data);
    let mut blocks_z = blocks.into_iter().collect::<Vec<_>>();
    blocks_z.sort_by(|a, b| a.2.cmp(&b.2));
    for block in &mut blocks_z {
        *block = descend(&mut grid, *block);
    }
    blocks_z.sort_by(|a, b| a.2.cmp(&b.2));
    let mut remove_total = 0;
    for idx in 0..blocks_z.len() {
        let mut can_remove = true;
        let block = blocks_z[idx];
        let id = grid[&block];
        let nghs = find_neighbors(&grid, block);
        for ngh in &nghs {
            grid.remove(ngh);
        }
        for ngh in &nghs {
            let above_pos = (ngh.0, ngh.1, ngh.2 + 1);
            if let Some(_) = grid.get(&above_pos) {
                let above_nghs = find_neighbors(&grid, above_pos);
                if descend_check(&grid, &above_nghs) < above_pos.2 {
                    can_remove = false;
                    break;
                }
            }
        }
        for ngh in nghs {
            grid.insert(ngh, id);
        }
        if can_remove {
            remove_total += 1;
        }
    }
    remove_total.to_string()
}

pub fn run_two(data: &str) -> String {
    let (mut grid, blocks) = parse(data);
    let mut blocks_z = blocks.into_iter().collect::<Vec<_>>();
    blocks_z.sort_by(|a, b| a.2.cmp(&b.2));
    for block in &mut blocks_z {
        *block = descend(&mut grid, *block);
    }
    blocks_z.sort_by(|a, b| a.2.cmp(&b.2));
    let mut fall_sum = 0;
    for idx in 0..blocks_z.len() {
        let mut fall_total = 0;
        let mut grid_copy = grid.clone();
        let mut blocks_copy = blocks_z.clone();
        let block = blocks_copy[idx];
        blocks_copy.remove(idx);
        let nghs = find_neighbors(&grid_copy, block);
        for ngh in &nghs {
            grid_copy.remove(ngh);
        }
        for falling in blocks_copy {
            let new_block = descend(&mut grid_copy, falling);
            if new_block != falling {
                fall_total += 1;
            }
        }
        fall_sum += fall_total;
    }
    fall_sum.to_string()
}
