
fn count_ways(time: f64, dist: f64) -> u64 {
    let inner = time * time - (4f64 * dist);
    let quad_less = (time - inner.sqrt()) / 2f64;
    let quad_more = (time + inner.sqrt()) / 2f64;
    let less = quad_less.floor();
    let more = quad_more.ceil();
    (more - less) as u64 - 1
}

pub fn execute_first(data: &str) -> String {
    let mut lines = data.lines().map(|line| {
        line.split_whitespace().skip(1).map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let distances = lines.pop().unwrap();
    let times = lines.pop().unwrap();
    times.iter().zip(distances.iter()).map(|(&time, &dist)| count_ways(time as f64, dist as f64))
        .fold(1u64, |accum, n| accum * n)
        .to_string()
}

pub fn execute_second(data: &str) -> String {
    let mut lines = data.lines().map(|line| {
        line.split_whitespace().skip(1).collect::<String>().parse::<u64>().unwrap() as f64
    }).collect::<Vec<_>>();
    let dist = lines.pop().unwrap();
    let time = lines.pop().unwrap();
    count_ways(time, dist).to_string()
}