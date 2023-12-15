use std::collections::HashMap;

fn hash(data: &str) -> usize {
    data.bytes()
        .fold(0, |accum, c| ((accum + c as usize) * 17) % 256)
}

pub fn run_one(data: &str) -> String {
    data.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|code| hash(code))
        .sum::<usize>()
        .to_string()
}

fn index(data: &Vec<(&str, usize)>, label: &str) -> Option<usize> {
    (0..data.len()).find(|val| data.get(*val).unwrap().0 == label)
}

pub fn run_two(data: &str) -> String {
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    data.lines().next().unwrap().split(',').for_each(|code| {
        if code.contains('=') {
            let (label, focal) = code.split_once('=').unwrap();
            let focal = focal.parse::<usize>().unwrap();
            let bx = hash(label);
            if let Some(data) = boxes.get_mut(&bx) {
                match index(data, label) {
                    Some(idx) => data[idx].1 = focal,
                    None => data.push((label, focal)),
                }
            } else {
                boxes.insert(bx, vec![(label, focal)]);
            }
        } else if code.contains('-') {
            let (label, _) = code.split_once('-').unwrap();
            let bx = hash(label);
            if let Some(data) = boxes.get_mut(&bx) {
                if let Some(index) = index(data, label) {
                    data.remove(index);
                }
            }
        } else {
            panic!()
        }
    });
    boxes
        .iter()
        .map(|(i, data)| {
            data.iter()
                .enumerate()
                .map(|(slot, &(_, val))| (slot + 1) * val)
                .sum::<usize>()
                * (i + 1)
        })
        .sum::<usize>()
        .to_string()
}
