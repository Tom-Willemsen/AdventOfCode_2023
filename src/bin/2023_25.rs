#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use ndarray::Array2;
use std::fs;

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let p1 = 0;
    let p2 = 0;
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_25");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_25");

    //     #[test]
    //     fn test_p1_example() {
    //         assert_eq!(calculate_p1::<7, 27>(&parse(EXAMPLE_DATA)), 2);
    //     }
    //
    //     #[test]
    //     fn test_p2_example() {
    //         assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 47);
    //     }
    //
    //     #[test]
    //     fn test_p1_real() {
    //         assert_eq!(calculate_p1::<P1_REAL_MIN, P1_REAL_MAX>(&parse(REAL_DATA)), 17906);
    //     }
    //
    //     #[test]
    //     fn test_p2_real() {
    //         assert_eq!(calculate::<2>(&parse(REAL_DATA)), 6646);
    //     }
    //
    //     #[cfg(feature = "bench")]
    //     mod benches {
    //         extern crate test;
    //         use test::{black_box, Bencher};
    //
    //         use super::*;
    //
    //         #[bench]
    //         fn bench(b: &mut Bencher) {
    //             b.iter(|| {
    //                 let data = parse(black_box(REAL_DATA));
    //                 let p1 = calculate_p1(&data);
    //                 let p2 = calculate_p2(&data);
    //                 (p1, p2)
    //             });
    //         }
    //     }
}
