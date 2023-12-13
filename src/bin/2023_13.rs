#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ndarray::{s, Array2};
use rayon::prelude::*;
use std::fs;

fn make_grid(raw_inp: &str) -> Array2<bool> {
    let columns = raw_inp
        .trim()
        .bytes()
        .position(|c| c == b'\n')
        .expect("can't get column count");

    Array2::from_shape_vec(
        ((raw_inp.trim().len() + 1) / (columns + 1), columns),
        raw_inp
            .bytes()
            .filter(|&x| x != b'\n')
            .map(|b| b == b'#')
            .collect(),
    )
    .expect("can't make array")
}

fn parse(raw_inp: &str) -> Vec<Array2<bool>> {
    raw_inp.trim().split("\n\n").map(make_grid).collect()
}

fn get_y_reflection(arr: &Array2<bool>, disallowed: Option<usize>) -> Option<usize> {
    (0..arr.dim().0 - 1)
        .filter(|start_y| Some(start_y + 1) != disallowed)
        .filter(|start_y| {
            (0..=*start_y)
                .rev()
                .zip(start_y + 1..arr.dim().0)
                .all(|(r1, r2)| arr.slice(s![r1, ..]) == arr.slice(s![r2, ..]))
        })
        .map(|start_y| start_y + 1)
        .next()
}

fn get_x_reflection(arr: &Array2<bool>, disallowed: Option<usize>) -> Option<usize> {
    (0..arr.dim().1 - 1)
        .filter(|start_x| Some(start_x + 1) != disallowed)
        .filter(|start_x| {
            (0..=*start_x)
                .rev()
                .zip(start_x + 1..arr.dim().1)
                .all(|(c1, c2)| arr.slice(s![.., c1]) == arr.slice(s![.., c2]))
        })
        .map(|start_x| start_x + 1)
        .next()
}

fn get_p2(arr: &mut Array2<bool>, p1_y: Option<usize>, p1_x: Option<usize>) -> usize {
    for y in 0..arr.dim().0 {
        for x in 0..arr.dim().1 {
            arr[(y, x)] = !arr[(y, x)];

            let y_refl = get_y_reflection(arr, p1_y);
            let x_refl = get_x_reflection(arr, p1_x);

            arr[(y, x)] = !arr[(y, x)];

            if let Some(res) = y_refl {
                return 100 * res;
            }

            if let Some(res) = x_refl {
                return res;
            }
        }
    }
    panic!("can't locate smudge");
}

fn calculate(data: &mut [Array2<bool>]) -> (usize, usize) {
    data.par_iter_mut()
        .map(|arr| {
            let p1_y_refl = get_y_reflection(arr, None);
            let p1_x_refl = get_x_reflection(arr, None);

            let p1 = if let Some(res) = p1_y_refl {
                res * 100
            } else if let Some(res) = p1_x_refl {
                res
            } else {
                panic!("no p1 answer");
            };

            (p1, get_p2(arr, p1_y_refl, p1_x_refl))
        })
        .reduce(|| (0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1))
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let mut data = parse(&inp);
    let (p1, p2) = calculate(&mut data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_13");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_13");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate(&mut parse(EXAMPLE_DATA)).0, 405);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate(&mut parse(EXAMPLE_DATA)).1, 400);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&mut parse(REAL_DATA)).0, 32723);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&mut parse(REAL_DATA)).1, 34536);
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| {
                let mut data = parse(black_box(REAL_DATA));
                calculate(&mut data);
            });
        }
    }
}
