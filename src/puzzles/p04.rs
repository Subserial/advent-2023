use std::collections::{HashMap, HashSet};

pub fn execute_first(data: &str) -> String {
    data.lines()
        .map(|line| {
            let (wins, board) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let wins = wins
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let total = board
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .filter(|num| wins.contains(&num))
                .count() as u32;
            if total == 0 {
                0
            } else {
                2u32.pow(total - 1)
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn execute_second(data: &str) -> String {
    let cards = data
        .lines()
        .map(|line| {
            let (wins, board) = line.split_once(":").unwrap().1.split_once("|").unwrap();
            let wins = wins
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            board
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .filter(|num| wins.contains(&num))
                .count() as u32
        })
        .collect::<Vec<u32>>();
    let mut known_wins: HashMap<usize, u32> = HashMap::new();
    for (i, &win) in cards.iter().enumerate().rev() {
        let mut sum = 1;
        for card_no in i + 1..=i + (win as usize) {
            sum += *known_wins.get(&card_no).unwrap();
        }
        known_wins.insert(i, sum);
    }
    known_wins.values().sum::<u32>().to_string()
}
