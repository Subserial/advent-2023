use std::collections::HashMap;

fn parse(data: &str) -> (&str, HashMap<String, (String, String)>) {
    let parser: regex::Regex =
        regex::Regex::new("^([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)").unwrap();
    let dirs = data.lines().next().unwrap();
    let nodes = data
        .lines()
        .skip(2)
        .filter_map(|line| {
            parser
                .captures(line)
                .map(|c| (c[1].to_string(), (c[2].to_string(), c[3].to_string())))
        })
        .collect();
    (dirs, nodes)
}

pub fn run_one(data: &str) -> String {
    let (dirs, nodes) = parse(data);
    let mut steps = 0;
    let mut node = "AAA";
    for dir in dirs.chars().cycle() {
        if node == "ZZZ" {
            break;
        }
        steps += 1;
        let (left, right) = nodes.get(node).unwrap();
        match dir {
            'L' => node = left,
            'R' => node = right,
            _ => panic!(),
        }
    }
    steps.to_string()
}

fn at(str: &str, i: usize) -> char {
    str.chars().skip(i).next().unwrap()
}

pub fn run_two(data: &str) -> String {
    let (dirs, nodes) = parse(data);
    let mut nodes_rv: Vec<String> = nodes.keys().map(|k| k.chars().rev().collect()).collect();
    nodes_rv.sort();
    let idx_to_node: Vec<String> = nodes_rv
        .into_iter()
        .rev()
        .map(|k| k.chars().rev().collect())
        .collect();
    let node_to_idx = idx_to_node
        .iter()
        .enumerate()
        .map(|(a, b)| (b.as_str(), a))
        .collect::<HashMap<_, _>>();
    let dir_n = dirs.len();
    let node_n = nodes.len();
    let starts = nodes
        .keys()
        .filter(|k| at(k.as_str(), 2) == 'A')
        .collect::<Vec<_>>();
    let maps = starts
        .iter()
        .map(|s| {
            let mut visits = vec![-1i64; dir_n * node_n];
            let mut path = Vec::new();
            let mut last = s.as_str();
            for (path_len, (idx, dir)) in dirs.chars().enumerate().cycle().enumerate() {
                let last_idx = node_to_idx[last];
                let visits_idx = idx + last_idx * dir_n;
                if visits[visits_idx] != -1 {
                    return (path, visits[visits_idx]);
                }
                path.push(visits_idx);
                visits[visits_idx] = path_len as i64;
                match dir {
                    'L' => last = &nodes[last].0,
                    'R' => last = &nodes[last].1,
                    _ => panic!(),
                }
            }
            unreachable!()
        })
        .collect::<Vec<_>>();
    let z_limit = starts.len() * dir_n;
    let meet = maps
        .iter()
        .map(|(map, _)| {
            map.iter()
                .enumerate()
                .filter(|(_, i)| **i < z_limit)
                .next()
                .unwrap()
                .0
                / dir_n
        })
        .fold(1, |accum, n| accum * n)
        * dir_n;
    meet.to_string()
}
