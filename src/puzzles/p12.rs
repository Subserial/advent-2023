use std::collections::HashMap;

fn can_suffix(board: &[char], size: usize) -> bool {
    if board.len() < size {
        return false;
    }
    let hash_before = board.len() > size && board[board.len() - size - 1] == '#';
    return !hash_before && !board[board.len() - size..].contains(&'.');
}

fn count_arrange<'a>(
    data: &'a [char],
    broken: &'a [usize],
    memo: &mut HashMap<(&'a [char], &'a [usize]), usize>,
) -> usize {
    if broken.len() == 0 {
        return if data.contains(&'#') { 0 } else { 1 };
    }
    let min_size = broken.iter().sum::<usize>() + broken.len() - 1;
    let search = *broken.last().unwrap();
    let broken = &broken[..broken.len() - 1];
    let mut sum = 0;
    let mut slice = &data[..];
    while slice.len() >= min_size {
        if slice.ends_with(&['.']) {
            slice = &slice[..slice.len() - 1];
            continue;
        }
        if can_suffix(slice, search) {
            let next_slice = &slice[..slice.len() - search];
            if next_slice.len() == 0 {
                sum += if broken.len() > 0 { 0 } else { 1 };
            } else {
                if let Some(val) = memo.get(&(slice, broken)) {
                    sum += val
                } else {
                    let val = count_arrange(&slice[..slice.len() - search - 1], &broken, memo);
                    memo.insert((slice, broken), val);
                    sum += val;
                }
            }
        }
        if slice.ends_with(&['#']) {
            break;
        }
        slice = &slice[..slice.len() - 1];
    }
    sum
}

pub fn run_one(data: &str) -> String {
    data.lines()
        .map(|line| {
            let (board, hints) = line.split_once(" ").unwrap();
            let board = board.chars().collect::<Vec<_>>();
            let hints = hints
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut memo: HashMap<(&[char], &[usize]), usize> = HashMap::new();
            let val = count_arrange(&board, &hints, &mut memo);
            val
        })
        .sum::<usize>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    data.lines()
        .map(|line| {
            let (board, hints) = line.split_once(" ").unwrap();
            let hints = hints
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let board = vec![board].repeat(5).join("?").chars().collect::<Vec<_>>();
            let hints = hints.repeat(5);
            let mut memo: HashMap<(&[char], &[usize]), usize> = HashMap::new();
            let val = count_arrange(&board, &hints, &mut memo);
            val
        })
        .sum::<usize>()
        .to_string()
}
