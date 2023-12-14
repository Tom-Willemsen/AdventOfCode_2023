#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashSet;
use ndarray::Array2;
use std::fs;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct PointDiff {
    pub x: isize,
    pub y: isize,
}

impl Point {
    fn apply_diff(&self, diff: &PointDiff) -> Option<Point> {
        Some(Point {
            x: self.x.checked_add_signed(diff.x)?,
            y: self.y.checked_add_signed(diff.y)?,
        })
    }
}

fn parse(raw_inp: &str) -> Array2<u8> {
    make_byte_grid(raw_inp)
}

fn get_rules_for(itm: u8) -> Vec<PointDiff> {
    match itm {
        b'|' => vec![PointDiff { y: -1, x: 0 }, PointDiff { y: 1, x: 0 }],
        b'-' => vec![PointDiff { y: 0, x: 1 }, PointDiff { y: 0, x: -1 }],
        b'L' => vec![PointDiff { y: -1, x: 0 }, PointDiff { y: 0, x: 1 }],
        b'J' => vec![PointDiff { y: -1, x: 0 }, PointDiff { y: 0, x: -1 }],
        b'7' => vec![PointDiff { y: 1, x: 0 }, PointDiff { y: 0, x: -1 }],
        b'F' => vec![PointDiff { y: 1, x: 0 }, PointDiff { y: 0, x: 1 }],
        _ => vec![],
    }
}

const LEFT: PointDiff = PointDiff { y: 0, x: -1 };
const RIGHT: PointDiff = PointDiff { y: 0, x: 1 };
const UP: PointDiff = PointDiff { y: -1, x: 0 };
const DOWN: PointDiff = PointDiff { y: 1, x: 0 };

const CONNECT_LEFT: [u8; 3] = [b'F', b'L', b'-'];
const CONNECT_RIGHT: [u8; 3] = [b'J', b'7', b'-'];
const CONNECT_UP: [u8; 3] = [b'F', b'7', b'|'];
const CONNECT_DOWN: [u8; 3] = [b'J', b'L', b'|'];

fn replace_start(data: &mut Array2<u8>, start: &Point) {
    let cons = [UP, DOWN, LEFT, RIGHT]
        .iter()
        .map(|diff| start.apply_diff(diff))
        .map(|pt| pt.and_then(|p| data.get((p.y, p.x))))
        .zip([CONNECT_UP, CONNECT_DOWN, CONNECT_LEFT, CONNECT_RIGHT])
        .map(|(pt, connections)| {
            if let Some(pt) = pt {
                connections.contains(pt)
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    let new = match (cons[0], cons[1], cons[2], cons[3]) {
        (true, true, false, false) => b'|',
        (true, false, true, false) => b'J',
        (true, false, false, true) => b'L',
        (false, false, true, true) => b'-',
        (false, true, true, false) => b'7',
        (false, true, false, true) => b'F',
        _ => panic!("can't find connections for start"),
    };

    data[(start.y, start.x)] = new;
}

fn get_main_loop(data: &mut Array2<u8>) -> AHashSet<Point> {
    let start_location = data
        .indexed_iter()
        .filter(|(_, &d)| d == b'S')
        .map(|(idx, _)| idx)
        .next()
        .expect("can't find start");

    let mut visited = AHashSet::new();

    let mut next = Point {
        y: start_location.0,
        x: start_location.1,
    };

    replace_start(data, &next);

    loop {
        visited.insert(next);
        let n = get_rules_for(data[(next.y, next.x)])
            .into_iter()
            .filter_map(|diff| next.apply_diff(&diff))
            .find(|p| !visited.contains(p));

        if let Some(n) = n {
            next = n;
        } else {
            break;
        }
    }

    visited
}

fn calculate_inside(data: Array2<u8>, main_loop: &AHashSet<Point>) -> usize {
    let mut result = 0;
    for y in 0..data.dim().0 {
        let mut inside = false;

        for x in 0..data.dim().1 {
            let cell = data[(y, x)];

            let in_main_loop = main_loop.contains(&Point { y, x });

            if in_main_loop && (cell == b'|' || cell == b'J' || cell == b'L') {
                inside = !inside
            }

            if !in_main_loop && inside {
                result += 1;
            }
        }
    }
    result
}

fn calculate(mut data: Array2<u8>) -> (usize, usize) {
    let main_loop = get_main_loop(&mut data);

    let p1 = main_loop.len() / 2;

    let p2 = calculate_inside(data, &main_loop);

    (p1, p2)
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

    const EXAMPLE_DATA_P1: &str = include_str!("../../inputs/examples/2023_10_p1");
    const EXAMPLE_DATA_P2_1: &str = include_str!("../../inputs/examples/2023_10_p2_1");
    const EXAMPLE_DATA_P2_2: &str = include_str!("../../inputs/examples/2023_10_p2_2");
    const EXAMPLE_DATA_P2_3: &str = include_str!("../../inputs/examples/2023_10_p2_3");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_10");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate(parse(EXAMPLE_DATA_P1)).0, 8);
    }

    #[test]
    fn test_p2_example_1() {
        assert_eq!(calculate(parse(EXAMPLE_DATA_P2_1)).1, 4);
    }

    #[test]
    fn test_p2_example_2() {
        assert_eq!(calculate(parse(EXAMPLE_DATA_P2_2)).1, 8);
    }

    #[test]
    fn test_p2_example_3() {
        assert_eq!(calculate(parse(EXAMPLE_DATA_P2_3)).1, 10);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(parse(REAL_DATA)).0, 6831);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(parse(REAL_DATA)).1, 305);
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
        fn bench_parse_and_calculate(b: &mut Bencher) {
            b.iter(|| calculate(parse(black_box(REAL_DATA))));
        }
    }
}
