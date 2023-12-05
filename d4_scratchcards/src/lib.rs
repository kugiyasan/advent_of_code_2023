use std::collections::HashSet;

pub fn get_wins_per_scratchcard<'a>(input: &'a Vec<&'a str>) -> impl Iterator<Item = usize> + 'a {
    input.iter().map(|line| {
        let line = line.split(": ").skip(1).next().unwrap();
        let mut line = line.split(" | ");
        let w = line
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap());
        let h = line
            .next()
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap());

        // let winning_numbers: HashSet<i32> = HashSet::from_iter(w);
        // let hand: HashSet<i32> = HashSet::from_iter(h);
        //
        // winning_numbers.intersection(&hand).count()

        let w = w.collect::<Vec<i32>>();
        h.filter(|n| w.contains(&n)).count()
    })
}

pub fn part1(input: &Vec<&str>) -> i32 {
    let arr = get_wins_per_scratchcard(input);
    let sum = arr
        // .into_iter()
        .map(|n| if n == 0 { 0 } else { 2i32.pow(n as u32 - 1) })
        .sum();
    sum
}

pub fn part2(input: &Vec<&str>) -> i32 {
    let len = input.len();
    let mut arr = get_wins_per_scratchcard(input);

    let mut cards = vec![1; len];

    for i in 0..len {
        for j in 0..arr.next().unwrap() {
            cards[i + j + 1] += cards[i]
        }
    }

    cards.into_iter().sum()
}
