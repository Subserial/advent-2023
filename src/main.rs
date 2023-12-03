mod data;
mod puzzles;

fn main() {
    println!("P01-1: {}", puzzles::p01::execute_first(data::DAY_01_INPUT));
    println!(
        "P01-2: {}",
        puzzles::p01::execute_second(data::DAY_01_INPUT)
    );
    println!("P02-1: {}", puzzles::p02::execute_first(data::DAY_02_INPUT));
    println!(
        "P02-2: {}",
        puzzles::p02::execute_second(data::DAY_02_INPUT)
    );
    println!("P03-1: {}", puzzles::p03::execute_first(data::DAY_03_INPUT));
    println!(
        "P03-2: {}",
        puzzles::p03::execute_second(data::DAY_03_INPUT)
    );
}
