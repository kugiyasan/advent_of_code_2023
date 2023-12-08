use aoc_utils::submit;
use std::{collections::HashMap, fs};

type Nodes<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_line(s: &str) -> (&str, (&str, &str)) {
    let a = s.split(" = ").collect::<Vec<_>>();
    let b = a[1][1..a[1].len() - 1].split(", ").collect::<Vec<_>>();

    (a[0], (b[0], b[1]))
}

fn find_n_step(input_sequence: &[u8], nodes: Nodes, key: &str) -> usize {
    let mut step = 0;
    let mut key = key;

    while key != "ZZZ" {
        let instruction = input_sequence[step % input_sequence.len()];
        let next = nodes[key];

        if instruction == b'L' {
            key = next.0;
        } else {
            key = next.1;
        }

        step += 1;
    }

    step
}

fn main() {
    let path = "input";
    // let path = "example2";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();
    let input_sequence = input[0].as_bytes();

    let nodes: Nodes = HashMap::from_iter(input[2..].iter().map(|line| parse_line(*line)));

    println!("{:?}", nodes);

    let step = find_n_step(input_sequence, nodes, "AAA");

    println!("{}", step);

    submit(&step.to_string(), false);
}
