#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use num::Integer;
use std::fs;

struct Data {
    directions: Vec<u8>,
    location_map: LocationMap,
    p2_starts: Vec<Location>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Location {
    loc: u16,
}

impl From<&[u8; 3]> for Location {
    fn from(item: &[u8; 3]) -> Self {
        debug_assert!(item[0] >= b'A' && item[0] <= b'Z', "invalid pos {:?}", item);
        debug_assert!(item[1] >= b'A' && item[1] <= b'Z', "invalid pos {:?}", item);
        debug_assert!(item[2] >= b'A' && item[2] <= b'Z', "invalid pos {:?}", item);

        // Bit-twiddling hackery for performance: Pack location into a u16
        Location {
            loc: ((item[0] - b'A') as u16 * 26 * 26)
                + ((item[1] - b'A') as u16 * 26)
                + (item[2] - b'A') as u16,
        }
    }
}

impl Location {
    fn ends_with<const END: u8>(&self) -> bool {
        (self.loc % 26) + b'A' as u16 == END as u16
    }

    fn as_arr_index(&self) -> usize {
        self.loc.into()
    }
}

struct LocationMap {
    map: [(Location, Location); 26 * 26 * 26],
}

impl LocationMap {
    fn new() -> LocationMap {
        // Make everything initially map back to "AAA".
        LocationMap {
            map: [(b"AAA".into(), b"AAA".into()); 26 * 26 * 26],
        }
    }

    fn set(&mut self, src: Location, left: Location, right: Location) {
        self.map[src.as_arr_index()] = (left, right);
    }

    fn next(&self, current: Location, dir: &u8) -> Location {
        debug_assert!(dir == &b'L' || dir == &b'R');

        let next = self.map[current.as_arr_index()];
        if dir == &b'L' {
            next.0
        } else {
            next.1
        }
    }
}

fn parse(raw_inp: &[u8]) -> Data {
    let directions = raw_inp
        .split(|&elem| elem == b'\n')
        .next()
        .map(|line| line.into())
        .expect("can't parse directions");

    let mut location_map = LocationMap::new();

    let mut p2_starts = vec![];

    raw_inp
        .split(|&elem| elem == b'\n')
        .skip(2) // Directions + blank line
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let src: &[u8; 3] = line[0..3].try_into().unwrap();
            let left: &[u8; 3] = line[7..10].try_into().unwrap();
            let right: &[u8; 3] = line[12..15].try_into().unwrap();

            let src: Location = src.into();

            location_map.set(src, left.into(), right.into());

            if src.ends_with::<b'A'>() {
                p2_starts.push(src);
            }
        });

    Data {
        directions,
        location_map,
        p2_starts,
    }
}

fn search(data: &Data, start: Location, cond: fn(Location) -> bool) -> u64 {
    let mut result = 0;

    let mut loc = start;

    for dir in data.directions.iter().cycle() {
        result += 1;

        loc = data.location_map.next(loc, dir);

        if cond(loc) {
            return result;
        }
    }
    panic!("no directions");
}

fn calculate_p1(data: &Data) -> u64 {
    search(data, b"AAA".into(), |loc| loc == b"ZZZ".into())
}

// Assumptions:
// - Cycle length Z -> Z is same length as initial path A -> Z
// - Cycle xxZ -> xxZ does not pass through *any* other node ending in Z
// These assumptions seem to be true for my input.
fn calculate_p2(data: &Data) -> u64 {
    data.p2_starts
        .iter()
        .map(|&k| search(data, k, |loc| loc.ends_with::<b'Z'>()))
        .fold(1, |acc, elem| acc.lcm(&elem))
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_P1_1: &[u8] = include_bytes!("../../inputs/examples/2023_08_p1_ex1");
    const EXAMPLE_DATA_P1_2: &[u8] = include_bytes!("../../inputs/examples/2023_08_p1_ex2");
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
