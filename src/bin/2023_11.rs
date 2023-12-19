#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_bool_grid;
use advent_of_code_2023::{Cli, Parser};
use itertools::Itertools;
use std::fs;

#[derive(Eq, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

struct Data {
    galaxies: Vec<Point>,
    extra_cols: Vec<usize>,
    extra_rows: Vec<usize>,
    rows: usize,
    cols: usize,
}

fn parse(raw_inp: &str) -> Data {
    let grid = make_bool_grid::<b'#'>(raw_inp);

    let galaxies = grid
        .indexed_iter()
        .filter(|(_, &v)| v)
        .map(|(idx, _)| Point { y: idx.0, x: idx.1 })
        .collect::<Vec<_>>();

    let extra_rows = (0..grid.dim().0)
        .filter(|&y| galaxies.iter().all(|g| g.y != y))
        .collect::<Vec<_>>();

    let extra_cols = (0..grid.dim().1)
        .filter(|&x| galaxies.iter().all(|g| g.x != x))
        .collect::<Vec<_>>();

    Data {
        galaxies,
        extra_cols,
        extra_rows,
        rows: grid.dim().0,
        cols: grid.dim().1,
    }
}

fn make_cumulative_sum<const P1_MUL: usize, const P2_MUL: usize, F>(
    n: usize,
    add_many_rows: F,
) -> (Vec<usize>, Vec<usize>)
where
    F: Fn(usize) -> bool,
{
    let mut csum_p1 = Vec::with_capacity(n);
    let mut csum_p2 = Vec::with_capacity(n);

    for i in 0..n {
        let add_many = add_many_rows(i);
        csum_p1.push(csum_p1.last().unwrap_or(&0) + if add_many { P1_MUL } else { 1 });
        csum_p2.push(csum_p2.last().unwrap_or(&0) + if add_many { P2_MUL } else { 1 });
    }

    (csum_p1, csum_p2)
}

fn calculate<const P1_MUL: usize, const P2_MUL: usize>(data: &Data) -> (usize, usize) {
    let (p1_rows_csum, p2_rows_csum) = make_cumulative_sum::<P1_MUL, P2_MUL, _>(data.rows, |y| {
        data.extra_rows.iter().any(|&ey| ey == y)
    });
    let (p1_cols_csum, p2_cols_csum) = make_cumulative_sum::<P1_MUL, P2_MUL, _>(data.cols, |x| {
        data.extra_cols.iter().any(|&ex| ex == x)
    });

    data.galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| {
            let p1_rows_cost = p1_rows_csum[g1.y.max(g2.y)] - p1_rows_csum[g1.y.min(g2.y)];
            let p1_cols_cost = p1_cols_csum[g1.x.max(g2.x)] - p1_cols_csum[g1.x.min(g2.x)];

            let p2_rows_cost = p2_rows_csum[g1.y.max(g2.y)] - p2_rows_csum[g1.y.min(g2.y)];
            let p2_cols_cost = p2_cols_csum[g1.x.max(g2.x)] - p2_cols_csum[g1.x.min(g2.x)];

            ((p1_rows_cost + p1_cols_cost), (p2_rows_cost + p2_cols_cost))
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
