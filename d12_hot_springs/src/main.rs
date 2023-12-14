use aoc_utils::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs;

fn create_arrangement(record: &str, choice: &Vec<u8>) -> Vec<u8> {
    let mut i = 0;

    record
        .bytes()
        .map(|c| {
            if c == b'?' {
                i += 1;
                choice[i - 1]
            } else {
                c
            }
        })
        .collect()
}

fn consecutive_filled_squares(arrangement: &[u8]) -> Vec<i64> {
    let mut result = vec![];
    let mut count = 0;

    for c in arrangement {
        if *c == b'#' {
            count += 1;
        } else {
            if count > 0 {
                result.push(count);
            }
            count = 0;
        }
    }

    if count > 0 {
        result.push(count);
    }

    result
}

fn solve(input: Vec<(&str, Vec<i64>)>) -> i64 {
    input
        .iter()
        // .par_iter()
        .map(|(record, group)| {
            let unknown = record.bytes().filter(|&c| c == b'?').count();
            let known_damaged = record.bytes().filter(|&c| c == b'#').count();
            let total_damaged = group.iter().sum::<i64>() as usize;
            let damaged = vec![b'#'; total_damaged - known_damaged];
            let operational = vec![b'.'; unknown - damaged.len()];
            let choices = [damaged, operational].concat();

            // dbg!(unknown, known_damaged, total_damaged);
            // println!("{:?}", choices);

            let choices = choices.into_iter().permutations(unknown).unique();
            let mut sum = 0;

            for choice in choices {
                let arrangement = create_arrangement(record, &choice);

                let g = consecutive_filled_squares(&arrangement);

                // println!("{:?} {:?} {:?}", arrangement, g, group);
                if group == &g {
                    sum += 1;
                }
                // println!("{:?}", c);
            }

            println!("u={}\ts={}", unknown, sum);
            sum
        })
        .sum()
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();
    // let buf = "???.### 1,1,3".to_string();
    let buf = "?????????.?.#???#?? 7,1,1,2".to_string();

    let input: Vec<_> = buf
        .trim()
        .split('\n')
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| (line[0], parse_ints(line[1])))
        .collect();

    let sum = solve(input);

    // println!("{:?}", &input[0..5]);
    println!("{:?}", sum);
    // submit("1", false);
}
