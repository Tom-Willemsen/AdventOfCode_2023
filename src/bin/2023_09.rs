use advent_of_code_2023::{Cli, Parser};
use itertools::Itertools;
use std::fs;

fn parse(raw_inp: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    raw_inp.trim().lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()
            .expect("failed to parse line")
    })
}

fn extrapolate(mut nums: Vec<i64>) -> (i64, i64) {
    // Use a buffer-swapping implementation so we never have more
    // than 2 arrays (current + next) in memory.
    let mut new_nums = Vec::with_capacity(nums.len() - 1);

    // Running counters for what the final result will be for backwards
    // and forwards interpolation
    let mut backwards = 0;
    let mut forwards = 0;

    // For extrapolating backwards we need to flip the sign of what we
    // add each "turn" to account for signs. This allows us to compute
    // forwards & backwards in the same loop, minimising total allocations.
    let mut flip = 1;

    while nums.iter().any(|&x| x != 0) {
        forwards += nums.last().expect("nums should not be empty");
        backwards += flip * nums.first().expect("nums should not be empty");

        new_nums.clear();

        new_nums.extend(nums.iter().tuple_windows().map(|(x, y)| y - x));

        // Buffer-swapping, always keeping just `nums` and `new_nums` as
        // the only two large allocations.
        std::mem::swap(&mut nums, &mut new_nums);

        // Swap the sign for p2.
        flip *= -1;
    }

    (backwards, forwards)
}

fn calculate(data: impl Iterator<Item = Vec<i64>>) -> (i64, i64) {
    data.map(extrapolate)
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
