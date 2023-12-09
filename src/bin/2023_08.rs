#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use num::Integer;
use rayon::prelude::*;
use std::fs;

struct Data<'a> {
    directions: Vec<u8>,
    map: AHashMap<&'a [u8; 3], (&'a [u8; 3], &'a [u8; 3])>,
}

fn parse(raw_inp: &[u8]) -> Data {
    let directions = raw_inp
        .split(|&elem| elem == b'\n')
        .next()
        .map(|line| line.into())
        .expect("can't parse directions");

    let map = raw_inp
        .split(|&elem| elem == b'\n')
        .skip(2) // Directions + blank line
        .filter(|line| !line.is_empty())
        .map(|line| {
            let src = line[0..3].try_into().unwrap();
            let left = line[7..10].try_into().unwrap();
            let right = line[12..15].try_into().unwrap();

            (src, (left, right))
        })
        .collect::<AHashMap<_, _>>();

    Data { directions, map }
}

fn search(data: &Data, start: &[u8; 3], cond: fn(&[u8; 3]) -> bool) -> u64 {
    let mut result = 0;

    let mut loc = start;

    for dir in data.directions.iter().cycle() {
        result += 1;

        let next = data.map.get(loc).expect("invalid location");

        loc = if dir == &b'L' { next.0 } else { next.1 };

        if cond(loc) {
            return result;
        }
    }
    panic!("no directions");
}

fn calculate_p1(data: &Data) -> u64 {
    search(data, b"AAA", |loc| loc == b"ZZZ")
}

// Assumptions:
// - Cycle length Z -> Z is same length as initial path A -> Z
// - Cycle xxZ -> xxZ does not pass through *any* other node ending in Z
// These assumptions seem to be true for my input.
fn calculate_p2(data: &Data) -> u64 {
    data.map
        .keys()
        .filter(|k| k[2] == b'Z')
        .par_bridge()
        .map(|k| search(data, k, |loc| loc[2] == b'Z'))
        .reduce(|| 1, |acc, elem| acc.lcm(&elem))
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .stack_size(64 * 1024) // 64k ought to be enough for anyone
        .num_threads(8)
        .build_global()
        .unwrap();

    let args = Cli::parse();

    let inp = fs::read(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = rayon::join(|| calculate_p1(&data), || calculate_p2(&data));
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_P1_1: &[u8] = include_bytes!("../../inputs/examples/2023_08_p1_ex1");
    const EXAMPLE_DATA_P1_2: &[u8] = include_bytes!("../../inputs/examples/2023_08_p1_ex2");
    const EXAMPLE_DATA_P2: &[u8] = include_bytes!("../../inputs/examples/2023_08_p2");
    const REAL_DATA: &[u8] = include_bytes!("../../inputs/real/2023_08");

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
        fn bench_p1_with_parse(b: &mut Bencher) {
            b.iter(|| calculate_p1(black_box(&mut parse(REAL_DATA))));
        }

        #[bench]
        fn bench_p2_with_parse(b: &mut Bencher) {
            b.iter(|| calculate_p2(black_box(&mut parse(REAL_DATA))));
        }
    }
}
