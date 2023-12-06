use aoc_utils::*;
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf
        .trim()
        .split('\n')
        .map(|line| line.replace(' ', "")) // comment this line for part 1
        .map(|line| parse_ints(&line))
        .collect();
    let input: Vec<_> = input[0].iter().zip(&input[1]).collect();

    let res: i64 = input
        .iter()
        .map(|(&time, &distance)| {
            for t in 1..time {
                let d = t * (time - t);
                if d > distance {
                    return time - t * 2 + 1;
                }
            }
            1
        })
        .product();

    println!("{:?} {}", input, res);
    submit(&res.to_string(), false);
}
