#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use itertools::Itertools;
use ndarray::Array2;
use std::fs;

#[derive(Eq, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn diff(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

struct Data {
    galaxies: Vec<Point>,
    extra_cols: Vec<usize>,
    extra_rows: Vec<usize>,
}

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

fn parse(raw_inp: &str) -> Data {
    let mut galaxies = vec![];
    let mut extra_rows = vec![];
    let mut extra_cols = vec![];

    let grid = make_grid(raw_inp);

    for y in 0..grid.dim().0 {
        let mut contains_any_galaxy = false;
        for x in 0..grid.dim().1 {
            if grid[(y, x)] {
                contains_any_galaxy = true;
                galaxies.push(Point { y, x });
            }
        }

        if !contains_any_galaxy {
            extra_rows.push(y);
        }
    }

    for x in 0..grid.dim().1 {
        let mut contains_any_galaxy = false;
        for y in 0..grid.dim().0 {
            if grid[(y, x)] {
                contains_any_galaxy = true;
                break;
            }
        }
        if !contains_any_galaxy {
            extra_cols.push(x);
        }
    }

    Data {
        galaxies,
        extra_cols,
        extra_rows,
    }
}

fn calculate<const P1_MUL: usize, const P2_MUL: usize>(data: &Data) -> (usize, usize) {
    data.galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| {
            let raw_dist = g1.diff(g2);

            let extra_rows = data
                .extra_rows
                .iter()
                .filter(|&&e| e > g1.y.min(g2.y) && e < g1.y.max(g2.y))
                .count();

            let extra_cols = data
                .extra_cols
                .iter()
                .filter(|&&e| e > g1.x.min(g2.x) && e < g1.x.max(g2.x))
                .count();

            (
                raw_dist + (extra_rows + extra_cols) * (P1_MUL - 1),
                raw_dist + (extra_rows + extra_cols) * (P2_MUL - 1),
            )
        })
        .fold((0, 0), |acc, elem| (acc.0 + elem.0, acc.1 + elem.1))
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = calculate::<2, 1000000>(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_11");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_11");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate::<2, 2>(&parse(EXAMPLE_DATA)).0, 374);
    }

    #[test]
    fn test_p2_examples() {
        assert_eq!(calculate::<10, 100>(&parse(EXAMPLE_DATA)), (1030, 8410));
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<2, 1000000>(&parse(REAL_DATA)).0, 10490062);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2, 1000000>(&parse(REAL_DATA)).1, 382979724122);
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| calculate::<2, 1000000>(&parse(black_box(REAL_DATA))));
        }
    }
}
