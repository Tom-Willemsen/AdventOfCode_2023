#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use anyhow::*;
use std::fs;
use std::str::FromStr;

struct SubGame {
    red: i64,
    green: i64,
    blue: i64,
}

impl FromStr for SubGame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for item in s.split(", ") {
            let (n, c) = item
                .split_once(' ')
                .ok_or(Error::msg("failed item split"))?;
            let n: i64 = n.parse()?;

            match c {
                "red" => red = n,
                "green" => green = n,
                "blue" => blue = n,
                _ => bail!("invalid colour"),
            }
        }

        Ok(SubGame { red, green, blue })
    }
}

struct Game {
    id: i64,
    subgames: Vec<SubGame>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (identifier, subgames) = s.split_once(": ").ok_or(Error::msg("game split failed"))?;

        let id: i64 = identifier
            .split_once(' ')
            .ok_or(Error::msg("id split failed"))?
            .1
            .parse()?;

        let subgames = subgames
            .split("; ")
            .map(|s| s.parse())
            .collect::<Result<Vec<SubGame>>>()?;

        Ok(Game { id, subgames })
    }
}

fn parse(raw_inp: &str) -> Vec<Game> {
    raw_inp.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn calculate_p1(data: &[Game]) -> i64 {
    data.iter()
        .filter(|game| {
            game.subgames
                .iter()
                .all(|subgame| subgame.red <= 12 && subgame.green <= 13 && subgame.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

fn calculate_p2(data: &[Game]) -> i64 {
    data.iter()
        .map(|game| {
            let min_red = game
                .subgames
                .iter()
                .map(|subgame| subgame.red)
                .max()
                .expect("no red");
            let min_green = game
                .subgames
                .iter()
                .map(|subgame| subgame.green)
                .max()
                .expect("no green");
            let min_blue = game
                .subgames
                .iter()
                .map(|subgame| subgame.blue)
                .max()
                .expect("no blue");

            min_red * min_green * min_blue
        })
        .sum()
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_02");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_02");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 8);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 2286);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 2551);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 62811);
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
        fn bench_p1(b: &mut Bencher) {
            let parsed = parse(REAL_DATA);
            b.iter(|| calculate_p1(black_box(&parsed)));
        }

        #[bench]
        fn bench_p2(b: &mut Bencher) {
            let parsed = parse(REAL_DATA);
            b.iter(|| calculate_p2(black_box(&parsed)));
        }
    }
}
