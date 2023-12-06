use advent_of_code_2023::{Cli, Parser};
use anyhow::*;
use itertools::*;
use std::cmp::min;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Mapping {
    dest: i64,
    src: i64,
    range: i64,
}

#[derive(Debug)]
struct SeedMap {
    mapping: Vec<Mapping>,
}

#[derive(Debug)]
struct Data {
    seeds: Vec<i64>,
    maps: Vec<SeedMap>,
}

impl FromStr for Mapping {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let nums = s
            .split_whitespace()
            .map(|num| num.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Mapping {
            dest: nums[0],
            src: nums[1],
            range: nums[2],
        })
    }
}

impl FromStr for SeedMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mapping = s
            .lines()
            .skip(1) // Header, not required to solve problem.
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SeedMap { mapping })
    }
}

impl FromStr for Data {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let seeds = s
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("seeds:"))
            .map(|line| line.trim())
            .and_then(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .ok_or(Error::msg("seeds parse failed"))?;

        let maps = s
            .split("\n\n")
            .skip(1) // Seeds
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Data { seeds, maps })
    }
}

fn parse(raw_inp: &str) -> Data {
    raw_inp.parse().expect("input parse failed")
}

fn apply_map(data: &Data, mut seed: i64) -> i64 {
    for mp in data.maps.iter() {
        for m in mp.mapping.iter() {
            if seed >= m.src && seed < m.src + m.range {
                seed = seed + m.dest - m.src;
                break;
            }
        }
    }

    seed
}

fn calculate_p1(data: &Data) -> i64 {
    data.seeds
        .iter()
        .map(|&seed| apply_map(data, seed))
        .min()
        .expect("should have at least one seed")
}

// Algorithm:
// apply transformation as normal, but also keep track of the next "interesting" decision point
// so that we can jump directly to that decision point next iteration.
fn best_seed_from_range(data: &Data, start: i64, range: i64) -> i64 {
    let mut result = i64::MAX;
    let mut seed = start;
    while seed <= start + range {
        let mut increment = i64::MAX;
        let mut s = seed;

        for mp in data.maps.iter() {
            for m in mp.mapping.iter() {
                if s < m.src {
                    increment = min(increment, m.src - s);
                }
            }
            for m in mp.mapping.iter() {
                if s >= m.src && s < m.src + m.range {
                    s = s + m.dest - m.src;
                    break;
                }
            }
        }
        result = min(result, s);
        seed = seed.saturating_add(increment);
    }
    result
}

fn calculate_p2(data: &Data) -> i64 {
    data.seeds
        .iter()
        .tuples()
        .map(|(&start, &range)| best_seed_from_range(data, start, range))
        .min()
        .expect("expected at least one seed range")
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_05");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_05");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 35);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 46);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 318728750);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 37384986);
    }
}
