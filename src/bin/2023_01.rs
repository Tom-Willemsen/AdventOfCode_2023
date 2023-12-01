use advent_of_code_2023::{Cli, Parser};
use std::fs;

fn map_to_digit<const PART: u8>(line: &str) -> Option<u32> {
    if let Some(result) = line.chars().next().and_then(|c| c.to_digit(10)) {
        return Some(result);
    } else if PART == 2 {
        if line.starts_with("one") {
            return Some(1);
        } else if line.starts_with("two") {
            return Some(2);
        } else if line.starts_with("three") {
            return Some(3);
        } else if line.starts_with("four") {
            return Some(4);
        } else if line.starts_with("five") {
            return Some(5);
        } else if line.starts_with("six") {
            return Some(6);
        } else if line.starts_with("seven") {
            return Some(7);
        } else if line.starts_with("eight") {
            return Some(8);
        } else if line.starts_with("nine") {
            return Some(9);
        }
    }
    None
}

fn get_first_digit<const PART: u8>(line: &str) -> u32 {
    (0..line.len())
        .map(|skip| &line[skip..])
        .find_map(map_to_digit::<PART>)
        .expect("no matching first digit")
}

fn get_last_digit<const PART: u8>(line: &str) -> u32 {
    (0..line.len())
        .rev()
        .map(|skip| &line[skip..])
        .find_map(map_to_digit::<PART>)
        .expect("no matching last digit")
}

fn calculate<const PART: u8>(raw_inp: &str) -> u32 {
    raw_inp
        .lines()
        .map(|line| {
            let first = get_first_digit::<PART>(line);
            let last = get_last_digit::<PART>(line);

            10 * first + last
        })
        .sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let p1 = calculate::<1>(&inp);
    let p2 = calculate::<2>(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_P1: &str = include_str!("../../inputs/examples/2023_01_p1");
    const EXAMPLE_DATA_P2: &str = include_str!("../../inputs/examples/2023_01_p2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_01");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate::<1>(&EXAMPLE_DATA_P1), 142);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&REAL_DATA), 56506);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate::<2>(&EXAMPLE_DATA_P2), 281);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2>(&REAL_DATA), 56017);
    }
}
