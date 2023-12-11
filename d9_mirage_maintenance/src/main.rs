use aoc_utils::*;
use std::fs;

fn extrapolate_polynomial(line: &Vec<i64>) -> i64 {
    let mut degrees = vec![line.clone()];
    loop {
        let a = degrees
            .last()
            .unwrap()
            .windows(2)
            .map(|arr| arr[1] - arr[0])
            .collect::<Vec<_>>();
        let all_zeroes = a.iter().all(|&n| n == 0);
        degrees.push(a);
        if all_zeroes {
            break;
        }
    }
    // dbg!(&degrees);
    degrees.iter().map(|ys| ys.last().unwrap()).sum()
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').map(parse_ints).collect();

    let sum: i64 = input.iter().map(extrapolate_polynomial).sum();

    // println!("{:?}", &input);
    println!("{:?}", sum);
    submit(&sum.to_string(), false);
}
