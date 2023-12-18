#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use std::fs;
use ahash::AHashSet;
use itertools::Itertools;

struct Inst<'a> {
    dir: u8,
    dist: usize,
    colour: &'a str,
}

fn parse(raw_inp: &str) -> Vec<Inst> {
    raw_inp.trim()
        .lines()
        .map(|x| {
            let (dir, rest) = x.split_once(' ').unwrap();
            let (dist, colour) = rest.split_once(' ').unwrap();
            
            Inst {
                dir: dir.bytes().next().unwrap(),
                dist: dist.parse().unwrap(),
                colour: colour,
            }
        })
        .collect()
}

fn calculate_p1(data: &[Inst]) -> usize {
    let mut dug = AHashSet::default();
    dug.insert((0, 0));
    
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    
    for inst in data {
        
        let dir = match inst.dir {
            b'L' => (0, -1),
            b'R' => (0, 1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => panic!("invalid dir"),
        };
        
        for _ in 0..inst.dist {
            y = y + dir.0;
            x = x + dir.1;
            dug.insert((y, x));
        }
    }
    
    let mut q = vec![];
    q.push((1, 1));
    
    while let Some(next) = q.pop() {
        dug.insert((next.0, next.1));
        
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if !dug.contains(&(next.0 + dir.0, next.1 + dir.1)) {
                q.push((next.0 + dir.0, next.1 + dir.1));
            }
        }
    }
    
    dug.len()
}

fn convert_hex(h: &str) -> (i64, u8) {
    let dir = match h.bytes().nth(7).unwrap() {
        b'0' => b'R',
        b'1' => b'D',
        b'2' => b'L',
        b'3' => b'U',
        _ => panic!("invalid dir"),
    };
    
    let h = &h[2..h.len()-2];
    
    let dist = i64::from_str_radix(&h, 16).unwrap();
    
    (dist, dir)
}

fn calculate_p2(data: &[Inst]) -> i64 {
    let mut dug_vertical_lines = vec![];
    
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    
    let mut result = 0;
    
    for inst in data {
        
        let (dist, raw_dir) = convert_hex(inst.colour);
        
        let dir = match raw_dir {
            b'L' => (0, -1),
            b'R' => (0, 1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => panic!("invalid dir"),
        };
        
        let start_y = y;
        let start_x = x;

        y = y + dir.0 * dist;
        x = x + dir.1 * dist;
        
        if raw_dir == b'U' || raw_dir == b'D' {
            dug_vertical_lines.push((x, y.min(start_y), y.max(start_y) - 1));
        } else if raw_dir == b'L' {
            // If going left now, we previously undercounted by stopping
            // too early. So add back in the cells dug precisely along this
            // row.
            result += (start_x - x).abs();
        }
    }
    
    assert!(y == 0 && x == 0, "should have dug in a loop");
    
    let (min_y, max_y) = dug_vertical_lines
        .iter()
        .fold((0, 0), |acc, line| (acc.0.min(line.1), acc.1.max(line.2)));
    
    for y in min_y..max_y+1 {
        let mut crossings = dug_vertical_lines.iter()
            .filter(|line| line.1 <= y && line.2 >= y)
            .map(|line| line.0)
            .collect::<Vec<i64>>();
            
        crossings.sort_unstable();
        
        debug_assert!(crossings.len() % 2 == 0);
            
        crossings.into_iter()
            .tuples::<(_, _)>()
            .for_each(|(x1, x2)| {
                result += (x2 - x1) + 1;
            });
    }
    
    result + 1
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
        assert_eq!(convert_hex("(#70c710)"), (461937, b'R'));
        assert_eq!(convert_hex("(#0dc571)"), (56407, b'D')); 
        assert_eq!(convert_hex("(#5713f0)"), (356671, b'R'));
        assert_eq!(convert_hex("(#d2c081)"), (863240, b'D'));
        assert_eq!(convert_hex("(#59c680)"), (367720, b'R'));
        assert_eq!(convert_hex("(#411b91)"), (266681, b'D'));
        assert_eq!(convert_hex("(#8ceee2)"), (577262, b'L'));
        assert_eq!(convert_hex("(#caa173)"), (829975, b'U'));
        assert_eq!(convert_hex("(#1b58a2)"), (112010, b'L'));
        assert_eq!(convert_hex("(#caa171)"), (829975, b'D'));
        assert_eq!(convert_hex("(#7807d2)"), (491645, b'L'));
        assert_eq!(convert_hex("(#a77fa3)"), (686074, b'U'));
        assert_eq!(convert_hex("(#015232)"), (5411, b'L'));  
        assert_eq!(convert_hex("(#7a21e3)"), (500254, b'U'));
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
