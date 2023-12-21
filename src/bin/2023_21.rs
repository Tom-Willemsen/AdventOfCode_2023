#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashSet;
use ndarray::Array2;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    make_byte_grid(raw_inp.trim())
}

fn calculate_p1<const N: usize>(data: &Array2<u8>) -> usize {
    let mut reachable = AHashSet::default();
    let mut newly_reachable = AHashSet::default();

    let start = data
        .indexed_iter()
        .filter(|(_, itm)| itm == &&b'S')
        .map(|(idx, _)| idx)
        .next()
        .expect("no start");

    reachable.insert(start);

    for _ in 0..N {
        newly_reachable.clear();

        for itm in reachable.iter() {
            let x = itm.1;
            let y = itm.0;
            if data.get((y.wrapping_add_signed(-1), x)) == Some(&b'.') {
                newly_reachable.insert((y.wrapping_add_signed(-1), x));
            }
            if data.get((y, x.wrapping_add_signed(-1))) == Some(&b'.') {
                newly_reachable.insert((y, x.wrapping_add_signed(-1)));
            }
            if data.get((y + 1, x)) == Some(&b'.') {
                newly_reachable.insert((y.wrapping_add_signed(1), x));
            }
            if data.get((y, x + 1)) == Some(&b'.') {
                newly_reachable.insert((y, x.wrapping_add_signed(1)));
            }
        }

        std::mem::swap(&mut reachable, &mut newly_reachable);
    }

    reachable.len() + 1 // +1 because we didn't count "S"
}

#[derive(Debug)]
struct RepetitionParameters {
    // Num reachable squares on last tiles
    // *directly* east/west/north/east of start.
    extremities: [u64; 4],
    // Score per outer corner - for odd tiles
    // Reachable in tile_size/2 steps from corner
    outer_corners: [u64; 4],
    // Score per inner corner - for inner tiles
    // Reachable in 3*tile_size/2 steps from corner
    inner_corners: [u64; 4],
    // Score per even tile
    even_tile_score: u64,
    // Score per odd tile
    odd_tile_score: u64,
}

fn get_repetition_parameters(data: &Array2<u8>) -> RepetitionParameters {
    let mut reachable = AHashSet::default();
    let mut newly_reachable = AHashSet::default();

    let mut data = data.clone();

    let start = data
        .indexed_iter()
        .filter(|(_, itm)| itm == &&b'S')
        .map(|(idx, _)| idx)
        .next()
        .expect("no start");

    data[start] = b'.';

    let start = (start.0 as i64, start.1 as i64);

    reachable.insert((0, 0));

    let y_dim = data.dim().0 as i64;
    let x_dim = data.dim().1 as i64;

    assert!(y_dim == x_dim, "solution assumes dims are the same");

    let full_tile = x_dim;
    let half_tile = full_tile / 2;
    let tile_and_a_half = full_tile + half_tile;
    let half_tile_plus_one = half_tile + 1;
    let tile_and_a_half_plus_one = tile_and_a_half + 1;

    // The least number of steps we can do is 2*full tiles + half tile
    let n_steps = (5 * x_dim) / 2;

    for _ in 0..n_steps {
        newly_reachable.clear();

        for itm in reachable.iter() {
            let x = itm.1;
            let y = itm.0;

            let up = (y - 1, x);
            let down = (y + 1, x);
            let left = (y, x - 1);
            let right = (y, x + 1);

            let up_rem = (
                (up.0 + start.0).rem_euclid(y_dim) as usize,
                (up.1 + start.1).rem_euclid(x_dim) as usize,
            );
            let down_rem = (
                (down.0 + start.0).rem_euclid(y_dim) as usize,
                (down.1 + start.1).rem_euclid(x_dim) as usize,
            );
            let left_rem = (
                (left.0 + start.0).rem_euclid(y_dim) as usize,
                (left.1 + start.1).rem_euclid(x_dim) as usize,
            );
            let right_rem = (
                (right.0 + start.0).rem_euclid(y_dim) as usize,
                (right.1 + start.1).rem_euclid(x_dim) as usize,
            );

            if data.get(up_rem) == Some(&b'.') {
                newly_reachable.insert(up);
            }
            if data.get(down_rem) == Some(&b'.') {
                newly_reachable.insert(down);
            }
            if data.get(left_rem) == Some(&b'.') {
                newly_reachable.insert(left);
            }
            if data.get(right_rem) == Some(&b'.') {
                newly_reachable.insert(right);
            }
        }

        std::mem::swap(&mut reachable, &mut newly_reachable);
    }

    // The four tile scores at the "corners" of the reachable area.
    let extremity_1 = reachable
        .iter()
        .filter(|(y, x)| *x >= -half_tile && *x <= half_tile && *y >= tile_and_a_half_plus_one)
        .count() as u64;
    let extremity_2 = reachable
        .iter()
        .filter(|(y, x)| *x >= -half_tile && *x <= half_tile && *y <= -tile_and_a_half_plus_one)
        .count() as u64;
    let extremity_3 = reachable
        .iter()
        .filter(|(y, x)| *y >= -half_tile && *y <= half_tile && *x >= tile_and_a_half_plus_one)
        .count() as u64;
    let extremity_4 = reachable
        .iter()
        .filter(|(y, x)| *y >= -half_tile && *y <= half_tile && *x <= -tile_and_a_half_plus_one)
        .count() as u64;

    let sc1 = reachable
        .iter()
        .filter(|(y, x)| *x >= half_tile_plus_one && *y >= tile_and_a_half_plus_one)
        .count() as u64;
    let sc2 = reachable
        .iter()
        .filter(|(y, x)| *x >= half_tile_plus_one && *y <= -tile_and_a_half_plus_one)
        .count() as u64;
    let sc3 = reachable
        .iter()
        .filter(|(y, x)| *x <= -half_tile_plus_one && *y >= tile_and_a_half_plus_one)
        .count() as u64;
    let sc4 = reachable
        .iter()
        .filter(|(y, x)| *x <= -half_tile_plus_one && *y <= -tile_and_a_half_plus_one)
        .count() as u64;

    let lc1 = reachable
        .iter()
        .filter(|(y, x)| {
            *x >= half_tile_plus_one
                && *x <= tile_and_a_half
                && *y >= half_tile_plus_one
                && *y <= tile_and_a_half
        })
        .count() as u64;
    let lc2 = reachable
        .iter()
        .filter(|(y, x)| {
            *x >= -tile_and_a_half
                && *x <= -half_tile_plus_one
                && *y >= half_tile_plus_one
                && *y <= tile_and_a_half
        })
        .count() as u64;
    let lc3 = reachable
        .iter()
        .filter(|(y, x)| {
            *x >= half_tile_plus_one
                && *x <= tile_and_a_half
                && *y >= -tile_and_a_half
                && *y <= -half_tile_plus_one
        })
        .count() as u64;
    let lc4 = reachable
        .iter()
        .filter(|(y, x)| {
            *x >= -tile_and_a_half
                && *x <= -half_tile_plus_one
                && *y >= -tile_and_a_half
                && *y <= -half_tile_plus_one
        })
        .count() as u64;

    let even_tile_score = reachable
        .iter()
        .filter(|(y, x)| *x >= -half_tile && *x <= half_tile && *y >= -half_tile && *y <= half_tile)
        .count() as u64;

    let odd_tile_score = reachable
        .iter()
        .filter(|(y, x)| {
            *x >= half_tile_plus_one && *x <= tile_and_a_half && *y >= -half_tile && *y <= half_tile
        })
        .count() as u64;

    RepetitionParameters {
        extremities: [extremity_1, extremity_2, extremity_3, extremity_4],
        inner_corners: [lc1, lc2, lc3, lc4],
        outer_corners: [sc1, sc2, sc3, sc4],
        even_tile_score,
        odd_tile_score,
    }
}

fn calculate_p2<const N: u64>(data: &Array2<u8>) -> u64 {
    let parameters = get_repetition_parameters(data);

    let full_tile = data.dim().0 as u64;
    let n_over_ft = N / full_tile;

    let perimeter_score = (n_over_ft - 1) * parameters.inner_corners.iter().sum::<u64>()
        + n_over_ft * parameters.outer_corners.iter().sum::<u64>();

    let t = N / (full_tile * 2);
    let num_even = (t - 1) * t;
    let num_odd = t * t;

    let even_tiles_score = num_even * parameters.even_tile_score;
    let odd_tiles_score = num_odd * parameters.odd_tile_score;

    let extremes_correction = parameters.extremities.iter().sum::<u64>();

    4 * (even_tiles_score + odd_tiles_score)
        + perimeter_score
        + extremes_correction
        + parameters.even_tile_score
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1::<64>(&data);
    let p2 = calculate_p2::<26501365>(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_21");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_21");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1::<6>(&parse(EXAMPLE_DATA)), 16);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1::<64>(&parse(REAL_DATA)), 3649);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2::<26501365>(&parse(REAL_DATA)), 612941134797232);
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
