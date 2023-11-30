use advent_of_code_2023::{Cli, Parser};
use std::cmp::max;
use std::fs;

fn parse(raw_inp: &str) -> Vec<i64> {
    raw_inp.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn calculate_p1(nums: &[i64]) -> i64 {
    0
}

fn calculate_p2(nums: &[i64]) -> i64 {
    0
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

    // const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_01");
    // const REAL_DATA: &str = include_str!("../../inputs/real/2023_01");
    // 
    // #[test]
    // fn test_p1_example() {
    //     assert_eq!(calculate_p1(&parse(&EXAMPLE_DATA)), 0);
    // }
    // 
    // #[test]
    // fn test_p1_real() {
    //     assert_eq!(calculate_p1(&parse(&REAL_DATA)), 0);
    // }
    // 
    // #[test]
    // fn test_p2_example() {
    //     assert_eq!(calculate_p2(&parse(&EXAMPLE_DATA)), 0);
    // }
    // 
    // #[test]
    // fn test_p2_real() {
    //     assert_eq!(calculate_p2(&parse(&REAL_DATA)), 0);
    // }
}
