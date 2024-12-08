#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use ndarray::Array2;
use rayon::prelude::*;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    make_byte_grid(raw_inp.trim())
}

type CostMap = AHashMap<(usize, usize), AHashMap<(usize, usize), usize>>;

fn longest_path(
    cost_map: &CostMap,
    visited: &mut AHashSet<(usize, usize)>,
    pos: (usize, usize),
    target: (usize, usize),
    cost_so_far: usize,
) -> Option<usize> {
    if pos == target {
        return Some(cost_so_far);
    }

    visited.insert(pos);

    let res = cost_map
        .get(&pos)
        .expect("landed at an invalid position?")
        .par_iter()
        .filter_map(|(new_pos, cost)| {
            if visited.contains(new_pos) {
                None
            } else {
                longest_path(
                    cost_map,
                    &mut visited.clone(),
                    *new_pos,
                    target,
                    cost_so_far + cost,
                )
            }
        })
        .max();

    visited.remove(&pos);

    res
}

fn neighbours(pos: &(usize, usize)) -> [(usize, usize); 4] {
    let down = (pos.0 + 1, pos.1);
    let up = (pos.0.wrapping_add_signed(-1), pos.1);
    let right = (pos.0, pos.1 + 1);
    let left = (pos.0, pos.1.wrapping_add_signed(-1));

    [left, right, up, down]
}

fn get_decision_points(data: &Array2<u8>) -> AHashSet<(usize, usize)> {
    data.indexed_iter()
        .filter(|(_, &itm)| itm != b'#')
        .filter(|(pos, _)| {
            neighbours(pos)
                .iter()
                .filter_map(|&pt| data.get(pt))
                .filter(|&pt| pt != &b'#')
                .count()
                > 2
        })
        .map(|(pos, _)| pos)
        .collect()
}

fn pathfind_to_any(
    data: &Array2<u8>,
    start: (usize, usize),
    first_step: (usize, usize),
    targets: &AHashSet<(usize, usize)>,
) -> ((usize, usize), usize) {
    let mut cost = 1;

    let mut last_pos = start;
    let mut pos = first_step;

    while !targets.contains(&pos) {
        let new_pos = *neighbours(&pos)
            .iter()
            .filter(|&new_pos| new_pos != &last_pos)
            .find(|&new_pos| {
                let next_tile = data.get(*new_pos);
                next_tile != Some(&b'#') && next_tile.is_some()
            })
            .expect("can't find a new position - dead end?");

        last_pos = pos;
        pos = new_pos;
        cost += 1;
    }

    (pos, cost)
}

fn make_cost_map<const PART: u8>(
    data: &Array2<u8>,
    decision_points: &AHashSet<(usize, usize)>,
) -> CostMap {
    decision_points
        .iter()
        .map(|pos| {
            let down = (pos.0 + 1, pos.1);
            let up = (pos.0.wrapping_add_signed(-1), pos.1);
            let right = (pos.0, pos.1 + 1);
            let left = (pos.0, pos.1.wrapping_add_signed(-1));

            let down_valid =
                PART == 2 || data.get(down) == Some(&b'v') || data.get(down) == Some(&b'.');
            let up_valid = PART == 2 || data.get(up) == Some(&b'^') || data.get(up) == Some(&b'.');
            let right_valid =
                PART == 2 || data.get(right) == Some(&b'>') || data.get(right) == Some(&b'.');
            let left_valid =
                PART == 2 || data.get(left) == Some(&b'<') || data.get(left) == Some(&b'.');

            let mut cost_map = AHashMap::default();

            [up, down, left, right]
                .iter()
                .zip([up_valid, down_valid, left_valid, right_valid].iter())
                .filter(|(_, &valid)| valid)
                .map(|(new_pos, _)| new_pos)
                .filter(|&new_pos| {
                    let next_tile = data.get(*new_pos);
                    next_tile != Some(&b'#') && next_tile.is_some()
                })
                .for_each(|dir| {
                    let (end_pos, cost) = pathfind_to_any(data, *pos, *dir, decision_points);
                    cost_map.insert(end_pos, cost);
                });

            (*pos, cost_map)
        })
        .collect()
}

fn calculate<const PART: u8>(data: &Array2<u8>) -> usize {
    let start = data
        .indexed_iter()
        .find(|((y, _), t)| y == &0 && t == &&b'.')
        .map(|(idx, _)| idx)
        .expect("can't find start");

    let end = data
        .indexed_iter()
        .find(|((y, _), t)| *y == data.dim().1 - 1 && t == &&b'.')
        .map(|(idx, _)| idx)
        .expect("can't find end");

    let mut decision_points = get_decision_points(data);
    decision_points.insert(start);
    decision_points.insert(end);

    let cost_map = make_cost_map::<PART>(data, &decision_points);
    let mut visited = AHashSet::default();

    // There is only one route, which is constant, from start to a decision point, and
    // also from end to a decision point. This lets us prune search space somewhat, by
    // just adding these offsets and then eliminating start/end from the graph.
    let real_start = pathfind_to_any(data, start, (start.0 + 1, start.1), &decision_points);
    let real_end = pathfind_to_any(data, start, (end.0 - 1, end.1), &decision_points);

    longest_path(
        &cost_map,
        &mut visited,
        real_start.0,
        real_end.0,
        real_start.1 + real_end.1,
    )
    .expect("no solution")
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate::<1>(&data);
    let p2 = calculate::<2>(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_23");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_23");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate::<1>(&parse(EXAMPLE_DATA)), 94);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate::<2>(&parse(EXAMPLE_DATA)), 154);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate::<1>(&parse(REAL_DATA)), 2282);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate::<2>(&parse(REAL_DATA)), 6646);
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
                let p1 = calculate::<1>(&data);
                let p2 = calculate::<2>(&data);
                (p1, p2)
            });
        }
    }
}
