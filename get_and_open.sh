#!/usr/bin/env bash
function run () {
COOKIE=$(cat cookie.txt 2>/dev/null)
DAY=${DAY:-$(TZ=America/New_York date "+%d")}
DATADIR=${DATADIR:-src/data}
PUZZLEDIR=${PUZZLEDIR:-src/puzzles}
TEMPLATE=${TEMPLATE:-template.rs.txt}

DAY_DIGITS=$(date --date="$(date "+%y%m")$DAY" "+%-d")
OUTPUT=${DATADIR}/${DAY}.txt
DATAMOD=${DATADIR}/mod.rs
PUZZLEMOD=${PUZZLEDIR}/mod.rs
NEWPUZZLE=${PUZZLEDIR}/p${DAY}.rs

if [ $? -ne 0 ]; then
  echo No session cookie provided
  return
fi
if [ "${#DAY}" != "2" ]; then
  echo Improper day format
  return
fi
if [ -f "$OUTPUT" ]; then
  echo $OUTPUT exists
else
  echo Fetching input as $OUTPUT
  curl --cookie "session=$COOKIE" -o "$OUTPUT" "https://adventofcode.com/2023/day/${DAY_DIGITS}/input"
fi

# I tried to write a rust macro to make this easier. It was not easier.
grep "${DAY}.txt" "$DATAMOD" 2>&1 > /dev/null
if [ $? -eq 0 ]; then
  echo import found in $DATAMOD
else
  echo "pub const DAY_${DAY}_INPUT: &str = std::include_str!(\"${DAY}.txt\");" >> "$DATAMOD"
fi

grep "pub mod p${DAY}" "$PUZZLEMOD" 2>&1 > /dev/null
if [ $? -eq 0 ]; then
  echo import found in $PUZZLEMOD
else
  echo "pub mod p${DAY};" >> "$PUZZLEMOD"
fi

if [ -f "$NEWPUZZLE" ]; then
  echo $NEWPUZZLE exists
else
  cp "$TEMPLATE" "$NEWPUZZLE"
fi

firefox -new-tab "https://adventofcode.com/2023/day/$DAY_DIGITS"

}; run