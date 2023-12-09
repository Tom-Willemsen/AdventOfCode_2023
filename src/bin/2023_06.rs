#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use anyhow::*;
use num_integer::*;
use std::fs;

#[derive(Clone)]
struct Race {
    time: i64,
    distance: i64,
}

fn parse_line(raw_inp: &str) -> Vec<i64> {
    raw_inp
        .trim()
        .split_ascii_whitespace()
        .skip(1) // "Time:" or "Distance:"
        .map(|x| x.parse())
        .collect::<Result<Vec<i64>, _>>()
        .expect("Couldn't parse line")
}

fn parse(raw_inp: &str) -> Vec<Race> {
    let (times, distances) = raw_inp
        .trim()
        .split_once('\n')
        .map(|(t, d)| (parse_line(t), parse_line(d)))
        .expect("not enough lines");

    times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Race { time, distance })
        .collect()
}

fn ways_to_win(race: &Race) -> i64 {
    let win_dist = race.distance + 1;
    let discriminant = race.time * race.time - 4 * win_dist;
    let root_discriminant = discriminant.sqrt();

    let mut minimum_charge_time = (race.time - root_discriminant) / 2;
    let mut maximum_charge_time = (race.time + root_discriminant) / 2;

    if minimum_charge_time * (race.time - minimum_charge_time) < win_dist {
        minimum_charge_time += 1;
    }

    if (maximum_charge_time + 1) * (race.time - (maximum_charge_time + 1)) >= win_dist {
        maximum_charge_time += 1;
    }

    maximum_charge_time - minimum_charge_time + 1
}

fn calculate_p1(data: &[Race]) -> i64 {
    data.iter().map(ways_to_win).product()
}

fn next_power_of_10(n: i64) -> i64 {
    10_i64.pow(n.ilog10() + 1)
}

fn calculate_p2(data: Vec<Race>) -> i64 {
    data.into_iter()
        .reduce(|acc, e| Race {
            time: next_power_of_10(e.time) * acc.time + e.time,
            distance: next_power_of_10(e.distance) * acc.distance + e.distance,
        })
        .map(|race| ways_to_win(&race))
        .expect("expected at least one race")
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_06");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_06");

    #[test]
    fn test_simple_winnable_races() {
        // Wins if held for 1 or 2 or 3s
        assert_eq!(
            ways_to_win(&Race {
                time: 4,
                distance: 0
            }),
            3
        );
        assert_eq!(
            ways_to_win(&Race {
                time: 4,
                distance: 1
            }),
            3
        );
        assert_eq!(
            ways_to_win(&Race {
                time: 4,
                distance: 2
            }),
            3
        );

        // Only way to win should be to hold for 2s
        assert_eq!(
            ways_to_win(&Race {
                time: 4,
                distance: 3
            }),
            1
        );
    }

    #[test]
    #[should_panic]
    fn test_simple_unwinnable_race() {
        // Can't win (can only equal the record) - should panic
        assert_eq!(
            ways_to_win(&Race {
                time: 4,
                distance: 4
            }),
            0
        );
    }

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 288);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(parse(EXAMPLE_DATA)), 71503);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 170000);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(parse(REAL_DATA)), 20537782);
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench_parse(b: &mut Bencher) {
            b.iter(|| parse(black_box(REAL_DATA)));
        }

        #[bench]
        fn bench_p1(b: &mut Bencher) {
            let parsed = parse(REAL_DATA);
            b.iter(|| calculate_p1(black_box(&parsed)));
        }

        #[bench]
        fn bench_p2(b: &mut Bencher) {
            let parsed = parse(REAL_DATA);
            b.iter(|| calculate_p2(black_box(parsed.clone())));
        }
    }
}
