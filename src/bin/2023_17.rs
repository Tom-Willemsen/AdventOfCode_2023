#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ndarray::{Array2, Array4};
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    let mut arr = make_byte_grid(raw_inp.trim());
    arr.mapv_inplace(|x| x - b'0');
    arr
}

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// Optimization: instead of using a Binary heap, which is rather general,
// we can use a much faster implementation based on a ring buffer of stacks.
//
// We can do this because we know that we will only ever have < 10 distinct
// priorities when we do the dijkstra search - since the max edge weight is 9
//
// This is approx 2x faster than a Binary Heap.
struct RingBufferMinPriorityQueue<const SIZE: usize, T> {
    smallest: usize,
    stacks: [Vec<T>; SIZE],
}

impl<const SIZE: usize, T> RingBufferMinPriorityQueue<SIZE, T> {
    const EMPTY_VEC: Vec<T> = vec![];

    fn new(initial_smallest: usize) -> RingBufferMinPriorityQueue<SIZE, T> {
        RingBufferMinPriorityQueue {
            smallest: initial_smallest,
            stacks: [Self::EMPTY_VEC; SIZE],
        }
    }

    fn push(&mut self, priority: usize, item: T) {
        debug_assert!(priority >= self.smallest && priority < self.smallest + SIZE);

        let idx = priority % SIZE;
        self.stacks[idx].push(item);
    }

    fn pop(&mut self) -> Option<(usize, T)> {
        let orig_smallest = self.smallest;

        for idx in orig_smallest..orig_smallest + SIZE {
            if let Some(item) = self.stacks[idx % SIZE].pop() {
                return Some((idx, item));
            }
            self.smallest += 1;
        }
        None
    }
}

struct State {
    pos: (usize, usize),
    last_dir: (isize, isize),
    dir_count: usize,
}

// Dijkstra
fn pathfind<const MIN_MOVES: usize, const MAX_MOVES: usize>(data: &Array2<u8>) -> usize {
    let mut heap: RingBufferMinPriorityQueue<16, _> = RingBufferMinPriorityQueue::new(0);
    heap.push(
        0,
        State {
            pos: (0, 0),
            last_dir: DIRS[0],
            dir_count: 1,
        },
    );
    heap.push(
        0,
        State {
            pos: (0, 0),
            last_dir: DIRS[1],
            dir_count: 1,
        },
    );

    // (dir_count, dir_idx, y, x, )
    let mut costs = Array4::from_elem((MAX_MOVES, 4, data.dim().0, data.dim().1), usize::MAX);

    let end = (data.dim().0 - 1, data.dim().1 - 1);

    while let Some((cost, state)) = heap.pop() {
        if state.pos == end && state.dir_count >= MIN_MOVES {
            return cost;
        }
        for dir_idx in 0..4 {
            let dir = DIRS[dir_idx];
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
                let next_cost = cost + (next_tile_cost as usize);

                let dir_count = if same_as_last_dir {
                    state.dir_count + 1
                } else {
                    1
                };

                let prev_cost = costs[(dir_count - 1, dir_idx, next_pos.0, next_pos.1)];

                if next_cost < prev_cost {
                    heap.push(
                        next_cost,
                        State {
                            pos: next_pos,
                            last_dir: dir,
                            dir_count,
                        },
                    );
                    costs[(dir_count - 1, dir_idx, next_pos.0, next_pos.1)] = next_cost;
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
