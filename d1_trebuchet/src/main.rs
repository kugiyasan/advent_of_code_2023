use aoc_utils::submit;
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();

    let n: i32 = input
        .iter()
        .map(|line| {
            let line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");

            let line: String = line.chars().filter(|c| c.is_digit(10)).collect();
            println!("{:?}", line);
            let line = line.chars().take(1).collect::<String>()
                + &line.chars().last().unwrap().to_string();
            println!("{:?}", line);
            line.parse::<i32>().unwrap()
        })
        .sum();

    println!("{:?}", &input[0..5]);
    println!("{}", n);
    submit(&n.to_string(), true);
}
