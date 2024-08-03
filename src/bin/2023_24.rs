#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use itertools::Itertools;
use ndarray::ArrayView2;
use ndarray_linalg::Inverse;
use std::fs;

#[derive(Debug)]
struct Hailstone {
    pos_x: i128,
    pos_y: i128,
    pos_z: i128,
    vel_x: i128,
    vel_y: i128,
    vel_z: i128,
}

impl Hailstone {
    fn crossing_with(&self, h2: &Hailstone) -> Option<(i128, i128)> {
        let x1 = self.pos_x;
        let y1 = self.pos_y;

        let x2 = self.pos_x + self.vel_x;
        let y2 = self.pos_y + self.vel_y;

        let x3 = h2.pos_x;
        let y3 = h2.pos_y;

        let x4 = h2.pos_x + h2.vel_x;
        let y4 = h2.pos_y + h2.vel_y;

        let d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if d == 0 {
            return None;
        }

        let px = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let py = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        let c = (px / d, py / d);

        Some(c)
    }
}

fn parse(raw_inp: &str) -> Vec<Hailstone> {
    raw_inp
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (pos, vel) = line.trim().split_once(" @ ").unwrap();

            let (px, pos) = pos.split_once(", ").unwrap();
            let (py, pz) = pos.split_once(", ").unwrap();

            let (vx, vel) = vel.split_once(", ").unwrap();
            let (vy, vz) = vel.split_once(", ").unwrap();

            Hailstone {
                pos_x: px.trim().parse().unwrap(),
                pos_y: py.trim().parse().unwrap(),
                pos_z: pz.trim().parse().unwrap(),
                vel_x: vx.trim().parse().unwrap(),
                vel_y: vy.trim().parse().unwrap(),
                vel_z: vz.trim().parse().unwrap(),
            }
        })
        .collect()
}

fn is_crossing_in_future(hailstone: &Hailstone, crossing: &(i128, i128)) -> bool {
    if hailstone.vel_x > 0 && crossing.0 < hailstone.pos_x
        || hailstone.vel_x < 0 && crossing.0 > hailstone.pos_x
        || hailstone.vel_y > 0 && crossing.1 < hailstone.pos_y
        || hailstone.vel_y < 0 && crossing.1 > hailstone.pos_y
    {
        return false;
    }
    true
}

fn calculate_p1<const AREA_MIN: i128, const AREA_MAX: i128>(data: &[Hailstone]) -> usize {
    data.iter()
        .tuple_combinations()
        .filter_map(|(h1, h2)| h1.crossing_with(h2).map(|c| (h1, h2, c)))
        .filter(|(h1, _h2, c)| is_crossing_in_future(h1, c))
        .filter(|(_h1, h2, c)| is_crossing_in_future(h2, c))
        .map(|(_, _, c)| c)
        .filter(|(c1, c2)| c1 >= &AREA_MIN && c1 <= &AREA_MAX && c2 >= &AREA_MIN && c2 <= &AREA_MAX)
        .count()
}

fn calculate_p2(data: &[Hailstone]) -> i128 {
    let h1 = data.first().expect("First hailstone should exist");
    let h2 = data.get(1).expect("Second hailstone should exist");
    let h3 = data.get(2).expect("Third hailstone should exist");

    // ... Large pile of algebra later ...

    #[rustfmt::skip]
    let m = [
        0, h1.vel_z - h2.vel_z, h2.vel_y - h1.vel_y, 0, h2.pos_z - h1.pos_z, h1.pos_y - h2.pos_y,
        0, h1.vel_z - h3.vel_z, h3.vel_y - h1.vel_y, 0, h3.pos_z - h1.pos_z, h1.pos_y - h3.pos_y,
        h2.vel_z - h1.vel_z, 0, h1.vel_x - h2.vel_x, h1.pos_z - h2.pos_z, 0, h2.pos_x - h1.pos_x,
        h3.vel_z - h1.vel_z, 0, h1.vel_x - h3.vel_x, h1.pos_z - h3.pos_z, 0, h3.pos_x - h1.pos_x,
        h1.vel_y - h2.vel_y, h2.vel_x - h1.vel_x, 0, h2.pos_y - h1.pos_y, h1.pos_x - h2.pos_x, 0,
        h1.vel_y - h3.vel_y, h3.vel_x - h1.vel_x, 0, h3.pos_y - h1.pos_y, h1.pos_x - h3.pos_x, 0,
    ];
    let m_f64 = m.iter().map(|&v| v as f64).collect::<Vec<_>>();
    let mat = ArrayView2::from_shape((6, 6), &m_f64).unwrap();

    let inv_m = mat.inv().expect("cannot invert");

    #[rustfmt::skip]
    let rhs = [
        (h2.pos_z * h2.vel_y - h2.pos_y * h2.vel_z) - (h1.pos_z * h1.vel_y - h1.pos_y * h1.vel_z),
        (h3.pos_z * h3.vel_y - h3.pos_y * h3.vel_z) - (h1.pos_z * h1.vel_y - h1.pos_y * h1.vel_z),
        (h2.pos_x * h2.vel_z - h2.pos_z * h2.vel_x) - (h1.pos_x * h1.vel_z - h1.pos_z * h1.vel_x),
        (h3.pos_x * h3.vel_z - h3.pos_z * h3.vel_x) - (h1.pos_x * h1.vel_z - h1.pos_z * h1.vel_x),
        (h2.pos_y * h2.vel_x - h2.pos_x * h2.vel_y) - (h1.pos_y * h1.vel_x - h1.pos_x * h1.vel_y),
        (h3.pos_y * h3.vel_x - h3.pos_x * h3.vel_y) - (h1.pos_y * h1.vel_x - h1.pos_x * h1.vel_y),
    ];
    let rhs_f64 = rhs.iter().map(|&v| v as f64).collect::<Vec<_>>();

    let result = inv_m.dot(&ArrayView2::from_shape((6, 1), &rhs_f64).unwrap());

    result[(0, 0)].round() as i128 + result[(1, 0)].round() as i128 + result[(2, 0)].round() as i128
}

const P1_REAL_MIN: i128 = 200000000000000;
const P1_REAL_MAX: i128 = 400000000000000;

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let p1 = calculate_p1::<P1_REAL_MIN, P1_REAL_MAX>(&data);
    let p2 = calculate_p2(&data);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_24");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_24");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1::<7, 27>(&parse(EXAMPLE_DATA)), 2);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 47);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(
            calculate_p1::<P1_REAL_MIN, P1_REAL_MAX>(&parse(REAL_DATA)),
            17906
        );
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 571093786416929);
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
                let p1 = calculate_p1::<P1_REAL_MIN, P1_REAL_MAX>(&data);
                let p2 = calculate_p2(&data);
                (p1, p2)
            });
        }
    }
}
