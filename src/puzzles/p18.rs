use std::collections::HashMap;

fn run(data: Vec<(&str, i64)>) -> i64 {
    let pos_first = data[0].0;
    let pos_second = data[1].0;
    let mut area_sum = 0;
    let mut edge_sum = 0;
    let mut dim = 0;
    data.chunks(2).for_each(|pair| {
        let (first, second) = (pair[0], pair[1]);
        edge_sum += first.1 + second.1;
        if first.0 == pos_first {
            dim += first.1;
        } else {
            dim -= first.1;
        }
        let width = if second.0 == pos_second {
            second.1
        } else {
            -second.1
        };
        area_sum += dim * width;
    });
    area_sum.abs() + edge_sum / 2 + 1
}

pub fn run_one(data: &str) -> String {
    let lines: Vec<(&str, i64)> = data
        .lines()
        .map(|line| {
            let (letter, rest) = line.split_once(" ").unwrap();
            let dist = rest.split_once(" ").unwrap().0;
            (letter, dist.parse().unwrap())
        })
        .collect();
    run(lines).to_string()
}

pub fn run_two(data: &str) -> String {
    let char_map = HashMap::from([("0", "R"), ("1", "D"), ("2", "L"), ("3", "U")]);
    let lines: Vec<(&str, i64)> = data
        .lines()
        .map(|line| {
            let (_, code) = line.split_once("(#").unwrap();
            (
                char_map[&code[5..6]],
                i64::from_str_radix(&code[0..5], 16).unwrap(),
            )
        })
        .collect();
    run(lines).to_string()
}
