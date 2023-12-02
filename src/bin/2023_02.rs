use advent_of_code_2023::{Cli, Parser};
use std::fs;
use std::str::FromStr;

struct SubGame {
    red: i64,
    green: i64,
    blue: i64,
}

impl FromStr for SubGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for item in s.split(", ") {
            let (n, c) = item.split_once(' ').ok_or("item split failed")?;
            let n: i64 = n.parse().or(Err("parse n failed"))?;

            match c {
                "red" => red = n,
                "green" => green = n,
                "blue" => blue = n,
                _ => return Err("invalid colour".into()),
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (identifier, subgames) = s.split_once(": ").ok_or("game split failed")?;

        let id: i64 = identifier
            .split_once(' ')
            .ok_or("id split failed")?
            .1
            .parse()
            .or(Err("id parse failed"))?;

        let subgames = subgames
            .split("; ")
            .map(|s| s.parse())
            .collect::<Result<Vec<SubGame>, _>>()?;

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
}
