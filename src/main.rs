mod puzzles;
mod today;

fn main() {
    let now = std::time::Instant::now();
    println!(
        "Puzzle 1: {} ({}s)",
        today::run_one(today::INPUT),
        now.elapsed().as_secs()
    );
    let now = std::time::Instant::now();
    println!(
        "Puzzle 2: {} ({}s)",
        today::run_two(today::INPUT),
        now.elapsed().as_secs()
    );
}
