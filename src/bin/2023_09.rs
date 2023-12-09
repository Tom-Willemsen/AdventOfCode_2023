use advent_of_code_2023::{Cli, Parser};
use itertools::Itertools;
use std::fs;

fn parse(raw_inp: &str) -> Vec<Vec<i64>> {
    raw_inp
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, _>>()
                .expect("failed to parse line")
        })
        .collect::<Vec<Vec<_>>>()
}

fn extrapolate(mut nums: Vec<i64>) -> (i64, i64) {
    let mut new_nums = Vec::with_capacity(nums.len());

    let mut backwards = 0;
    let mut forwards = 0;

    // For extrapolating backwards flip the sign
    // of what we add each "turn"
    let mut flip = 1;

    while nums.iter().any(|&x| x != 0) {
        forwards += nums.last().expect("nums should not be empty");
        backwards += flip * nums.first().expect("nums should not be empty");

        nums.iter()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .for_each(|v| new_nums.push(v));

        std::mem::swap(&mut nums, &mut new_nums);
        new_nums.clear();

        flip *= -1;
    }

    (backwards, forwards)
}

fn calculate(data: Vec<Vec<i64>>) -> (i64, i64) {
    data.into_iter()
        .map(extrapolate)
        .fold((0, 0), |acc, elem| (acc.0 + elem.1, acc.1 + elem.0))
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = calculate(data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_09");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_09");

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate(parse(EXAMPLE_DATA)).0, 114);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate(parse(EXAMPLE_DATA)).1, 2);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(parse(REAL_DATA)).0, 1868368343);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(parse(REAL_DATA)).1, 1022);
    }
}
