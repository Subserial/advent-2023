## Advent Of Code 2023, Rust Edition

Running `get.sh` will initiate work on a puzzle given a session cookie in `cookie.txt`:
- Retrieve the puzzle input as `src/data/<DAY>.txt`
- Append a constant to `src/data/mod.rs` to load the input contents.
- Append `mod p<DAY>` to `src/data/puzzles/mod.rs`.
- Copy `template.rs.txt` to `src/data/puzzles/p<DAY>.rs`.
- Open the puzzle in Firefox.

Safeguards against existing values and files are in place.