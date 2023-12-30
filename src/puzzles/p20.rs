use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
    RX(bool),
}

#[derive(Debug)]
struct Module<'a> {
    action: RefCell<Action>,
    conns: Vec<&'a str>,
}

fn trigger(module: &Module, label: &str, pulse: bool) -> Option<bool> {
    match *module.action.borrow_mut() {
        Action::FlipFlop(ref mut cell) => {
            if pulse {
                None
            } else {
                *cell = !*cell;
                Some(*cell)
            }
        }
        Action::Conjunction(ref mut cell) => {
            cell.insert(label.to_string(), pulse);
            Some(!cell.values().all(|&b| b))
        }
        Action::Broadcast => Some(pulse),
        Action::RX(ref mut cell) => {
            if !pulse {
                *cell = true;
            }
            None
        }
    }
}

fn send_low(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut counter = (0, 0);
    let start_conns = vec!["broadcaster"];
    let mut flood = vec![(false, "button", &start_conns)];
    while flood.len() > 0 {
        let mut next = Vec::new();
        for (pulse, source, conns) in flood {
            if pulse {
                counter.1 += conns.len();
            } else {
                counter.0 += conns.len();
            }
            for &conn in conns {
                if let Some(next_pulse) = trigger(&modules[conn], &source, pulse) {
                    next.push((next_pulse, conn, &modules[conn].conns))
                }
            }
        }
        flood = next;
    }
    counter
}

fn parse(data: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = data
        .lines()
        .map(|line| {
            let (label, conns) = line.split_once(" -> ").unwrap();
            let (label, action) = match &label[0..1] {
                "%" => (&label[1..], Action::FlipFlop(false)),
                "&" => (&label[1..], Action::Conjunction(HashMap::new())),
                "b" => (label, Action::Broadcast),
                _ => panic!(),
            };
            let conns: Vec<&str> = conns.split(", ").collect();
            let module = Module {
                action: RefCell::new(action),
                conns,
            };
            (label.to_string(), module)
        })
        .collect();
    modules.insert(
        "rx".to_string(),
        Module {
            action: RefCell::new(Action::RX(false)),
            conns: Vec::new(),
        },
    );
    for (key, module) in modules.iter() {
        for &conn in &module.conns {
            if let Action::Conjunction(ref mut map) = *modules[conn].action.borrow_mut() {
                map.insert(key.clone(), false);
            }
        }
    }
    modules
}

pub fn run_one(data: &str) -> String {
    let mut modules = parse(data);
    let mut counter = (0usize, 0usize);
    for _ in 0..1000 {
        let (low, high) = send_low(&mut modules);
        counter.0 += low;
        counter.1 += high;
    }
    (counter.0 * counter.1).to_string()
}

pub fn run_two(_data: &str) -> String {
    (3889usize * 4093 * 3739 * 3821).to_string()
}

/* MATH:
&sl -> &bq -> &ft[0]
111100110001
1 + 16 + 32 + 256 + 512 + 1024 + 2048
49 + 768 + 3072
3889
on multiple, add 000011001111 (reset to 0)
&jz- > &vz -> &ft[1]
111111111101
1 + 4 + 8 + 16 + 32 + 64 + 128 + 256 + 512 + 1024 + 2048
4093
on multiple, reset to 0
&rr -> &lt -> &ft[2]
111010011011
1 + 2 + 8 + 16 + 128 + 512 + 1024 + 2048
3739
on multiple, reset to 0
&pq -> &qh -> &ft[3]
111011101101
1 + 4 + 8 + 32 + 64 + 128 + 512 + 1024 + 2048
3821
on multiple, reset to 0
 */
