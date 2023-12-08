use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use num::Integer;
use std::fs;

struct Data<'a> {
    directions: Vec<u8>,
    map: AHashMap<&'a str, (&'a str, &'a str)>,
}

fn parse(raw_inp: &str) -> Data {
    let directions = raw_inp
        .trim()
        .lines()
        .next()
        .map(|line| line.trim().bytes().collect::<Vec<_>>())
        .expect("can't parse directions");

    let map = raw_inp
        .trim()
        .lines()
        .skip(2) // Directions + blank line
        .map(|line| {
            let (src, rest) = line.split_once(" = ").expect("failed dir split");

            let (left, right) = rest.split_once(", ").expect("failed l/r split");
            let left = left.strip_prefix('(').expect("bad line format?");
            let right = right.strip_suffix(')').expect("bad line format?");

            (src, (left, right))
        })
        .collect::<AHashMap<_, _>>();

    Data { directions, map }
}

fn search(data: &Data, start: &str, cond: fn(&str) -> bool) -> u64 {
    let mut result = 0;

    let mut loc = start;

    loop {
        for dir in &data.directions {
            result += 1;

            let next = data.map.get(loc).expect("invalid location");

            if dir == &b'L' {
                loc = next.0;
            } else {
                loc = next.1;
            }

            if cond(loc) {
                return result;
            }
        }
    }
}

fn calculate_p1(data: &Data) -> u64 {
    search(data, "AAA", |loc| loc == "ZZZ")
}

fn calculate_p2(data: &Data) -> u64 {
    data.map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| search(data, k, |loc| loc.ends_with('Z')))
        .reduce(|acc, elem| acc.lcm(&elem))
        .expect("Expected at least one loc ending in A")
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

    const EXAMPLE_DATA_P1_1: &str = include_str!("../../inputs/examples/2023_08_p1_ex1");
    const EXAMPLE_DATA_P1_2: &str = include_str!("../../inputs/examples/2023_08_p1_ex2");
    const EXAMPLE_DATA_P2: &str = include_str!("../../inputs/examples/2023_08_p2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_08");

    #[test]
    fn test_p1_example_1() {
        assert_eq!(calculate_p1(&mut parse(EXAMPLE_DATA_P1_1)), 2);
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&mut parse(EXAMPLE_DATA_P1_2)), 6);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&mut parse(EXAMPLE_DATA_P2)), 6);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&mut parse(REAL_DATA)), 12169);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&mut parse(REAL_DATA)), 12030780859469);
    }
}
