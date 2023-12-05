use d4_scratchcards::*;
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();
    let input: Vec<_> = buf.trim().split('\n').collect();

    let sum = part1(&input);
    println!("part 1 sum = {sum}");

    let sum = part2(&input);
    println!("part 2 sum = {sum}");
}
