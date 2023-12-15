use std::collections::HashMap;

fn load(count: usize, top_row: usize) -> usize {
    (top_row - count..top_row).sum()
}

pub fn run_one(data: &str) -> String {
    let datamap = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut sum = 0;
    for col in 0..datamap[0].len() {
        let mut count = 0;
        for row in (0..datamap.len()).rev() {
            match datamap[row][col] {
                '.' => (),
                'O' => count += 1,
                '#' => {
                    sum += load(count, datamap.len() - row);
                    count = 0;
                }
                _ => panic!(),
            }
        }
        sum += load(count, datamap.len() + 1);
    }
    sum.to_string()
}

fn roll_north(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_board: Vec<Vec<char>> = board
        .iter()
        .map(|row| {
            row.iter()
                .map(|col| if col.eq(&'#') { '#' } else { '.' })
                .collect()
        })
        .collect();
    for col in 0..board[0].len() {
        let mut count = 0;
        for row in (0..board.len()).rev() {
            match board[row][col] {
                '.' => (),
                'O' => count += 1,
                '#' => {
                    for nr in row + 1..row + count + 1 {
                        new_board[nr][col] = 'O';
                    }
                    count = 0;
                }
                _ => panic!(),
            }
        }
        for nr in 0..count {
            new_board[nr][col] = 'O';
        }
    }

    new_board
}

fn rotate_clockwise(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..board[0].len())
        .map(|col| (0..board.len()).rev().map(|row| board[row][col]).collect())
        .collect()
}

fn spin(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut board = board;
    for _ in 0..4 {
        board = roll_north(board);
        board = rotate_clockwise(board);
    }
    board
}

fn calc_board(board: &Vec<Vec<char>>) -> usize {
    (1..=board.len())
        .rev()
        .zip(board.iter())
        .map(|(val, row)| row.iter().filter(|c| **c == 'O').count() * val)
        .sum()
}

fn as_string(board: &Vec<Vec<char>>) -> String {
    board
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<String>()
}

pub fn run_two(data: &str) -> String {
    let mut seen = HashMap::new();
    let mut north_board = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut iter = 0;
    loop {
        let rep = as_string(&north_board);
        if seen.contains_key(&rep) {
            let loop_len = iter - seen[&rep];
            let remain = (1000000000 - iter) % loop_len;
            for _ in 0..remain {
                north_board = spin(north_board);
            }
            return calc_board(&north_board).to_string();
        }
        seen.insert(rep, iter);
        iter += 1;
        north_board = spin(north_board);
    }
}
