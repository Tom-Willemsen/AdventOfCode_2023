#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Brick {
    x1: i64,
    y1: i64,
    z1: i64,
    x2: i64,
    y2: i64,
    z2: i64,
}

impl Brick {
    fn min_x(&self) -> i64 {
        self.x1.min(self.x2)
    }

    fn min_y(&self) -> i64 {
        self.y1.min(self.y2)
    }

    fn min_z(&self) -> i64 {
        self.z1.min(self.z2)
    }

    fn max_x(&self) -> i64 {
        self.x1.max(self.x2)
    }

    fn max_y(&self) -> i64 {
        self.y1.max(self.y2)
    }

    fn max_z(&self) -> i64 {
        self.z1.max(self.z2)
    }

    fn x_overlaps(&self, other: &Brick) -> bool {
        (self.min_x() <= other.min_x() && self.max_x() >= other.min_x())
            || (self.min_x() <= other.max_x() && self.max_x() >= other.max_x())
            || (other.min_x() <= self.min_x() && other.max_x() >= self.min_x())
            || (other.min_x() <= self.max_x() && other.max_x() >= self.max_x())
    }

    fn y_overlaps(&self, other: &Brick) -> bool {
        (self.min_y() <= other.min_y() && self.max_y() >= other.min_y())
            || (self.min_y() <= other.max_y() && self.max_y() >= other.max_y())
            || (other.min_y() <= self.min_y() && other.max_y() >= self.min_y())
            || (other.min_y() <= self.max_y() && other.max_y() >= self.max_y())
    }

    fn z_overlaps(&self, other: &Brick) -> bool {
        (self.min_z() <= other.min_z() && self.max_z() >= other.min_z())
            || (self.min_z() <= other.max_z() && self.max_z() >= other.max_z())
            || (other.min_z() <= self.min_z() && other.max_z() >= self.min_z())
            || (other.min_z() <= self.max_z() && other.max_z() >= self.max_z())
    }

    fn overlaps_with(&self, other: &Brick) -> bool {
        self.x_overlaps(other) && self.y_overlaps(other) && self.z_overlaps(other)
    }

    fn overlaps_with_any(&self, others: &[Brick]) -> bool {
        others.iter().any(|b| self.overlaps_with(b))
    }

    fn is_supported_by(&self, other: &Brick) -> bool {
        if self.min_z() == 1 {
            false
        } else {
            self.x_overlaps(other) && self.y_overlaps(other) && self.min_z() == other.max_z() + 1
        }
    }

    fn is_supported(self, others: &[Brick], exclude: &Brick) -> bool {
        self.min_z() == 1
            || others
                .iter()
                .filter(|b| b != &exclude)
                .any(|b| self.is_supported_by(b))
    }

    fn adjust_z_relative(&mut self, z_adj: i64) {
        self.z1 += z_adj;
        self.z2 += z_adj;
    }

    fn adjust_z_absolute(&mut self, z_abs: i64) {
        let z_size = self.max_z() - self.min_z();
        self.z1 = z_abs;
        self.z2 = z_abs + z_size;
    }
}

fn parse(raw_inp: &str) -> Vec<Brick> {
    raw_inp
        .trim()
        .lines()
        .map(|line| {
            let (b1, b2) = line.split_once('~').unwrap();

            let (x1, bb1) = b1.split_once(',').unwrap();
            let (y1, z1) = bb1.split_once(',').unwrap();

            let (x2, bb2) = b2.split_once(',').unwrap();
            let (y2, z2) = bb2.split_once(',').unwrap();

            Brick {
                x1: x1.parse().unwrap(),
                y1: y1.parse().unwrap(),
                z1: z1.parse().unwrap(),
                x2: x2.parse().unwrap(),
                y2: y2.parse().unwrap(),
                z2: z2.parse().unwrap(),
            }
        })
        .collect()
}

fn settled_bricks(data: &[Brick]) -> Vec<Brick> {
    let mut brick_q = data.to_vec();

    brick_q.sort_unstable_by_key(|b| std::cmp::Reverse(b.min_z()));

    let mut brick_pile: Vec<Brick> = vec![];

    let mut top_of_pile = 1;

    while let Some(mut next_brick) = brick_q.pop() {
        next_brick.adjust_z_absolute(top_of_pile);

        while !next_brick.overlaps_with_any(&brick_pile) && next_brick.min_z() >= 1 {
            next_brick.adjust_z_relative(-1);
        }
        next_brick.adjust_z_relative(1);

        top_of_pile = top_of_pile.max(next_brick.max_z() + 1);

        brick_pile.push(next_brick);
    }

    brick_pile
}

fn count_falling_bricks(data: &[&Brick], min_z: i64, max_z: i64) -> i64 {
    data.iter()
        .filter(|&b| b.min_z() > min_z && b.min_z() <= max_z)
        .filter_map(|&b| {
            let mut cloned_b = *b;

            cloned_b.adjust_z_relative(-1);

            let would_conflict = data
                .iter()
                .filter(|&&ob| ob != b)
                .any(|ob| ob.overlaps_with(&cloned_b));

            if !would_conflict {
                Some(
                    count_falling_bricks(
                        &data
                            .iter()
                            .filter(|&&ob| ob != b)
                            .copied()
                            .collect::<Vec<_>>(),
                        min_z,
                        max_z.max(b.max_z() + 1),
                    ) + 1,
                )
            } else {
                None
            }
        })
        .next()
        .unwrap_or(0)
}

fn calculate(data: &[Brick]) -> (i64, i64) {
    let pile = settled_bricks(data);

    let biggest_brick_z = data
        .iter()
        .map(|b| b.max_z() - b.min_z() + 1)
        .max()
        .unwrap_or(1);

    pile.par_iter()
        .map(|removed_brick| {
            let unsupported = pile
                .iter()
                .filter(|b| b.is_supported_by(removed_brick))
                .find(|b| !b.is_supported(&pile, removed_brick));

            if let Some(_zapped) = unsupported {
                let zapped = pile
                    .iter()
                    .filter(|&b| b != removed_brick)
                    .filter(|&b| b.min_z() > removed_brick.min_z() - biggest_brick_z)
                    .collect::<Vec<_>>();

                let min_z = 1.max(removed_brick.min_z());
                let max_z = min_z + biggest_brick_z;
                let fallers = count_falling_bricks(&zapped, min_z, max_z);
                (0, fallers)
            } else {
                (1, 0)
            }
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = calculate(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_22");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_22");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)).0, 5);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)).1, 7);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).0, 507);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate(&parse(REAL_DATA)).1, 51733);
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
                calculate(&data)
            });
        }
    }
}
