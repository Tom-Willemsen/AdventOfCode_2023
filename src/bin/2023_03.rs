#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ndarray::Array2;
use std::fs;

fn parse(raw_inp: &str) -> Array2<u8> {
    let columns = raw_inp
        .trim()
        .bytes()
        .position(|c| c == b'\n')
        .expect("can't get column count");

    Array2::from_shape_vec(
        ((raw_inp.trim().len() + 1) / (columns + 1), columns),
        raw_inp.bytes().filter(|&x| x != b'\n').collect(),
    )
    .expect("can't make array")
}

const SYMBOLS: [u8; 10] = [b'#', b'$', b'%', b'&', b'*', b'+', b'-', b'/', b'=', b'@'];

fn is_near_any_symbol(data: &Array2<u8>, y: usize, x: usize) -> bool {
    for y_diff in [-1, 0, 1] {
        let ny = y.wrapping_add_signed(y_diff);
        for x_diff in [-1, 0, 1] {
            let nx = x.wrapping_add_signed(x_diff);

            if let Some(cell) = data.get((ny, nx)) {
                if SYMBOLS.contains(cell) {
                    return true;
                }
            }
        }
    }
    false
}

fn is_digit(data: &Array2<u8>, y: usize, x: usize) -> bool {
    data.get((y, x))
        .map(|x| x.is_ascii_digit())
        .unwrap_or(false)
}

fn is_first_digit(data: &Array2<u8>, y: usize, x: usize) -> bool {
    is_digit(data, y, x) && !is_digit(data, y, x.wrapping_sub(1))
}

fn make_number(data: &Array2<u8>, y: usize, x: usize) -> u32 {
    data.get((y, x))
        .and_then(|&x| char::from(x).to_digit(10))
        .expect("make_number called on non-digit")
}

fn make_full_number(data: &Array2<u8>, y: usize, mut x: usize) -> u32 {
    // Scan backwards until start of number
    while is_digit(data, y, x.wrapping_sub(1)) {
        x = x.wrapping_sub(1);
    }

    let mut result = make_number(data, y, x);

    // Scan forwards until end of number
    while is_digit(data, y, x + 1) {
        x += 1;
        result *= 10;
        result += make_number(data, y, x);
    }

    result
}

fn calculate_p1(data: &Array2<u8>) -> u32 {
    data.indexed_iter()
        .filter(|((y, x), _)| is_first_digit(data, *y, *x))
        .filter(|((y, x), _)| {
            let mut near = is_near_any_symbol(data, *y, *x);
            let mut x = *x;

            while !near && is_digit(data, *y, x + 1) {
                x += 1;
                near |= is_near_any_symbol(data, *y, x);
            }

            near
        })
        .map(|((y, x), _)| make_full_number(data, y, x))
        .sum()
}

fn numbers_adjacent_to(data: &Array2<u8>, y: usize, x: usize) -> Vec<u32> {
    let mut result = vec![];

    for y_diff in [-1, 0, 1] {
        let ny = y.wrapping_add_signed(y_diff);
        for x_diff in [-1, 0, 1] {
            let nx = x.wrapping_add_signed(x_diff);

            if is_digit(data, ny, nx) && (x_diff == -1 || is_first_digit(data, ny, nx)) {
                result.push(make_full_number(data, ny, nx));
            }
        }
    }

    result
}

fn calculate_p2(data: &Array2<u8>) -> u32 {
    data.indexed_iter()
        .filter(|(_, &value)| value == b'*')
        .map(|((y, x), _)| numbers_adjacent_to(data, y, x))
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter().product::<u32>())
        .sum()
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_03");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_03");

    const TEST_DATA_1: &str = "
.......5......
..7*..*.......
...*13*.......
.......15.....";

    const TEST_DATA_2: &str = "
12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

    const TEST_DATA_3: &str = "
333.3
...*.";

    const TEST_DATA_4: &str = "
....................
..-52..52-..52..52..
..................-.";

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 4361);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 467835);
    }

    #[test]
    fn test_test_data_1() {
        assert_eq!(calculate_p2(&parse(TEST_DATA_1)), 442);
    }

    #[test]
    fn test_test_data_2() {
        assert_eq!(calculate_p1(&parse(TEST_DATA_2)), 925);
        assert_eq!(calculate_p2(&parse(TEST_DATA_2)), 6756);
    }

    #[test]
    fn test_test_data_3() {
        assert_eq!(calculate_p1(&parse(TEST_DATA_3)), 336);
        assert_eq!(calculate_p2(&parse(TEST_DATA_3)), 999);
    }

    #[test]
    fn test_test_data_4() {
        assert_eq!(calculate_p1(&parse(TEST_DATA_4)), 156);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 498559);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 72246648);
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
            b.iter(|| calculate_p2(black_box(&parsed)));
        }
    }
}
