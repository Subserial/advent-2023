fn collect_diffs(line: &str) -> Vec<Vec<i64>> {
    let data = line
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let len = data.len();
    let mut diffs: Vec<Vec<i64>> = Vec::new();
    diffs.push(data);
    for i in 0..len - 1 {
        diffs.push(
            (0..len - i - 1)
                .map(|j| diffs[i][j + 1] - diffs[i][j])
                .collect(),
        );
    }
    diffs
}

pub fn run_one(data: &str) -> String {
    data.lines()
        .map(|line| {
            let diffs = collect_diffs(line);
            diffs.iter().map(|vec| vec.last().unwrap()).sum::<i64>()
        })
        .sum::<i64>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    data.lines()
        .map(|line| {
            let diffs = collect_diffs(line);
            let mut accum = 0;
            let mut sign = 1;
            for row in diffs.iter() {
                accum += sign * *row.first().unwrap();
                sign *= -1;
            }
            accum
        })
        .sum::<i64>()
        .to_string()
}
