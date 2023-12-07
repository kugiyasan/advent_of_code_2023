use aoc_utils::submit;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: HandType,
    bytes: Vec<usize>,
}

fn parse_hand(hand: &str) -> Hand {
    let cards = b"AKQJT98765432";

    let hand = hand.bytes().collect::<Vec<_>>();

    let counts = cards
        .iter()
        .map(|&card| hand.iter().filter(|&&h| h == card).count())
        .collect::<Vec<_>>();

    let hand = hand
        .iter()
        .map(|h| cards.iter().position(|c| c == h).unwrap())
        .collect::<Vec<_>>();

    let pairs_count = counts
        .iter()
        .filter(|&&count| count == 2)
        .count();

    let max_of_a_kind = *counts.iter().max().unwrap();

    if counts.contains(&2) && counts.contains(&3) {
        return Hand {
            kind: HandType::FullHouse,
            bytes: hand,
        };
    }

    let kind = match (max_of_a_kind, pairs_count) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, _) => HandType::ThreeOfAKind,
        (_, 2) => HandType::TwoPairs,
        (_, 1) => HandType::OnePair,
        _ => HandType::HighCard,
    };

    Hand { kind, bytes: hand }
}

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let mut input: Vec<(Hand, i32)> = buf
        .trim()
        .split('\n')
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| (parse_hand(line[0]), line[1].parse().unwrap()))
        .collect();

    input.sort_by(|a, b| b.0.cmp(&a.0));

    let s = input
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as i32 + 1) * bid)
        .collect::<Vec<_>>();
    let sum: i32 = s.iter().sum();

    println!("{:?} {:?} {}", &input[0..5], s, sum);
    submit(&sum.to_string(), false);
}
