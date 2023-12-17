#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use ndarray::Array2;
use std::collections::BinaryHeap;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    let mut arr = make_byte_grid(raw_inp.trim());
    arr.mapv_inplace(|x| x - b'0');
    arr
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// Dijkstra
fn pathfind<const MIN_MOVES: usize, const MAX_MOVES: usize>(data: &Array2<u8>) -> i64 {
    let mut heap = BinaryHeap::new();
    let start_pos = (0_usize, 0_usize);
    heap.push((0, start_pos, DIRS[0], 1));
    heap.push((0, start_pos, DIRS[1], 1));

    let mut costs = AHashMap::default();

    let end = (data.dim().0 - 1, data.dim().1 - 1);

    while let Some((cost, pos, last_dir, last_dir_count)) = heap.pop() {
        for dir in DIRS {
            if dir == (-last_dir.0, -last_dir.1) {
                // Crucible not allowed to reverse directions
                continue;
            }

            if dir != last_dir && last_dir_count < MIN_MOVES {
                continue;
            }
            if dir == last_dir && last_dir_count >= MAX_MOVES {
                continue;
            }

            let next_pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );

            if let Some(next_tile_cost) = data.get(next_pos) {
                let next_cost = cost - (*next_tile_cost as i64);

                let dir_count = if dir == last_dir {
                    last_dir_count + 1
                } else {
                    1
                };

                let next = (next_cost, next_pos, dir, dir_count);

                let key = (next_pos, dir, dir_count);
                let prev_cost = *costs.get(&key).unwrap_or(&i64::MIN);

                if next_pos == end && dir_count >= MIN_MOVES {
                    return -next_cost;
                } else if next_cost > prev_cost {
                    heap.push(next);
                    costs.insert(key, next_cost);
                }
            }
        }
    }

    panic!("no solution");
}

fn calculate_p1(data: &Array2<u8>) -> i64 {
    pathfind::<0, 3>(data)
}

fn calculate_p2(data: &Array2<u8>) -> i64 {
    pathfind::<4, 10>(data)
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_17");
    const EXAMPLE_DATA_2: &str = include_str!("../../inputs/examples/2023_17_2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_17");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 102);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 94);
    }

    #[test]
    fn test_p2_example_2() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_2)), 71);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 758);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 892);
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
                let p2 = calculate_p2(&data);
                (p1, p2)
            });
        }
    }
}
