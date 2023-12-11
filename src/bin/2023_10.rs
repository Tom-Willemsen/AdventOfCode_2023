#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ndarray::Array2;
use std::fs;
use ahash::AHashSet;

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
        Some(Point { x: self.x.checked_add_signed(diff.x)?, y: self.y.checked_add_signed(diff.y)? })
    }
}

fn parse(raw_inp: &str) -> Array2<u8> {
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
            .collect(),
    )
    .expect("can't make array")
}

fn get_rules_for(itm: u8) -> Vec<PointDiff> {
    if itm == b'|' {
        vec![
            PointDiff { y: -1, x: 0 },
            PointDiff { y: 1, x: 0 },
        ]
    } else if itm == b'-' {
        vec![
            PointDiff { y: 0, x: 1 },
            PointDiff { y: 0, x: -1 },
        ]
    } else if itm == b'L' {
        vec![
            PointDiff { y: -1, x: 0 },
            PointDiff { y: 0, x: 1 },
        ]
    } else if itm == b'J' {
        vec![
            PointDiff { y: -1, x: 0 },
            PointDiff { y: 0, x: -1 },
        ]
    } else if itm == b'7' {
        vec![
            PointDiff { y: 1, x: 0 },
            PointDiff { y: 0, x: -1 },
        ]
    } else if itm == b'F' {
        vec![
            PointDiff { y: 1, x: 0 },
            PointDiff { y: 0, x: 1 },
        ]
    } else if itm == b'S' {
        vec![
            // TODO HACK FIXME
            // PointDiff { y: 1, x: 0 },
            PointDiff { y: 0, x: 1 },
            // PointDiff { y: -1, x: 0 },
            // PointDiff { y: 0, x: -1 },
        ]
    } else {
        vec![]
    }
}

fn get_main_loop(data: &Array2<u8>) -> AHashSet<Point> {
    let start_location = data
        .indexed_iter()
        .filter(|(_, &d)| d == b'S')
        .map(|(idx, _)| idx)
        .next()
        .expect("can't find start");
        
    let mut visited = AHashSet::new();
    
    let mut next = Point { y: start_location.0, x: start_location.1 };
    
    loop {
        visited.insert(next.clone());
        let n = get_rules_for(data[(next.y, next.x)])
            .into_iter()
            .filter_map(|diff| next.apply_diff(&diff))
            .filter(|p| !visited.contains(p))
            .next();
            
        if let Some(n) = n {
            next = n;
        } else {
            break;
        }
    }
    
    visited
}

fn calculate_inside(data: &Array2<u8>, main_loop: &AHashSet<Point>) -> usize {
    let mut result = 0;
    for y in 0..data.dim().0 {
        let mut inside = false;
        
        for x in 0..data.dim().1 {
            let cell = data[(y, x)];
            
            let in_main_loop = main_loop.contains(&Point { y, x });
            
            // TODO: handle "S" properly
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

fn calculate(data: &Array2<u8>) -> (usize, usize) {
    let main_loop = get_main_loop(data);
    
    let p1 = main_loop.len() / 2;
    
    let p2 = calculate_inside(data, &main_loop);
    
    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = calculate(&data);
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
        assert_eq!(calculate(&parse(EXAMPLE_DATA_P1)).0, 8);
    }

    #[test]
    fn test_p2_example_1() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA_P2_1)).1, 4);
    }

    #[test]
    fn test_p2_example_2() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA_P2_2)).1, 8);
    }

    #[test]
    fn test_p2_example_3() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA_P2_3)).1, 10);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).0, 6831);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).1, 305);
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
        fn bench_calculate(b: &mut Bencher) {
            let parsed = parse(black_box(REAL_DATA));
            b.iter(|| calculate(black_box(&parsed)));
        }
    }
}
