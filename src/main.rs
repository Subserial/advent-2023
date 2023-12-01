mod data;
mod puzzles;

fn main() {
    println!("P01-1: {}", puzzles::p01::execute_first(data::DAY_01_INPUT));
    println!("P01-2: {}", puzzles::p01::execute_second(data::DAY_01_INPUT));
}