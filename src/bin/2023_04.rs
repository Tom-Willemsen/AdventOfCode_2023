use advent_of_code_2023::{Cli, Parser};
use anyhow::*;
use std::fs;
use std::str::FromStr;

struct Game {
    winning_cards: Vec<u32>,
    cards: Vec<u32>,
}

fn parse_number_list(s: &str) -> Result<Vec<u32>> {
    Ok(s.split_whitespace()
        .map(|n| n.trim().parse())
        .collect::<Result<Vec<u32>, _>>()?)
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, numbers) = s.split_once(": ").ok_or(Error::msg("failed split"))?;

        let (winning_cards, cards) = numbers
            .split_once('|')
            .ok_or(Error::msg("failed card split"))?;

        let winning_cards = parse_number_list(winning_cards)?;
        let cards = parse_number_list(cards)?;

        Ok(Game {
            winning_cards,
            cards,
        })
    }
}

fn get_num_wins(game: Game) -> u32 {
    game.cards
        .into_iter()
        .filter(|x| game.winning_cards.contains(x))
        .count()
        .try_into()
        .expect("exceeded u32 range")
}

fn parse(raw_inp: &str) -> Vec<u32> {
    raw_inp
        .trim()
        .lines()
        .map(|line| line.parse().expect("failed parse"))
        .map(get_num_wins)
        .collect()
}

fn calculate_p1(data: &[u32]) -> u32 {
    data.iter()
        .filter(|&&wins| wins >= 1)
        .map(|wins| 2u32.pow(wins - 1))
        .sum()
}

fn calculate_p2(data: &[u32]) -> u32 {
    let mut n_cards = vec![1; data.len()];

    for (idx, num_wins) in data.iter().enumerate() {
        for x in 1..=*num_wins as usize {
            // precondition in puzzle description: will not get
            // out-of-bounds index here.
            n_cards[idx + x] += n_cards[idx];
        }
    }

    n_cards.iter().sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_04");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_04");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 13);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 30);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 21919);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 9881048);
    }
}
