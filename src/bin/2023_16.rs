#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use bitvec::prelude::*;
use ndarray::Array2;
use rayon::prelude::*;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    make_byte_grid(raw_inp.trim())
}

const NORTH: (isize, isize) = (-1, 0);
const EAST: (isize, isize) = (0, 1);
const SOUTH: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (0, -1);

struct SeenStartLocations {
    y_size: usize,
    x_size: usize,
    seen: BitVec<u32>,
}

impl SeenStartLocations {
    fn new(dims: (usize, usize)) -> SeenStartLocations {
        SeenStartLocations {
            y_size: dims.0,
            x_size: dims.1,
            seen: bitvec![u32, Lsb0; 0; dims.0 * dims.1 * 4],
        }
    }

    // Returns whether this location was already set
    fn insert(&mut self, loc: (usize, usize), dir: (isize, isize)) -> bool {
        let mut result = false;
        if loc.0 < self.y_size && loc.1 < self.x_size {
            let idx = loc.0 * self.x_size * 4
                + loc.1 * 4
                + match dir {
                    NORTH => 0,
                    SOUTH => 1,
                    EAST => 2,
                    WEST => 3,
                    _ => unreachable!(),
                };
            result = *self.seen.get(idx).unwrap();
            self.seen.set(idx, true);
        }
        result
    }
}

fn simulate(data: &Array2<u8>, initial_pos: (usize, usize), initial_dir: (isize, isize)) -> usize {
    let fake_start = (
        initial_pos.0.wrapping_add_signed(-initial_dir.0),
        initial_pos.1.wrapping_add_signed(-initial_dir.1),
    );

    let mut starts = Vec::with_capacity(32);
    starts.push((fake_start, initial_dir));

    let mut energised = bitvec![u32, Lsb0; 0; data.dim().0 * data.dim().1];
    let mut seen_starts = SeenStartLocations::new(data.dim());

    while let Some((start_pos, dir)) = starts.pop() {
        // If this start/dir combination has been seen already, don't process it.
        // Beam can go in an infinite loop
        if seen_starts.insert(start_pos, dir) {
            continue;
        }

        let mut y = start_pos.0.wrapping_add_signed(dir.0);
        let mut x = start_pos.1.wrapping_add_signed(dir.1);

        while let Some(&grid_cell) = data.get((y, x)) {
            energised.set(y * data.dim().1 + x, true);

            match (dir, grid_cell) {
                (NORTH, b'-') => {
                    starts.push(((y, x), EAST));
                    starts.push(((y, x), WEST));
                    break;
                }
                (SOUTH, b'-') => {
                    starts.push(((y, x), EAST));
                    starts.push(((y, x), WEST));
                    break;
                }
                (EAST, b'|') => {
                    starts.push(((y, x), NORTH));
                    starts.push(((y, x), SOUTH));
                    break;
                }
                (WEST, b'|') => {
                    starts.push(((y, x), NORTH));
                    starts.push(((y, x), SOUTH));
                    break;
                }
                (_, b'/') => {
                    starts.push((
                        (y, x),
                        match dir {
                            NORTH => EAST,
                            EAST => NORTH,
                            SOUTH => WEST,
                            WEST => SOUTH,
                            _ => unreachable!(),
                        },
                    ));
                    break;
                }
                (_, b'\\') => {
                    starts.push((
                        (y, x),
                        match dir {
                            NORTH => WEST,
                            EAST => SOUTH,
                            SOUTH => EAST,
                            WEST => NORTH,
                            _ => unreachable!(),
                        },
                    ));
                    break;
                }
                _ => {
                    y = y.wrapping_add_signed(dir.0);
                    x = x.wrapping_add_signed(dir.1);
                }
            }
        }
    }

    energised.count_ones()
}

fn calculate_p1(data: &Array2<u8>) -> usize {
    simulate(data, (0, 0), EAST)
}

fn calculate_p2(data: &Array2<u8>) -> usize {
    let (best_y, best_x) = rayon::join(
        || {
            (0..data.dim().0)
                .into_par_iter()
                .map(|y| {
                    let (east, west) = rayon::join(
                        || simulate(data, (y, 0), EAST),
                        || simulate(data, (y, data.dim().1 - 1), WEST),
                    );
                    east.max(west)
                })
                .max()
                .expect("should have y size")
        },
        || {
            (0..data.dim().1)
                .into_par_iter()
                .map(|x| {
                    let (south, north) = rayon::join(
                        || simulate(data, (0, x), SOUTH),
                        || simulate(data, (data.dim().0 - 1, x), NORTH),
                    );
                    north.max(south)
                })
                .max()
                .expect("should have x size")
        },
    );

    best_y.max(best_x)
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_16");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_16");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 46);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 51);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 7472);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 7716);
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
                let (p1, p2) = rayon::join(|| calculate_p1(&data), || calculate_p2(&data));
                (p1, p2)
            });
        }
    }
}
