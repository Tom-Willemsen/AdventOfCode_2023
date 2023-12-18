#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use itertools::Itertools;
use std::fs;

struct RawInst<'a> {
    dir: u8,
    dist: i64,
    colour: &'a str,
}

#[derive(Debug, Eq, PartialEq)]
struct Inst {
    dir: u8,
    dist: i64,
}

fn parse(raw_inp: &str) -> Vec<RawInst> {
    raw_inp
        .trim()
        .lines()
        .map(|x| {
            let (dir, rest) = x.split_once(' ').expect("failed split");
            let (dist, colour) = rest.split_once(' ').expect("failed split");

            RawInst {
                dir: dir.as_bytes()[0],
                dist: dist.parse().expect("failed dist parse"),
                colour,
            }
        })
        .collect()
}

fn convert_hex(h: &str) -> Inst {
    let dir = match h.as_bytes()[7] {
        b'0' => b'R',
        b'1' => b'D',
        b'2' => b'L',
        b'3' => b'U',
        _ => panic!("invalid dir"),
    };

    let dist = i64::from_str_radix(&h[2..h.len() - 2], 16).expect("invalid colour code");

    Inst { dir, dist }
}

struct VerticalLine {
    x: i64,
    y_start: i64,
    y_end: i64,
}

fn dig(data: &[Inst]) -> (Vec<VerticalLine>, i64) {
    let mut dug_vertical_lines = vec![];
    let mut undercount = 0;

    let mut x = 0;
    let mut y = 0;

    for inst in data {
        let dir = match inst.dir {
            b'L' => (0, -1),
            b'R' => (0, 1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => panic!("invalid dir"),
        };

        let start_y = y;
        let start_x = x;

        y += dir.0 * inst.dist;
        x += dir.1 * inst.dist;

        if inst.dir == b'U' || inst.dir == b'D' {
            dug_vertical_lines.push(VerticalLine {
                x,
                y_start: y.min(start_y),
                y_end: y.max(start_y) - 1,
            });
        } else if inst.dir == b'L' {
            // If going left now, we previously undercounted by stopping
            // too early. So explicitly add the cells dug precisely along this
            // row.
            undercount += (start_x - x).abs();
        }
    }

    assert!(y == 0 && x == 0, "should have dug in a loop");

    (dug_vertical_lines, undercount)
}

fn dig_and_fill(inst: &[Inst]) -> i64 {
    let (dug_vertical_lines, undercount) = dig(inst);

    // Be extra careful around transition points
    let mut interesting_y = dug_vertical_lines
        .iter()
        .flat_map(|line| [line.y_start, line.y_end + 1])
        .collect::<Vec<_>>();

    interesting_y.sort_unstable();

    undercount
        + 1
        + interesting_y
            .iter()
            .tuple_windows()
            .filter(|(y1, y2)| y2 != y1)
            .map(|(y1, y2)| {
                let mut crossings = dug_vertical_lines
                    .iter()
                    .filter(|line| line.y_start <= *y1 && line.y_end >= *y1)
                    .map(|line| line.x)
                    .collect::<Vec<i64>>();

                crossings.sort_unstable();

                debug_assert!(crossings.len() % 2 == 0);

                (y2 - y1)
                    * crossings
                        .into_iter()
                        .tuples()
                        .map(|(x1, x2)| (x2 - x1) + 1)
                        .sum::<i64>()
            })
            .sum::<i64>()
}

fn calculate_p1(inst: &[RawInst]) -> i64 {
    let actual_instructions = inst
        .iter()
        .map(|raw_inst| Inst {
            dir: raw_inst.dir,
            dist: raw_inst.dist,
        })
        .collect::<Vec<_>>();

    dig_and_fill(&actual_instructions)
}

fn calculate_p2(inst: &[RawInst]) -> i64 {
    let actual_instructions = inst
        .iter()
        .map(|inst| convert_hex(inst.colour))
        .collect::<Vec<_>>();

    dig_and_fill(&actual_instructions)
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_18");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_18");

    #[test]
    fn test_convert_hex() {
        assert_eq!(
            convert_hex("(#70c710)"),
            Inst {
                dist: 461937,
                dir: b'R'
            }
        );
        assert_eq!(
            convert_hex("(#0dc571)"),
            Inst {
                dist: 56407,
                dir: b'D'
            }
        );
        assert_eq!(
            convert_hex("(#5713f0)"),
            Inst {
                dist: 356671,
                dir: b'R'
            }
        );
        assert_eq!(
            convert_hex("(#d2c081)"),
            Inst {
                dist: 863240,
                dir: b'D'
            }
        );
        assert_eq!(
            convert_hex("(#59c680)"),
            Inst {
                dist: 367720,
                dir: b'R'
            }
        );
        assert_eq!(
            convert_hex("(#411b91)"),
            Inst {
                dist: 266681,
                dir: b'D'
            }
        );
        assert_eq!(
            convert_hex("(#8ceee2)"),
            Inst {
                dist: 577262,
                dir: b'L'
            }
        );
        assert_eq!(
            convert_hex("(#caa173)"),
            Inst {
                dist: 829975,
                dir: b'U'
            }
        );
        assert_eq!(
            convert_hex("(#1b58a2)"),
            Inst {
                dist: 112010,
                dir: b'L'
            }
        );
        assert_eq!(
            convert_hex("(#caa171)"),
            Inst {
                dist: 829975,
                dir: b'D'
            }
        );
        assert_eq!(
            convert_hex("(#7807d2)"),
            Inst {
                dist: 491645,
                dir: b'L'
            }
        );
        assert_eq!(
            convert_hex("(#a77fa3)"),
            Inst {
                dist: 686074,
                dir: b'U'
            }
        );
        assert_eq!(
            convert_hex("(#015232)"),
            Inst {
                dist: 5411,
                dir: b'L'
            }
        );
        assert_eq!(
            convert_hex("(#7a21e3)"),
            Inst {
                dist: 500254,
                dir: b'U'
            }
        );
    }

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 62);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 952408144115);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 47527);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 52240187443190);
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
