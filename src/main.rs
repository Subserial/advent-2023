mod puzzles;
mod today;

fn main() {
    println!("Puzzle 1: {}", today::run_one(today::INPUT));
    println!("Puzzle 2: {}", today::run_two(today::INPUT));
}
