#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ndarray::Array2;
use std::fs;
use ahash::AHashMap;

fn make_grid(raw_inp: &str) -> Array2<u8> {
    let columns = raw_inp
        .trim()
        .bytes()
        .position(|c| c == b'\n')
        .expect("can't get column count");

    Array2::from_shape_vec(
        ((raw_inp.trim().len() + 1) / (columns + 1), columns),
        raw_inp.bytes().filter(|&x| x != b'\n').collect(),
    )
    .expect("can't make array")
}

fn parse(raw_inp: &str) -> Array2<u8> {
    make_grid(raw_inp.trim())
}

const NORTH: u8 = 0;
const EAST: u8 = 1;
const SOUTH: u8 = 2;
const WEST: u8 = 3;

fn next_location<const DIR: u8>(idx: &(usize, usize)) -> (usize, usize) {
    match DIR {
        NORTH => (idx.0 - 1, idx.1),
        WEST => (idx.0, idx.1 - 1),
        SOUTH => (idx.0 + 1, idx.1),
        EAST => (idx.0, idx.1 + 1),
        _ => panic!("invalid direction"),
    }
}

fn roll<const DIR: u8>(data: &mut Array2<u8>) -> bool {
    let rocks_to_move = data
        .indexed_iter()
        .filter(|(idx, _)| match DIR {
            NORTH => idx.0 != 0,
            WEST => idx.1 != 0,
            SOUTH => idx.0 != data.dim().0 - 1,
            EAST => idx.1 != data.dim().1 - 1,
            _ => panic!("invalid direction"),
        })
        .filter(|(_, &itm)| itm == b'O')
        .filter(|(idx, _)| data[next_location::<DIR>(idx)] == b'.')
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    rocks_to_move.iter().for_each(|idx| {
        data[next_location::<DIR>(idx)] = b'O';
        data[(idx.0, idx.1)] = b'.';
    });

    !rocks_to_move.is_empty()
}

fn calculate_total_load(data: &Array2<u8>) -> usize {
    data.indexed_iter()
        .filter(|(_, &itm)| itm == b'O')
        .map(|(idx, _)| data.dim().0 - idx.0)
        .sum()
}

fn calculate_p1(orig_data: &Array2<u8>) -> usize {
    let mut data = orig_data.clone();

    while roll::<NORTH>(&mut data) {}

    calculate_total_load(&data)
}

fn apply_one_cycle(data: &mut Array2<u8>) {
    while roll::<NORTH>(data) {}
    while roll::<WEST>(data) {}
    while roll::<SOUTH>(data) {}
    while roll::<EAST>(data) {}
}

// Tried hare-and-tortoise algorithm but that's slower
// as the cycle just isn't very long and f() is expensive.
// So just use a map of previous states instead.
//
// Note: data mutated in-place and will be at position:
//      head + cycle_length
// after this function returns.
fn find_cycle(data: &mut Array2<u8>) -> (usize, usize) {
    
    let mut map: AHashMap<Array2<u8>, usize> = AHashMap::with_capacity(128);
    
    let mut curr = 0;

    loop {
        curr += 1;
        apply_one_cycle(data);

        if let Some(&previous) = map.get(&data) {
            return (previous, curr - previous);
        } else {
            map.insert(data.clone(), curr);
        }
    }
}

fn calculate_p2(mut data: Array2<u8>) -> usize {
    let (head, cycle_length) = find_cycle(&mut data);

    let tail = (1000000000 - head) % cycle_length;

    for _ in 0..tail {
        apply_one_cycle(&mut data);
    }

    calculate_total_load(&data)
}

fn main() {
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
