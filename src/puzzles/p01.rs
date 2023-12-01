use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub fn execute_first(data: &str) -> String {
    let result = data.lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit)
                .expect("no digit");
            let last = line.chars().rev().find(char::is_ascii_digit)
                .expect("no digit (reversed)");
            return 10 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap()
        })
        .sum::<u32>();
    result.to_string()
}

lazy_static! {
    static ref REGEX_WORD_NUM: Regex = Regex::new("(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    static ref MAP_WORD_NUM: HashMap<&'static str, u32> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].into_iter().collect();
}

pub fn execute_second(data: &str) -> String {
    let result = data.lines()
        .map(|line| {
            let data = REGEX_WORD_NUM.find_iter(line)
                .map(|c| c.as_str().to_string())
                .collect::<Vec<String>>();
            let first: &str = data[0].as_ref();
            let last: &str = data[data.len()-1].as_ref();
            let digit_first = MAP_WORD_NUM.get(first)
                .map(u32::clone)
                .unwrap_or_else(|| first.parse::<u32>().expect("first not digit"));
            let digit_last = MAP_WORD_NUM.get(last)
                .map(u32::clone)
                .unwrap_or_else(|| last.parse::<u32>().expect("last not digit"));
            println!("{} {}, {}", first, last, 10 * digit_first + digit_last);
            return 10 * digit_first + digit_last
        })
        .sum::<u32>();
    result.to_string()
}