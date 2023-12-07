#!/usr/bin/env bash
function run () {
DAY=${DAY:-$(TZ=America/New_York date "+%d")}

COOKIE=$(cat cookie.txt 2>/dev/null)
DAY_DIGITS=$(date --date="$(date "+%y%m")$DAY" "+%-d")
PUZZLEMOD=src/puzzles/mod.rs
NEWPUZZLE=src/puzzles/p${DAY}.rs

if [ $? -ne 0 ]; then
  echo No session cookie provided
  return
fi

if [ "${#DAY}" != "2" ]; then
  echo Improper day format
  return
fi

echo Fetching input as src/today.txt
curl --cookie "session=$COOKIE" -o "src/today.txt" "https://adventofcode.com/2023/day/${DAY_DIGITS}/input"

grep "pub mod p${DAY}" "$PUZZLEMOD" 2>&1 > /dev/null
if [ $? -eq 0 ]; then
  echo import found in $PUZZLEMOD
else
  echo "pub mod p${DAY};" >> "$PUZZLEMOD"
fi

if [ -f "$NEWPUZZLE" ]; then
  echo $NEWPUZZLE exists
else
  cp "template.rs.txt" "$NEWPUZZLE"
fi

cat > src/today.rs << EOF
pub use crate::puzzles::p$DAY::*;
pub const INPUT: &str = include_str!("today.txt");
EOF

xdg-open "https://adventofcode.com/2023/day/$DAY_DIGITS"

}; run