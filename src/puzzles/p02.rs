use std::cmp::max;

const RMAX: u32 = 12;
const GMAX: u32 = 13;
const BMAX: u32 = 14;

pub fn run_one(data: &str) -> String {
    data.lines()
        .map(|line| {
            let (g_no, games) = line.split_once(": ").unwrap();
            let g_no = g_no.split_once(" ").unwrap().1.parse::<u32>().unwrap();
            let possible = games
                .split("; ")
                .map(|pull| pull.split(", "))
                .flatten()
                .all(|group| {
                    let (num, color) = group.split_once(" ").unwrap();
                    let num = num.parse::<u32>().unwrap();
                    match color {
                        "red" => num <= RMAX,
                        "green" => num <= GMAX,
                        "blue" => num <= BMAX,
                        _ => panic!(),
                    }
                });
            return if possible { g_no } else { 0 };
        })
        .sum::<u32>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    data.lines()
        .map(|line| {
            let (mut r, mut g, mut b) = (0, 0, 0);
            line.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .map(|pull| pull.split(", "))
                .flatten()
                .for_each(|group| {
                    let (num, color) = group.split_once(" ").unwrap();
                    let num = num.parse::<u32>().unwrap();
                    match color {
                        "red" => r = max(r, num),
                        "green" => g = max(g, num),
                        "blue" => b = max(b, num),
                        _ => panic!(),
                    };
                });
            r * g * b
        })
        .sum::<u32>()
        .to_string()
}
