#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use std::fs;

fn parse(raw_inp: &str) -> Vec<&str> {
    raw_inp.trim().split(',').collect()
}

fn hash(s: &str) -> usize {
    s.bytes()
        .fold(0, |acc, elem| ((acc + elem as usize) * 17) % 256)
}

fn calculate_p1(data: &[&str]) -> usize {
    data.iter().map(|s| hash(s)).sum()
}

fn calculate_p2_score(library: &[Vec<(&str, usize)>; 256]) -> usize {
    library
        .iter()
        .enumerate()
        .map(|(k, v)| {
            v.iter()
                .enumerate()
                .map(|(idx, itm)| (k + 1) * (idx + 1) * itm.1)
                .sum::<usize>()
        })
        .sum()
}

fn calculate_p2(data: &[&str]) -> usize {
    const EMPTY_VEC: Vec<(&str, usize)> = vec![];
    let mut library: [Vec<(&str, usize)>; 256] = [EMPTY_VEC; 256];

    data.iter().for_each(|s| {
        if let Some((key, value)) = s.split_once('=') {
            let bx = hash(key);
            let new_value: usize = value.parse().unwrap();
            let mut is_new = true;

            for (label, val) in &mut library[bx] {
                if &key == label {
                    is_new = false;
                    *val = new_value;
                    break;
                }
            }

            if is_new {
                library[bx].push((key, new_value));
            }
        } else if let Some((key, _)) = s.split_once('-') {
            library[hash(key)].retain(|elem| elem.0 != key);
        }
    });

    calculate_p2_score(&library)
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_15");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_15");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 1320);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 145);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 513158);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 200277);
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
