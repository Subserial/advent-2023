use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Result {
    Accept,
    Reject,
    Workflow(String),
}

struct Part(i64, i64, i64, i64);

fn parse_prop(prop: &str) -> Box<dyn Fn(&Part) -> bool> {
    let access: Box<dyn Fn(&Part) -> i64> = Box::new(match &prop[0..1] {
        "x" => |p| p.0,
        "m" => |p| p.1,
        "a" => |p| p.2,
        "s" => |p| p.3,
        _ => panic!(),
    });
    let compare: Box<dyn Fn(&i64, &i64) -> bool> = Box::new(match &prop[1..2] {
        ">" => i64::gt,
        "<" => i64::lt,
        _ => panic!(),
    });
    let val = prop[2..].parse::<i64>().unwrap();
    Box::new(move |part| compare(&access(part), &val))
}

fn parse_target(target: &str) -> Result {
    match target {
        "A" => Result::Accept,
        "R" => Result::Reject,
        _ => Result::Workflow(target.to_string()),
    }
}

pub fn run_one(data: &str) -> String {
    let (workflows, parts) = data.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (label, rest) = line.split_once("{").unwrap();
            let rules = rest
                .split(",")
                .map(|rule| match rule.split_once(":") {
                    Some((prop, val)) => {
                        let prop = parse_prop(prop);
                        let target = parse_target(val);
                        (prop, target)
                    }
                    None => {
                        let prop: Box<dyn Fn(&Part) -> bool> = Box::new(|_| true);
                        let target = parse_target(&rule[..rule.len() - 1]);
                        (prop, target)
                    }
                })
                .collect::<Vec<_>>();
            let find_target: Box<dyn Fn(&Part) -> Result> = Box::new(move |part: &Part| {
                for (rule, target) in &rules {
                    if rule(part) {
                        return target.clone();
                    }
                }
                panic!();
            });
            (label, find_target)
        })
        .collect::<HashMap<&str, Box<dyn Fn(&Part) -> Result>>>();
    let parts = parts
        .lines()
        .map(|line| {
            let vals = line[1..line.len() - 1]
                .split(",")
                .map(|val| {
                    let (_, val) = val.split_once('=').unwrap();
                    val.parse().unwrap()
                })
                .collect::<Vec<_>>();
            Part(vals[0], vals[1], vals[2], vals[3])
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    for part in parts {
        let mut target = Result::Workflow("in".to_string());
        while let Result::Workflow(label) = target {
            target = workflows[label.as_str()](&part);
            println!("{:?}", target);
        }
        if let Result::Accept = target {
            total += part.0 + part.1 + part.2 + part.3
        }
    }
    total.to_string()
}

#[derive(Clone)]
struct XmasRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

fn product(range: &XmasRange) -> i64 {
    if range.x.0 > range.x.1
        || range.m.0 > range.m.1
        || range.a.0 > range.a.1
        || range.s.0 > range.s.1
    {
        return 0;
    }
    (range.x.1 - range.x.0 + 1)
        * (range.m.1 - range.m.0 + 1)
        * (range.a.1 - range.a.0 + 1)
        * (range.s.1 - range.s.0 + 1)
}

struct Action(char, i64, bool, Result);

fn apply_action(action: &Action, range_pass: &mut (i64, i64), range_fail: &mut (i64, i64)) {
    match action.2 {
        true => {
            range_pass.0 = range_pass.0.max(action.1 + 1);
            range_fail.1 = range_fail.1.min(action.1);
        }
        false => {
            range_pass.1 = range_pass.1.min(action.1 - 1);
            range_fail.0 = range_fail.0.max(action.1);
        }
    }
}

fn split_prop(range: XmasRange, actions: &Vec<Action>) -> Vec<(XmasRange, Result)> {
    let mut put = Vec::new();
    let mut ranges = vec![range];
    for action in actions {
        let search = ranges;
        ranges = Vec::new();
        for mut range in search {
            let mut range_pass = range.clone();
            match action.0 {
                ' ' => {
                    put.push((range, action.3.clone()));
                    continue;
                }
                'x' => apply_action(&action, &mut range_pass.x, &mut range.x),
                'm' => apply_action(&action, &mut range_pass.m, &mut range.m),
                'a' => apply_action(&action, &mut range_pass.a, &mut range.a),
                's' => apply_action(&action, &mut range_pass.s, &mut range.s),
                _ => panic!(),
            }
            put.push((range_pass, action.3.clone()));
            ranges.push(range);
        }
    }
    put
}

pub fn run_two(data: &str) -> String {
    let (workflows, _) = data.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (label, rest) = line.split_once("{").unwrap();
            let rules = rest
                .split(",")
                .map(|rule| match rule.split_once(":") {
                    Some((prop, val)) => {
                        let field = prop.chars().nth(0).unwrap();
                        let is_gt = prop.chars().nth(1).unwrap() == '>';
                        let bound = prop[2..].parse::<i64>().unwrap();
                        let target = parse_target(val);
                        Action(field, bound, is_gt, target)
                    }
                    None => {
                        let target = parse_target(&rule[..rule.len() - 1]);
                        Action(' ', 0, false, target)
                    }
                })
                .collect::<Vec<_>>();
            (label, rules)
        })
        .collect::<HashMap<&str, Vec<Action>>>();
    let mut total = 0;
    let mut ranges = vec![(
        "in".to_string(),
        XmasRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    )];
    while ranges.len() > 0 {
        let search = ranges;
        ranges = Vec::new();
        for (label, range) in search {
            for (range, target) in split_prop(range, &workflows[label.as_str()]) {
                match target {
                    Result::Accept => total += product(&range),
                    Result::Reject => (),
                    Result::Workflow(next_label) => ranges.push((next_label, range)),
                }
            }
        }
    }
    total.to_string()
}
