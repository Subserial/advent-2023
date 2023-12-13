use std::cmp::min;

pub fn run_one(data: &str) -> String {
    data.split("\n\n")
        .map(|board| {
            let data = board
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let (rows, cols) = (board.lines().count(), board.lines().next().unwrap().len());
            let row_lines = board.lines().map(String::from).collect::<Vec<_>>();
            let col_lines = (0..cols)
                .map(|col| (0..rows).map(|row| data[row][col]).collect::<String>())
                .collect::<Vec<_>>();
            for row in 1..rows {
                let row_count = min(row, rows - row);
                if (0..row_count)
                    .all(|offset| row_lines[row - offset - 1] == row_lines[row + offset])
                {
                    return 100 * row;
                }
            }
            for col in 1..cols {
                let col_count = min(col, cols - col);
                if (0..col_count)
                    .all(|offset| col_lines[col - offset - 1] == col_lines[col + offset])
                {
                    return col;
                }
            }
            return 0;
        })
        .sum::<usize>()
        .to_string()
}

fn diff(a: &str, b: &str) -> usize {
    a.len() - a.chars().zip(b.chars()).filter(|&(a, b)| a == b).count()
}

pub fn run_two(data: &str) -> String {
    data.split("\n\n")
        .map(|board| {
            let data = board
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let (rows, cols) = (board.lines().count(), board.lines().next().unwrap().len());
            let row_lines = board.lines().map(String::from).collect::<Vec<_>>();
            let col_lines = (0..cols)
                .map(|col| (0..rows).map(|row| data[row][col]).collect::<String>())
                .collect::<Vec<_>>();
            for row in 1..rows {
                let row_count = min(row, rows - row);
                if (0..row_count)
                    .map(|offset| diff(&row_lines[row - offset - 1], &row_lines[row + offset]))
                    .sum::<usize>()
                    == 1
                {
                    return 100 * row;
                }
            }
            for col in 1..cols {
                let col_count = min(col, cols - col);
                if (0..col_count)
                    .map(|offset| diff(&col_lines[col - offset - 1], &col_lines[col + offset]))
                    .sum::<usize>()
                    == 1
                {
                    return col;
                }
            }
            return 0;
        })
        .sum::<usize>()
        .to_string()
}
