use aoc_utils::submit;
use std::str;
use std::{fmt::Debug, fs};

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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: HandType,
    bytes: Vec<usize>,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.bytes.iter().map(|&b| CARDS[b]).collect::<Vec<_>>();
        f.debug_struct("Hand")
            .field("kind", &self.kind)
            .field("bytes", &str::from_utf8(&b).unwrap())
            .finish()
    }
}

// const CARDS: &'static [u8] = b"AKQJT98765432";
const CARDS: &'static [u8] = b"AKQT98765432J";

fn parse_hand(hand: &str) -> Hand {
    // 2J57T
    let hand = hand.bytes().collect::<Vec<_>>();

    let counts = CARDS
        .iter()
        .map(|&card| hand.iter().filter(|&&h| h == card).count())
        .collect::<Vec<_>>();

    let counts_without_joker = &counts[0..counts.len() - 1];

    let hand = hand
        .iter()
        .map(|h| CARDS.iter().position(|c| c == h).unwrap())
        .collect::<Vec<_>>();

    let joker_count = *counts.last().unwrap();
    // println!("joker {}", joker_count);

    let pairs_count = counts.iter().filter(|&&count| count == 2).count();

    let max_of_a_kind = *counts_without_joker.iter().max().unwrap();

    if counts_without_joker.contains(&2) && counts_without_joker.contains(&3)
        || pairs_count == 2 && joker_count == 1
    {
        return Hand {
            kind: HandType::FullHouse,
            bytes: hand,
        };
    }

    let kind = match (max_of_a_kind + joker_count, pairs_count + joker_count) {
        (n, _) if n >= 5 => HandType::FiveOfAKind,
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
    // let path = "example2";
    let buf = fs::read_to_string(path).unwrap();

    let mut input: Vec<(Hand, i32)> = buf
        .trim()
        .split('\n')
        .map(|line| line.split(' ').collect::<Vec<_>>())
        .map(|line| (parse_hand(line[0]), line[1].parse().unwrap()))
        .collect();

    println!(
        "{}",
        &input[0..]
            .iter()
            .map(|t| format!("{:?}\n", t))
            // .filter(|s| s.contains('J'))
            .collect::<String>()
    );

    input.sort_by(|a, b| b.0.cmp(&a.0));

    let s = input
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as i32 + 1) * bid)
        .collect::<Vec<_>>();
    let sum: i32 = s.iter().sum();

    // println!(
    //     "{}",
    //     &input[0..10]
    //         .iter()
    //         .map(|t| format!("{:?}\n", t))
    //         .collect::<String>()
    // );
    // println!("{:?}", s);
    println!("sum {}", sum);
    submit(&sum.to_string(), true);
}
