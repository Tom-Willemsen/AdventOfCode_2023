use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    cards: Vec<u8>,
    bid: u64,
}

fn hand_strength<const PART: u8>(hand: &Hand) -> u8 {
    let joker_count = if PART == 1 {
        0
    } else {
        hand.cards.iter().filter(|&&c| c == b'J').count()
    };

    let mut card_counts = AHashMap::with_capacity(5);

    hand.cards
        .iter()
        .filter(|&&c| PART == 1 || c != b'J')
        .for_each(|c| {
            card_counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        });

    if joker_count >= 4 || card_counts.values().any(|&count| count >= 5 - joker_count) {
        // Five-of-a-kind
        return 6;
    }

    if joker_count >= 3 || card_counts.values().any(|&count| count >= 4 - joker_count) {
        // Four-of-a-kind
        return 5;
    }

    let is_natural_full_house = card_counts.values().any(|&count| count == 3)
        && card_counts.values().any(|&count| count == 2);

    let has_two_natural_pairs = card_counts.values().filter(|&&count| count == 2).count() == 2;

    let can_make_full_house = joker_count == 1 && has_two_natural_pairs;

    if is_natural_full_house || can_make_full_house {
        // Full house
        return 4;
    }

    if joker_count >= 2 || card_counts.values().any(|&count| count >= 3 - joker_count) {
        // Three-of-a-kind
        return 3;
    }

    if has_two_natural_pairs {
        // Two-pair
        return 2;
    }

    if joker_count >= 1 || card_counts.values().any(|&count| count >= 2 - joker_count) {
        // One-pair
        return 1;
    }

    0
}

fn card_strength<const PART: u8>(card: u8) -> u8 {
    match card {
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'T' => 10,
        b'J' => {
            if PART == 1 {
                11
            } else {
                1
            }
        }
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => panic!("invalid card"),
    }
}

fn compare_hands<const PART: u8>(hand1: &Hand, hand2: &Hand) -> Ordering {
    let strength1 = hand_strength::<PART>(hand1);
    let strength2 = hand_strength::<PART>(hand2);

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

    Hand {
        cards: cards_str.bytes().collect(),
        bid: bid_str.parse().expect("invalid bid"),
    }
}

fn parse(raw_inp: &str) -> Vec<Hand> {
    raw_inp.trim().lines().map(parse_line).collect()
}

fn calculate<const PART: u8>(data: &mut [Hand]) -> u64 {
    data.sort_by(compare_hands::<PART>);

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
}
