## Advent Of Code 2023, Rust Edition

Running `get.sh` will initiate work on a puzzle given a session cookie in `cookie.txt`:
- Record today (according to New York time (wrong)) as `<DAY>`.
- Retrieve the puzzle input as `src/data/<DAY>.txt`
- Append a macro to `src/data/mod.rs` to load the input from the file.
- Append `pub mod p<DAY>;` to `src/data/puzzles/mod.rs`.
- Copy `template.rs.txt` to `src/data/puzzles/p<DAY>.rs`.
- Open the puzzle in Firefox.

Safeguards against existing values and files are in place.