#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashSet;
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

fn calculate(data: &Array2<u8>, initial_pos: (usize, usize), initial_dir: (isize, isize)) -> usize {
    let fake_start = (
        initial_pos.0.wrapping_add_signed(-initial_dir.0),
        initial_pos.1.wrapping_add_signed(-initial_dir.1),
    );

    let mut starts = vec![(fake_start, initial_dir)];

    let mut energised = Array2::from_elem(data.dim(), false);

    let mut seen: AHashSet<((usize, usize), (isize, isize))> = AHashSet::with_capacity(1024);

    while let Some((start_pos, dir)) = starts.pop() {
        // If this start/dir combination has been seen already, don't process it.
        // Beam can go in an infinite loop
        if !seen.insert((start_pos, dir)) {
            continue;
        }

        let mut y = start_pos.0.wrapping_add_signed(dir.0);
        let mut x = start_pos.1.wrapping_add_signed(dir.1);

        while let Some(&grid_cell) = data.get((y, x)) {
            energised[(y, x)] = true;

            match (dir, grid_cell) {
                (NORTH, b'-') => break,
                (SOUTH, b'-') => break,
                (EAST, b'|') => break,
                (WEST, b'|') => break,
                (_, b'/') => break,
                (_, b'\\') => break,
                _ => {}
            }

            y = y.wrapping_add_signed(dir.0);
            x = x.wrapping_add_signed(dir.1);
        }

        let end = (y, x);

        if let Some(&cell) = data.get(end) {
            match (dir, cell) {
                (NORTH, b'/') => starts.push((end, EAST)),
                (SOUTH, b'/') => starts.push((end, WEST)),
                (EAST, b'/') => starts.push((end, NORTH)),
                (WEST, b'/') => starts.push((end, SOUTH)),
                (NORTH, b'\\') => starts.push((end, WEST)),
                (SOUTH, b'\\') => starts.push((end, EAST)),
                (EAST, b'\\') => starts.push((end, SOUTH)),
                (WEST, b'\\') => starts.push((end, NORTH)),
                (EAST, b'|') => {
                    starts.push((end, NORTH));
                    starts.push((end, SOUTH));
                }
                (WEST, b'|') => {
                    starts.push((end, NORTH));
                    starts.push((end, SOUTH));
                }
                (NORTH, b'-') => {
                    starts.push((end, EAST));
                    starts.push((end, WEST));
                }
                (SOUTH, b'-') => {
                    starts.push((end, EAST));
                    starts.push((end, WEST));
                }
                _ => {}
            }
        }
    }

    energised.iter().filter(|&&x| x).count()
}

fn calculate_p1(data: &Array2<u8>) -> usize {
    calculate(data, (0, 0), EAST)
}

fn calculate_p2(data: &Array2<u8>) -> usize {
    let (best_y, best_x) = rayon::join(
        || {
            (0..data.dim().0)
                .into_par_iter()
                .map(|y| {
                    let (east, west) = rayon::join(
                        || calculate(data, (y, 0), EAST),
                        || calculate(data, (y, data.dim().1 - 1), WEST),
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
                        || calculate(data, (0, x), SOUTH),
                        || calculate(data, (data.dim().0 - 1, x), NORTH),
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
