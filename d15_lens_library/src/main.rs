use aoc_utils::*;
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split(',').collect();

    let sum = input
        .iter()
        .map(|e| e.bytes().fold(0, |acc, c| (acc + c as i64) * 17 % 256))
        .sum::<i64>();

    // println!("{:?}", &input[0..5]);
    println!("{:?}", sum);
    // submit("1", false);
}
