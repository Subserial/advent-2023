use std::collections::HashMap;
use std::ops::Deref;
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
    // Actual line: 3two9six9sixfiveoneightf
    static ref REGEX_WORD_NUM_REVERSE: Regex = Regex::new("(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();
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
            let first = REGEX_WORD_NUM.find(line)
                .expect("no match")
                .as_str();
            let rev = line.chars().rev().collect::<String>();
            let last = REGEX_WORD_NUM_REVERSE.find(&rev)
                .expect("no match")
                .as_str()
                .chars()
                .rev()
                .collect::<String>();
            println!("{} {}", first, last);
            let digit_first = MAP_WORD_NUM.get(first)
                .map(u32::clone)
                .unwrap_or_else(|| first.parse::<u32>().expect("first not digit"));
            let digit_last = MAP_WORD_NUM.get(last.deref())
                .map(u32::clone)
                .unwrap_or_else(|| last.parse::<u32>().expect("last not digit"));
            return 10 * digit_first + digit_last
        })
        .sum::<u32>();
    result.to_string()
}