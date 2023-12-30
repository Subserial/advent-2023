//const D_MIN: f64 = 7.0;
//const D_MAX: f64 = 27.0;
const D_MIN: f64 = 200000000000000.0;
const D_MAX: f64 = 400000000000000.0;

#[derive(Debug)]
struct Hail {
    pos: (f64, f64, f64),
    dir: (f64, f64, f64),
}

fn parse(data: &str) -> Vec<Hail> {
    data.lines()
        .map(|line| {
            let (pos, dir) = line.split_once(" @ ").unwrap();
            let mut pos_dims = pos
                .split(",")
                .map(|p| p.trim().parse::<i64>().unwrap() as f64);
            let pos = (
                pos_dims.next().unwrap(),
                pos_dims.next().unwrap(),
                pos_dims.next().unwrap(),
            );
            let mut dir_dims = dir
                .split(",")
                .map(|p| p.trim().parse::<i64>().unwrap() as f64);
            let dir = (
                dir_dims.next().unwrap(),
                dir_dims.next().unwrap(),
                dir_dims.next().unwrap(),
            );
            Hail { pos, dir }
        })
        .collect::<Vec<_>>()
}

pub fn run_one(data: &str) -> String {
    let stones = parse(data);
    let stone_xy = stones
        .iter()
        .map(|stone| {
            let (x_m, x_b, y_m, y_b) = (stone.dir.0, stone.pos.0, stone.dir.1, stone.pos.1);
            let slope = y_m / x_m;
            let x_val = -(y_b / y_m) * x_m + x_b;
            let y_int = -slope * x_val;
            (slope, y_int)
        })
        .collect::<Vec<_>>();
    let mut crossings = 0;
    for first in 0..stones.len() {
        let hail_a = &stones[first];
        let path_a = &stone_xy[first];
        for second in first+1..stones.len() {
            let hail_b = &stones[second];
            let path_b = &stone_xy[second];
            if path_a.0 == path_b.0 {
                println!("PARALLEL: {} + {}", first, second);
                if path_a.1 == path_b.1 {
                    crossings += 1;
                }
                continue;
            }
            let potential_x = (path_a.1 - path_b.1) / (path_b.0 - path_a.0);
            let potential_y = path_a.0 * potential_x + path_a.1;
            if (potential_x < D_MIN || D_MAX < potential_x)
                || (potential_y < D_MIN || D_MAX < potential_y)
            {
                continue;
            }
            if ((hail_a.pos.0 != potential_x)
                && (hail_a.dir.0 >= 0.0) != (hail_a.pos.0 < potential_x))
                || ((hail_a.pos.1 != potential_x)
                    && (hail_a.dir.1 >= 0.0) != (hail_a.pos.1 <= potential_y))
                || ((hail_b.pos.0 != potential_x)
                    && (hail_b.dir.0 >= 0.0) != (hail_b.pos.0 <= potential_x))
                || ((hail_b.pos.1 != potential_x)
                    && (hail_b.dir.1 >= 0.0) != (hail_b.pos.1 <= potential_y))
            {
                continue;
            }
            crossings += 1;
        }
    }
    crossings.to_string()
}




pub fn run_two(data: &str) -> String {
    let stones = parse(data);
    let stone_xy = stones
        .iter()
        .map(|stone| {
            let (x_m, x_b, y_m, y_b) = (stone.dir.0, stone.pos.0, stone.dir.1, stone.pos.1);
            let slope = y_m / x_m;
            let x_val = -(y_b / y_m) * x_m + x_b;
            let y_int = -slope * x_val;
            (slope, y_int)
        })
        .collect::<Vec<_>>();
    let stone_xz = stones
        .iter()
        .map(|stone| {
            let (x_m, x_b, y_m, y_b) = (stone.dir.0, stone.pos.0, stone.dir.2, stone.pos.2);
            let slope = y_m / x_m;
            let x_val = -(y_b / y_m) * x_m + x_b;
            let y_int = -slope * x_val;
            (slope, y_int)
        })
        .collect::<Vec<_>>();
    let stone_yz = stones
        .iter()
        .map(|stone| {
            let (x_m, x_b, y_m, y_b) = (stone.dir.1, stone.pos.1, stone.dir.2, stone.pos.2);
            let slope = y_m / x_m;
            let x_val = -(y_b / y_m) * x_m + x_b;
            let y_int = -slope * x_val;
            (slope, y_int)
        })
        .collect::<Vec<_>>();
    for first in 0..stones.len() {
        let path_a = &stone_xy[first];
        for second in first+1..stones.len() {
            let path_b = &stone_xy[second];
            if path_a.0 == path_b.0 {
                println!("PARALLEL XY: {:?} + {:?}", &stones[first], &stones[second]);
            }
        }
    }
    for first in 0..stones.len() {
        let path_a = &stone_xz[first];
        for second in first+1..stones.len() {
            let path_b = &stone_xz[second];
            if path_a.0 == path_b.0 {
                println!("PARALLEL XZ: {:?} + {:?}", &stones[first], &stones[second]);
            }
        }
    }
    for first in 0..stones.len() {
        let path_a = &stone_yz[first];
        for second in first+1..stones.len() {
            let path_b = &stone_yz[second];
            if path_a.0 == path_b.0 {
                println!("PARALLEL YZ: {:?} + {:?}", &stones[first], &stones[second]);
            }
        }
    }





    "0".to_string()
}
