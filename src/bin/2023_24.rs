#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use ndarray::Array2;
use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Hailstone {
    pos_x: i128,
    pos_y: i128,
    pos_z: i128,
    vel_x: i128,
    vel_y: i128,
    vel_z: i128,
}

fn parse(raw_inp: &str) -> Vec<Hailstone> {
    raw_inp.trim().lines()
        .map(|line| {
            let (pos, vel) = line.trim().split_once(" @ ").unwrap();
            
            let (px, pos) = pos.split_once(", ").unwrap();
            let (py, pz) = pos.split_once(", ").unwrap();
            
            let (vx, vel) = vel.split_once(", ").unwrap();
            let (vy, vz) = vel.split_once(", ").unwrap();
            
            Hailstone {
                pos_x: px.trim().parse().unwrap(),
                pos_y: py.trim().parse().unwrap(),
                pos_z: pz.trim().parse().unwrap(),
                vel_x: vx.trim().parse().unwrap(),
                vel_y: vy.trim().parse().unwrap(),
                vel_z: vz.trim().parse().unwrap(),
            }
        })
        .collect()
}

const TEST_MIN: f64 = 200000000000000.0;
const TEST_MAX: f64 = 400000000000000.0;

// const TEST_MIN: f64 = 7.0;
// const TEST_MAX: f64 = 27.0;

fn calculate_crossing_point(h1: &Hailstone, h2: &Hailstone) -> Option<(f64, f64)> {
    let x1 = h1.pos_x;
    let y1 = h1.pos_y;
    
    let x2 = h1.pos_x + h1.vel_x;
    let y2 = h1.pos_y + h1.vel_y;
    
    let x3 = h2.pos_x;
    let y3 = h2.pos_y;
    
    let x4 = h2.pos_x + h2.vel_x;
    let y4 = h2.pos_y + h2.vel_y;
    
    let d1 = (x1 - x2) * (y3 - y4);
    let d2 = (y1 - y2) * (x3 - x4);
    
    let d = d1 - d2;
    
    if d == 0 {
        return None;
    }
    
    let px = (x1 * y2 - y1 * x2)*(x3 - x4) - (x1 - x2) * (x3*y4 - y3*x4); 
    let py = (x1 * y2 - y1 * x2)*(y3 - y4) - (y1 - y2) * (x3*y4 - y3*x4);
    
    let c = ((px as f64) / (d as f64), (py as f64) / (d as f64));
    
    Some(c)
}

fn is_crossing_in_future(hailstone: &Hailstone, crossing: &(f64, f64)) -> bool {
    if hailstone.vel_x > 0 && crossing.0 < hailstone.pos_x as f64 {
        return false;
    } else if hailstone.vel_x < 0 && crossing.0 > hailstone.pos_x as f64 {
        return false;
    } else if hailstone.vel_y > 0 && crossing.1 < hailstone.pos_y as f64 {
        return false;
    } else if hailstone.vel_y < 0 && crossing.1 > hailstone.pos_y as f64 {
        return false;
    }
    true
}

fn calculate_p1(data: &[Hailstone]) -> usize {
    // not 12176
    // not 1804
    data.iter()
        .tuple_combinations()
        .filter_map(|(h1, h2)| {
            if let Some(c) = calculate_crossing_point(h1, h2) {
                Some((h1, h2, c))
            } else {
                None
            }
        })
        .filter(|(h1, h2, c)| is_crossing_in_future(h1, c))
        .filter(|(h1, h2, c)| is_crossing_in_future(h2, c))
        .map(|(_, _, c)| c)
        .filter(|(c1, c2)| c1 >= &TEST_MIN && c1 <= &TEST_MAX && c2 >= &TEST_MIN && c2 <= &TEST_MAX)
        .count()
}

fn calculate_p2(data: &[Hailstone]) -> usize {
    0
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_24");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_24");

//     #[test]
//     fn test_p1_example() {
//         assert_eq!(calculate::<1>(&parse(EXAMPLE_DATA)), 94);
//     }
//     
//     #[test]
//     fn test_p2_example() {
//         assert_eq!(calculate::<2>(&parse(EXAMPLE_DATA)), 154);
//     }
//     
//     #[test]
//     fn test_p1_real() {
//         assert_eq!(calculate::<1>(&parse(REAL_DATA)), 2282);
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
