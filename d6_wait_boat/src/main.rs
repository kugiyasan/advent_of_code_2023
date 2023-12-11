use aoc_utils::*;
use d6_wait_boat::*;
use std::fs;

fn main() {
    // let path = "input";
    let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let res1 = iterator_version(&buf);
    let res2 = forloop_version(&buf);

    println!("{}", res1);
    println!("{}", res2);
    // println!("{:?} {}", input, res);
    // submit(&res.to_string(), false);
}
