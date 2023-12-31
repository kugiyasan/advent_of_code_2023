use aoc_utils::parse_ints;
use aoc_utils::submit;
use rayon::prelude::IntoParallelRefMutIterator;
use rayon::prelude::ParallelIterator;
use std::fs;

fn get_value_from_map(seed: i64, m: &Vec<Vec<i64>>) -> i64 {
    for row in m {
        let dest_start = row[0];
        let src_start = row[1];
        let len = row[2];

        let src = src_start..src_start + len;

        if src.contains(&seed) {
            return seed - src_start + dest_start;
        }
    }

    seed
}

fn main() {
    let path = "input";
    // let path = "example";
    let part2 = true;
    let buf = fs::read_to_string(path).unwrap();

    let mut input = buf
        .trim()
        .split("\n\n")
        .map(|m| m.split('\n').map(parse_ints).collect::<Vec<_>>());

    let seeds = &mut input.next().unwrap()[0];

    // comment for part 1
    let mut seeds = seeds
        .chunks(2)
        .flat_map(|r| (r[0]..r[0] + r[1]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("len {}", seeds.len());

    let maps: Vec<Vec<_>> = input.map(|m| m.into_iter().skip(1).collect()).collect();

    let min = seeds
        .par_iter_mut()
        .map(|seed| {
            let mut s = *seed;
            for m in &maps {
                s = get_value_from_map(s, m);
            }
            s
        })
        .min()
        .unwrap();

    submit(&min.to_string(), part2);
}
