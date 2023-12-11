use aoc_utils::*;

pub fn iterator_version(buf: &str) -> i64 {
    let input = buf
        .trim()
        .split('\n')
        .map(|line| line.replace(' ', "")) // comment this line for part 1
        .map(|line| parse_ints(&line))
        .collect::<Vec<_>>();

    input[0]
        .iter()
        .zip(&input[1])
        .map(|(&time, &distance)| {
            (1..time)
                .find(|&t| {
                    let d = t * (time - t);
                    d > distance
                })
                .map(|t| time - t * 2 + 1)
                .unwrap()
        })
        .product()
}

pub fn forloop_version(buf: &str) -> i64 {
    let mut lines = vec![];

    for line in buf.trim().split('\n') {
        let line = line.replace(' ', ""); // comment this line for part 1
        let line = parse_ints(&line);
        lines.push(line);
    }

    let mut time_distance = vec![];

    for i in 0..lines[0].len() {
        time_distance.push((lines[0][i], lines[1][i]));
    }

    let mut prod = 1;

    for (time, distance) in time_distance {
        for t in 1..time {
            let d = t * (time - t);
            if d > distance {
                prod *= time - t * 2 + 1;
                break;
            }
        }
    }

    prod
}
