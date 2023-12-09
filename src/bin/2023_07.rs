#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use std::cmp::Ordering;
use std::fs;

#[derive(Eq, PartialEq)]
struct Hand {
    bid: u64,
    strength_p1: HandType,
    strength_p2: HandType,
    cards: [u8; 5],
}

#[repr(u8)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_strength<const PART: u8>(cards: &[u8]) -> HandType {
    let mut card_counts = [0_usize; 14];

    let mut joker_count = 0;
    for card in cards {
        if PART == 2 && card == &b'J' {
            joker_count += 1;
        } else {
            let mapped = card_strength::<PART>(*card);
            card_counts[mapped as usize] += 1;
        }
    }

    let (_, second_most_common, tail) = card_counts.select_nth_unstable(12);

    // Always beneficial to add jokers to most common card type
    let most_common = tail[0] + joker_count;

    match (most_common, second_most_common) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::Pair,
        _ => HandType::HighCard,
    }
}

fn card_strength<const PART: u8>(card: u8) -> u8 {
    match card {
        b'T' => 9,
        b'J' => {
            if PART == 1 {
                10
            } else {
                0
            }
        }
        b'Q' => 11,
        b'K' => 12,
        b'A' => 13,
        card => card - b'1',
    }
}

fn compare_hands<const PART: u8>(hand1: &Hand, hand2: &Hand) -> Ordering {
    let strength1 = if PART == 1 {
        hand1.strength_p1
    } else {
        hand1.strength_p2
    };
    let strength2 = if PART == 1 {
        hand2.strength_p1
    } else {
        hand2.strength_p2
    };

    match strength1.cmp(&strength2) {
        Ordering::Equal => {
            for (c1, c2) in hand1.cards.iter().zip(hand2.cards.iter()) {
                let card_strength_1 = card_strength::<PART>(*c1);
                let card_strength_2 = card_strength::<PART>(*c2);

                match card_strength_1.cmp(&card_strength_2) {
                    Ordering::Equal => {}
                    o => return o,
                }
            }
            Ordering::Equal
        }
        o => o,
    }
}

fn parse_line(inp: &str) -> Hand {
    let (cards_str, bid_str) = inp.split_once(' ').expect("invalid format");

    let cards: Vec<u8> = cards_str.bytes().collect();

    Hand {
        strength_p1: hand_strength::<1>(&cards),
        strength_p2: hand_strength::<2>(&cards),
        cards: cards.try_into().expect("invalid card length"),
        bid: bid_str.parse().expect("invalid bid"),
    }
}

fn parse(raw_inp: &str) -> Vec<Hand> {
    raw_inp.trim().lines().map(parse_line).collect()
}

fn calculate<const PART: u8>(data: &mut [Hand]) -> u64 {
    data.sort_unstable_by(compare_hands::<PART>);

    data.iter().zip(1..).map(|(hand, idx)| idx * hand.bid).sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let mut data = parse(&inp);
    let p1 = calculate::<1>(&mut data);
    let p2 = calculate::<2>(&mut data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_07");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_07");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate::<1>(&mut parse(EXAMPLE_DATA)), 6440);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate::<2>(&mut parse(EXAMPLE_DATA)), 5905);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&mut parse(REAL_DATA)), 251216224);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2>(&mut parse(REAL_DATA)), 250825971);
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench_parse(b: &mut Bencher) {
            b.iter(|| parse(black_box(REAL_DATA)));
        }

        #[bench]
        fn bench_p1_with_parse(b: &mut Bencher) {
            b.iter(|| calculate::<1>(black_box(&mut parse(REAL_DATA))));
        }

        #[bench]
        fn bench_p2_with_parse(b: &mut Bencher) {
            b.iter(|| calculate::<2>(black_box(&mut parse(REAL_DATA))));
        }
    }
}
