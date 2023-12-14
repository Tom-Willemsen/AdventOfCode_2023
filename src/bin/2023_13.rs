#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_bool_grid;
use advent_of_code_2023::{Cli, Parser};
use ndarray::{s, Array2};
use std::fs;

fn parse(raw_inp: &str) -> Vec<Array2<bool>> {
    raw_inp
        .trim()
        .split("\n\n")
        .map(make_bool_grid::<b'#'>)
        .collect()
}

fn get_y_reflection<const WANTED_DIFF: usize>(arr: &Array2<bool>) -> Option<usize> {
    (0..arr.dim().0 - 1)
        .filter(|&start_y| {
            (0..=start_y)
                .rev()
                .zip(start_y + 1..arr.dim().0)
                .map(|(r1, r2)| {
                    arr.slice(s![r1, ..])
                        .iter()
                        .zip(arr.slice(s![r2, ..]))
                        .map(|(e1, e2)| if e1 != e2 { 1 } else { 0 })
                        .sum::<usize>()
                })
                .sum::<usize>()
                == WANTED_DIFF
        })
        .map(|start_y| start_y + 1)
        .next()
}

fn get_x_reflection<const WANTED_DIFF: usize>(arr: &Array2<bool>) -> Option<usize> {
    (0..arr.dim().1 - 1)
        .filter(|&start_x| {
            (0..=start_x)
                .rev()
                .zip(start_x + 1..arr.dim().1)
                .map(|(c1, c2)| {
                    arr.slice(s![.., c1])
                        .iter()
                        .zip(arr.slice(s![.., c2]))
                        .map(|(e1, e2)| if e1 != e2 { 1 } else { 0 })
                        .sum::<usize>()
                })
                .sum::<usize>()
                == WANTED_DIFF
        })
        .map(|start_x| start_x + 1)
        .next()
}

fn calculate<const WANTED_DIFF: usize>(data: &[Array2<bool>]) -> usize {
    data.iter()
        .map(|arr| {
            if let Some(res) = get_y_reflection::<WANTED_DIFF>(arr) {
                res * 100
            } else if let Some(res) = get_x_reflection::<WANTED_DIFF>(arr) {
                res
            } else {
                panic!("no answer");
            }
        })
        .sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate::<0>(&data);
    let p2 = calculate::<1>(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_13");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_13");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate::<0>(&parse(EXAMPLE_DATA)), 405);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate::<1>(&parse(EXAMPLE_DATA)), 400);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<0>(&parse(REAL_DATA)), 32723);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<1>(&parse(REAL_DATA)), 34536);
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| {
                let data = parse(black_box(REAL_DATA));
                let p1 = calculate::<0>(&data);
                let p2 = calculate::<1>(&data);
                (p1, p2)
            });
        }
    }
}
