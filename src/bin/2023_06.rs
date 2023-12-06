use advent_of_code_2023::{Cli, Parser};
use anyhow::*;
use std::fs;
use num_integer::*;

struct Race {
    time: i64,
    distance: i64,
}

fn parse_line_n(raw_inp: &str, n: usize) -> Vec<i64> {
    raw_inp
        .lines()
        .nth(n)
        .expect("No line to parse?")
        .split_ascii_whitespace()
        .skip(1) // "Time:"
        .map(|x| x.parse())
        .collect::<Result<Vec<i64>, _>>()
        .expect("Couldn't parse line")
}

fn parse(raw_inp: &str) -> Vec<Race> {
    let times = parse_line_n(raw_inp, 0);
    let distances = parse_line_n(raw_inp, 1);

    times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Race { time, distance })
        .collect()
}

fn ways_to_win(race: &Race) -> i64 {
    let win_dist = race.distance + 1;
    let discriminant = race.time * race.time - 4 * win_dist;

    let mut minimum_charge_time = (race.time - discriminant.sqrt()) / 2;
    let mut maximum_charge_time = (race.time + discriminant.sqrt()) / 2;
    
    if minimum_charge_time * (race.time - minimum_charge_time) < win_dist {
        minimum_charge_time += 1;
    }
    
    if (maximum_charge_time + 1) * (race.time - (maximum_charge_time + 1)) >= win_dist {
        maximum_charge_time += 1;
    }

    maximum_charge_time - minimum_charge_time + 1
}

fn calculate_p1(data: &[Race]) -> i64 {
    data.iter().map(ways_to_win).product()
}

fn next_power_of_10(n: i64) -> i64 {
    10_i64.pow(n.ilog10() + 1)
}

fn calculate_p2(data: Vec<Race>) -> i64 {
    data.into_iter()
        .reduce(|acc, e| Race {
            time: next_power_of_10(e.time) * acc.time + e.time,
            distance: next_power_of_10(e.distance) * acc.distance + e.distance,
        })
        .map(|race| ways_to_win(&race))
        .expect("expected at least one race")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_06");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_06");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 288);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(parse(EXAMPLE_DATA)), 71503);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 170000);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(parse(REAL_DATA)), 20537782);
    }
}
