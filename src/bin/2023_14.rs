#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use ndarray::Array2;
use rayon::prelude::*;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    make_byte_grid(raw_inp.trim())
}

const NORTH: u8 = 0;
const EAST: u8 = 1;
const SOUTH: u8 = 2;
const WEST: u8 = 3;

#[inline(never)]
fn roll<const DIR: u8>(data: &mut Array2<u8>) {
    match DIR {
        NORTH => data.columns_mut(),
        SOUTH => data.columns_mut(),
        WEST => data.rows_mut(),
        EAST => data.rows_mut(),
        _ => panic!("invalid dir"),
    }
    .into_iter()
    .par_bridge()
    .for_each(|mut slice| {
        let mut rocks = slice
            .iter()
            .enumerate()
            .filter(|(_, &itm)| itm == b'O')
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        while let Some(rock_idx) = rocks.pop() {
            if slice.get(rock_idx) != Some(&b'O') {
                continue;
            }

            let (prev_idx, next_idx) = match DIR {
                SOUTH => (rock_idx.wrapping_sub(1), rock_idx.wrapping_add(1)),
                EAST => (rock_idx.wrapping_sub(1), rock_idx.wrapping_add(1)),
                NORTH => (rock_idx.wrapping_add(1), rock_idx.wrapping_sub(1)),
                WEST => (rock_idx.wrapping_add(1), rock_idx.wrapping_sub(1)),
                _ => panic!("invalid dir"),
            };

            if slice.get(next_idx) == Some(&b'.') {
                slice[next_idx] = b'O';
                slice[rock_idx] = b'.';

                rocks.push(next_idx);

                // If we just made space for a different rock to move,
                // add that one to the queue.
                if slice.get(prev_idx) == Some(&b'O') {
                    rocks.push(prev_idx);
                }
            }
        }
    });
}

fn calculate_total_load(data: &Array2<u8>) -> usize {
    data.indexed_iter()
        .filter(|(_, &itm)| itm == b'O')
        .map(|(idx, _)| data.dim().0 - idx.0)
        .sum()
}

fn calculate_p1(orig_data: &Array2<u8>) -> usize {
    let mut data = orig_data.clone();

    roll::<NORTH>(&mut data);

    calculate_total_load(&data)
}

fn apply_one_cycle(data: &mut Array2<u8>) {
    roll::<NORTH>(data);
    roll::<WEST>(data);
    roll::<SOUTH>(data);
    roll::<EAST>(data);
}

// Tried hare-and-tortoise algorithm but that's slower
// as the cycle just isn't very long and f() is expensive.
// So just use a map of all previous states instead (!).
fn calculate_p2(mut data: Array2<u8>) -> usize {
    let mut map: AHashMap<Array2<u8>, usize> = AHashMap::with_capacity(128);

    let mut curr = 0;

    let (head, cycle_length) = loop {
        curr += 1;
        apply_one_cycle(&mut data);

        if let Some(&previous) = map.get(&data) {
            break (previous, curr - previous);
        } else {
            map.insert(data.clone(), curr);
        }
    };

    let tail = (1000000000 - head) % cycle_length;

    let final_grid = map
        .iter()
        .filter(|(_, &v)| v == head + tail)
        .map(|(k, _)| k)
        .next()
        .expect("no valid answer?");

    calculate_total_load(final_grid)
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1(&data);
    let p2 = calculate_p2(data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_14");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_14");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 136);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(parse(EXAMPLE_DATA)), 64);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 112048);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(parse(REAL_DATA)), 105606);
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
                let p1 = calculate_p1(&data);
                let p2 = calculate_p2(data);
                (p1, p2)
            });
        }
    }
}