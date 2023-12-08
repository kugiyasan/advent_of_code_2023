use aoc_utils::submit;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Key = String;
type Value = (Key, Key);

// type Key = String;
// type Value = (String, String);

// type Nodes<'a> = HashMap<&'a str, (&'a str, &'a str)>;
type Nodes = HashMap<Key, Value>;
// type Nodes<'a> = HashMap<&'a [u8], (&'a [u8], &'a [u8])>;

fn parse_line(s: &str) -> (Key, Value) {
    let a = s.split(" = ").collect::<Vec<_>>();
    let c = &a[1][1..a[1].len() - 1];
    let b = c.split(", ").collect::<Vec<_>>();

    // (a[0].as_bytes(), (b[0].as_bytes(), b[1].as_bytes()))
    (a[0].to_string(), (b[0].to_string(), b[1].to_string()))
}

fn create_preprocessing_hashmap(
    nodes: &Nodes,
    input_sequence: &[u8],
) -> HashMap<Key, (Key, HashSet<usize>)> {
    let mut after_a_whole_input_sequence: HashMap<Key, (Key, HashSet<usize>)> = HashMap::new();

    for (original_key, (left, right)) in nodes.iter() {
        let mut key = &original_key.to_string();
        let mut set = HashSet::new();

        for (i, letter) in input_sequence.iter().enumerate() {
            let next = &nodes[key];

            if *letter == b'L' {
                key = &next.0;
            } else {
                key = &next.1;
            }

            if key.ends_with('Z') {
                set.insert(i + 1);
            }
        }

        after_a_whole_input_sequence.insert(original_key.to_string(), (key.to_string(), set));
    }

    println!("{:?}", after_a_whole_input_sequence);
    after_a_whole_input_sequence
}

fn solve(nodes: Nodes, input_sequence: &[u8]) -> usize {
    let mut keys = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let after_a_whole_input_sequence = create_preprocessing_hashmap(&nodes, input_sequence);
    // hashmap string -> ((String, Set<int>), (String, Set<int>))
    // Set<int> n steps
    let mut step = 0;

    loop {
        let values = keys
            .iter_mut()
            .map(|key| &after_a_whole_input_sequence[*key])
            .collect::<Vec<_>>();

        // println!("{:?}", values);
        keys = values.iter().map(|v| &v.0).collect();

        let first = values[0].1.clone();
        let intersection = values.into_iter().fold(first, |acc, item| {
            acc.intersection(&item.1).map(|n| *n).collect()
        });
        
        // println!("{:?}", intersection);

        if intersection.len() > 0 {
            return step * input_sequence.len() + intersection.iter().min().unwrap();
        }

        step += 1;

        if step % 1000000 == 0 {
        // if step % 10 == 0 {
            println!("step {}", step);
            // return 0;
        }
    }
}

fn main() {
    let path = "input";
    // let path = "example3";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();
    let input_sequence = input[0].as_bytes();

    let nodes: Nodes = HashMap::from_iter(input[2..].iter().map(|line| parse_line(*line)));

    // println!("{:?}", nodes);
    // println!("{} {:?}", keys.len(), keys);

    let step = solve(nodes, input_sequence);

    println!("step {}", step);

    submit(&step.to_string(), true);
}
