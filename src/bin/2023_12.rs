#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use rayon::prelude::*;
use std::fs;

struct Data {
    items: Vec<u8>,
    counts: Vec<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    consumed_items: usize,
    consumed_counts: usize,
    current_run: usize,
}

fn parse(raw_inp: &str) -> Vec<Data> {
    raw_inp
        .trim()
        .lines()
        .filter_map(|s| s.split_once(' '))
        .map(|(items, counts)| {
            let c = counts
                .split(',')
                .map(|x| x.parse())
                .collect::<Result<Vec<usize>, _>>()
                .expect("failed parse");

            let i = items.bytes().collect::<Vec<_>>();

            Data {
                items: i,
                counts: c,
            }
        })
        .collect()
}

fn possible_paths_dot(data: &Data, state: State, cache: &mut AHashMap<State, u64>) -> u64 {
    let is_dot_valid =
        state.current_run == 0 || state.current_run == data.counts[state.consumed_counts];

    if is_dot_valid {
        get_possible_paths(
            data,
            State {
                consumed_items: state.consumed_items + 1,
                consumed_counts: if state.current_run > 0 {
                    state.consumed_counts + 1
                } else {
                    state.consumed_counts
                },
                current_run: 0,
            },
            cache,
        )
    } else {
        0
    }
}

fn possible_paths_hash(data: &Data, state: State, cache: &mut AHashMap<State, u64>) -> u64 {
    let is_hash_valid = state.consumed_counts < data.counts.len()
        && state.current_run < data.counts[state.consumed_counts];

    if is_hash_valid {
        get_possible_paths(
            data,
            State {
                consumed_items: state.consumed_items + 1,
                consumed_counts: state.consumed_counts,
                current_run: state.current_run + 1,
            },
            cache,
        )
    } else {
        0
    }
}

fn get_possible_paths(data: &Data, state: State, cache: &mut AHashMap<State, u64>) -> u64 {
    if let Some(&res) = cache.get(&state) {
        return res;
    }

    let counts = &data.counts;
    let items = &data.items;
    let consumed_counts = state.consumed_counts;
    let consumed_items = state.consumed_items;
    let current_run = state.current_run;

    let res = if consumed_counts > counts.len() {
        return 0;
    } else if consumed_items == items.len() {
        let counts_valid = (consumed_counts == counts.len() && current_run == 0)
            || (consumed_counts == counts.len() - 1 && current_run == counts[consumed_counts]);

        if counts_valid {
            1
        } else {
            0
        }
    } else {
        match items[consumed_items] {
            b'?' => {
                possible_paths_dot(data, state, cache) + possible_paths_hash(data, state, cache)
            }
            b'#' => possible_paths_hash(data, state, cache),
            b'.' => possible_paths_dot(data, state, cache),
            _ => panic!("unexpected character!"),
        }
    };

    cache.insert(state, res);
    res
}

fn calculate_p1(data: &[Data]) -> u64 {
    data.par_iter()
        .map(|d| {
            let mut cache = AHashMap::default();
            let initial_state = State {
                consumed_counts: 0,
                consumed_items: 0,
                current_run: 0,
            };
            get_possible_paths(d, initial_state, &mut cache)
        })
        .sum()
}

fn calculate_p2(data: &[Data]) -> u64 {
    data.par_iter()
        .map(|d| {
            let mut new_items = vec![];
            let mut new_counts = vec![];

            for _ in 0..4 {
                new_items.extend(d.items.clone());
                new_items.push(b'?');
            }
            new_items.extend(d.items.clone());

            for _ in 0..5 {
                new_counts.extend(d.counts.clone());
            }

            Data {
                items: new_items,
                counts: new_counts,
            }
        })
        .map(|d| {
            let mut cache = AHashMap::default();
            let initial_state = State {
                consumed_counts: 0,
                consumed_items: 0,
                current_run: 0,
            };
            get_possible_paths(&d, initial_state, &mut cache)
        })
        .sum()
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

    const REAL_DATA: &str = include_str!("../../inputs/real/2023_12");

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 6958);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 6555315065024);
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
