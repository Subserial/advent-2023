pub fn execute_first(data: &str) -> String {
    let lines = data.lines().collect::<Vec<_>>();
    let seeds = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps: Vec<Box<dyn Fn(u64) -> u64>> = Vec::new();
    let mut indices: Vec<(u64, u64, u64)> = Vec::new();
    for line in lines[3..].iter().chain(vec![&""]) {
        if line.is_empty() {
            let tables = indices;
            indices = Vec::new();
            maps.push(Box::new(move |n| {
                for &(dest, start, len) in tables.iter() {
                    if start <= n && n < start + len {
                        return n - start + dest;
                    }
                }
                n
            }));
            continue;
        } else if line.contains(":") {
            continue;
        }
        let vec = line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        indices.push((vec[0].clone(), vec[1].clone(), vec[2].clone()))
    }

    seeds
        .into_iter()
        .map(|n| {
            let mut val = n;
            for f in maps.iter() {
                val = f(val);
            }
            val
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn execute_second(data: &str) -> String {
    let lines = data.lines().collect::<Vec<_>>();
    let seeds = lines[0]
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|c| (c[0], c[1]))
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();
    let mut indices: Vec<(u64, u64, u64)> = Vec::new();
    for line in lines[3..].iter().chain(vec![&""]) {
        if line.is_empty() {
            maps.push(indices);
            indices = Vec::new();
            continue;
        } else if line.contains(":") {
            continue;
        }
        let vec = line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        indices.push((vec[0].clone(), vec[1].clone(), vec[2].clone()))
    }

    let mut seed_ranges = seeds;
    for tables in maps.iter() {
        let mut matched = Vec::new();
        let mut split_ranges = seed_ranges.clone();
        for &(t_dest, t_start, t_len) in tables.iter() {
            let mut unmatched = Vec::new();
            for &(o_start, o_len) in split_ranges.iter() {
                let starts_before = o_start < t_start;
                let starts_within = o_start >= t_start && o_start < t_start + t_len;
                let exceeds_start = o_start + o_len > t_start;
                let exceeds_end = o_start + o_len > t_start + t_len;
                if starts_before && exceeds_start {
                    let left_len = t_start - o_start;
                    unmatched.push((o_start, left_len));
                    if exceeds_end {
                        let right_start = t_start + t_len;
                        let right_len = o_len - (left_len + t_len);
                        matched.push((t_dest, t_len));
                        unmatched.push((right_start, right_len));
                    } else {
                        let mid_len = o_len - left_len;
                        matched.push((t_dest, mid_len));
                    }
                    continue;
                }
                if starts_within {
                    let mid_start = o_start - t_start + t_dest;
                    if exceeds_end {
                        let right_start = t_start + t_len;
                        let right_len = o_start + o_len - right_start;
                        let mid_len = right_start - o_start;
                        matched.push((mid_start, mid_len));
                        unmatched.push((right_start, right_len));
                    } else {
                        matched.push((mid_start, o_len));
                    }
                    continue;
                }
                unmatched.push((o_start, o_len));
            }
            split_ranges = unmatched;
        }
        matched.append(&mut split_ranges);
        seed_ranges = matched
    }

    let mut starts = seed_ranges.into_iter().map(|x| x.0).collect::<Vec<_>>();
    starts.sort();
    starts.first().unwrap().to_string()
}
