#!/usr/bin/env sh

set -eo pipefail

# cookies.txt should contain a line with "session=<your session cookie>"
COOKIES=$(cat cookies.txt)
DAY=$1
NAME=$2
FOLDER="d$1_$2"

new_rust() {
    # Append the new workspace to Cargo.toml
    sed -E -i "s/(^    \".+\"$)/\1,\n    \"$FOLDER\"/g" Cargo.toml

    cargo new $FOLDER
    cd $FOLDER
    cargo add aoc_utils

    cat > src/main.rs << EOF
use std::fs;
use aoc_utils::submit;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();

    println!("{:?}", &input[0..5]);
    // submit("1", false);
}
EOF
}

new_python() {
    mkdir $FOLDER
    cd $FOLDER
    cat > main.py << EOF
import aoc_utils
from typing import List


PATH = "input"
# PATH = "example"


def part1(lines: List[str]) -> str:
    pass


def part2(lines: List[str]) -> str:
    pass


def submit(answer: str, part2: bool) -> None:
    part = "2" if part2 else "1"
    i = input(f"Press 'y' to submit '{answer}' for part {part}?: ")

    if i == "y":
        print("Submitting...")
        aoc_utils.submit(answer, part2)


def main() -> None:
    with open(PATH) as f:
        lines = f.readlines()
        submit(str(part1(lines)), False)
        # submit(str(part2(lines)), True)


if __name__ == "__main__":
    main()
EOF
}

main() {
    new_rust
    # new_python

    if [ -e "input" ]; then
        echo 'input file already exists' >&2
        exit 1
    fi

    curl -O --cookie $COOKIES "https://adventofcode.com/2023/day/$DAY/input"
}

main
