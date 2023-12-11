use aoc_utils::*;
use std::fs;

fn extrapolate_polynomial(line: &Vec<i64>, part2: bool) -> i64 {
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
    if !part2 {
        degrees.iter().map(|ys| ys.last().unwrap()).sum()
    } else {
        degrees
            .iter()
            .map(|ys| *ys.first().unwrap())
            .rev()
            .fold(0, |acc, e| e - acc)
    }
}

pub trait PrintIterator<T> {
    fn print(&mut self, message: &str) -> Box<dyn Iterator<Item = T>>;
}

// TODO: remove 'static, and pass self instead
impl<T: std::fmt::Debug + 'static, U: Iterator<Item = T>> PrintIterator<T> for U {
    fn print(&mut self, message: &str) -> Box<dyn Iterator<Item = T>> {
        let v = self.collect::<Vec<_>>();
        println!("{}: {:?}", message, v);
        Box::new(v.into_iter())
    }
}

fn main() {
    let path = "input";
    // let path = "example";
    let part2 = true;
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').map(parse_ints).collect();

    let sum: i64 = input
        .iter()
        .map(|line| extrapolate_polynomial(line, part2))
        .print("lines")
        .sum();

    // println!("{:?}", &input);
    println!("{:?}", sum);
    submit(&sum.to_string(), part2);
}
