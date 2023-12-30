use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> Vec<(&str, &str)> {
    data.lines().flat_map(|line| {
        let (src, dsts) = line.split_once(": ").unwrap();
        dsts.split(' ').map(|dst| (src, dst)).collect::<Vec<_>>()
    }).collect()
}

fn numify(data: Vec<(&str, &str)>) -> Vec<(usize, usize)> {
    let deduped = data.iter().flat_map(|v| [v.0, v.1]).collect::<HashSet<_>>();
    let hashed = deduped.iter().enumerate().map(|(i, &e)| (e, i)).collect::<HashMap<_, _>>();
    data.into_iter().map(|(l, r)| (hashed[l], hashed[r])).collect::<Vec<_>>()
}

fn karger(_edges: &Vec<(usize, usize)>) -> (usize, usize, usize) {
    panic!()
}

pub fn run_one(data: &str) -> String {
    let edges = numify(parse(data));
    let result = karger(&edges);
    println!("{:?}", result);
    let result = karger(&edges);
    println!("{:?}", result);
    let result = karger(&edges);
    println!("{:?}", result);
    let result = karger(&edges);
    println!("{:?}", result);
    let result = karger(&edges);
    println!("{:?}", result);
    let result = karger(&edges);
    println!("{:?}", result);

    panic!()
}

pub fn run_two(_data: &str) -> String {
    "0".to_string()
}