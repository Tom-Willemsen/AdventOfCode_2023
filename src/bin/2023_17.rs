#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ndarray::{Array2, Array4};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    let mut arr = make_byte_grid(raw_inp.trim());
    arr.mapv_inplace(|x| x - b'0');
    arr
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    last_dir: (isize, isize),
    dir_count: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra
fn pathfind<const MIN_MOVES: usize, const MAX_MOVES: usize>(data: &Array2<u8>) -> usize {
    let mut heap = BinaryHeap::new();
    let start_pos = (0_usize, 0_usize);
    heap.push(State {
        cost: 0,
        pos: start_pos,
        last_dir: DIRS[0],
        dir_count: 1,
    });
    heap.push(State {
        cost: 0,
        pos: start_pos,
        last_dir: DIRS[1],
        dir_count: 1,
    });

    // (y, x, dir_idx, dir_count)
    let mut costs = Array4::from_elem((data.dim().0, data.dim().1, 4, MAX_MOVES), usize::MAX);

    let end = (data.dim().0 - 1, data.dim().1 - 1);

    while let Some(state) = heap.pop() {
        for (dir_idx, &dir) in DIRS.iter().enumerate() {
            if dir == (-state.last_dir.0, -state.last_dir.1) {
                // Crucible not allowed to reverse directions
                continue;
            }

            let same_as_last_dir = dir == state.last_dir;

            if !same_as_last_dir && state.dir_count < MIN_MOVES {
                continue;
            }
            if same_as_last_dir && state.dir_count >= MAX_MOVES {
                continue;
            }

            let next_pos = (
                state.pos.0.wrapping_add_signed(dir.0),
                state.pos.1.wrapping_add_signed(dir.1),
            );

            if let Some(&next_tile_cost) = data.get(next_pos) {
                let next_cost = state.cost + (next_tile_cost as usize);

                let dir_count = if same_as_last_dir {
                    state.dir_count + 1
                } else {
                    1
                };

                let prev_cost = costs[(next_pos.0, next_pos.1, dir_idx, dir_count - 1)];

                if next_pos == end && dir_count >= MIN_MOVES {
                    return next_cost;
                } else if next_cost < prev_cost {
                    heap.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        last_dir: dir,
                        dir_count,
                    });
                    costs[(next_pos.0, next_pos.1, dir_idx, dir_count - 1)] = next_cost;
                }
            }
        }
    }

    panic!("no solution");
}

fn calculate_p1(data: &Array2<u8>) -> usize {
    pathfind::<0, 3>(data)
}

fn calculate_p2(data: &Array2<u8>) -> usize {
    pathfind::<4, 10>(data)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = rayon::join(|| calculate_p1(&data), || calculate_p2(&data));
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
