use aoc_utils::submit;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    // FiveOfAKind([usize; 1]),
    // FourOfAKind([usize; 2]),
    // FullHouse([usize; 2]),
    // ThreeOfAKind([usize; 3]),
    // TwoPairs([usize; 3]),
    // OnePair([usize; 4]),
    // HighCard([usize; 5]),
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
    // let cards = b"23456789TJQKA";

    let hand = hand.bytes().collect::<Vec<_>>();

    let counts = cards
        .iter()
        .map(|&card| hand.iter().filter(|&&h| h == card).count())
        .collect::<Vec<_>>();

    let hand = hand
        .iter()
        .map(|h| cards.iter().position(|c| c == h).unwrap())
        .collect::<Vec<_>>();

    let two_index = counts.iter().position(|&count| count == 2);
    let three_index = counts.iter().position(|&count| count == 3);
    let four_index = counts.iter().position(|&count| count == 4);
    let five_index = counts.iter().position(|&count| count == 5);

    let pairs = counts
        .iter()
        .filter(|&&count| count == 2)
        .collect::<Vec<_>>();
    let singles = counts.iter().filter(|&&c| c == 1).collect::<Vec<_>>();

    if let Some(i) = two_index {
        if let Some(j) = three_index {
            // return Hand::FullHouse([i, j]);
            return Hand {
                kind: HandType::FullHouse,
                bytes: hand,
            };
        }
    }

    let max_of_a_kind = *counts.iter().max().unwrap();
    println!("{}", max_of_a_kind);

    if max_of_a_kind == 5 {
        // return Hand::FiveOfAKind([five_index.unwrap()]);
        return Hand {
            kind: HandType::FiveOfAKind,
            bytes: hand,
        };
    } else if max_of_a_kind == 4 {
        // return Hand::FourOfAKind([four_index.unwrap(), *singles[0]]);
        return Hand {
            kind: HandType::FourOfAKind,
            bytes: hand,
        };
    } else if max_of_a_kind == 3 {
        // return Hand::ThreeOfAKind([three_index.unwrap(), *singles[0], *singles[1]]);
        return Hand {
            kind: HandType::ThreeOfAKind,
            bytes: hand,
        };
    }

    if pairs.len() == 2 {
        // return Hand::TwoPairs([*pairs[0], *pairs[1], *singles[0]]);
        return Hand {
            kind: HandType::TwoPairs,
            bytes: hand,
        };
    } else if pairs.len() == 1 {
        // return Hand::OnePair([*pairs[0], *singles[0], *singles[1], *singles[2]]);
        return Hand {
            kind: HandType::OnePair,
            bytes: hand,
        };
    }

    // Hand::HighCard([0; 5])

    return Hand {
        kind: HandType::HighCard,
        bytes: hand,
    };
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
        .map(|(i, (hand, bid))| (i as i32 + 1) * bid)
        .collect::<Vec<_>>();
    let sum: i32 = s.iter().sum();

    println!("{:?} {:?} {}", &input[0..5], s, sum);
    submit(&sum.to_string(), false);
}
