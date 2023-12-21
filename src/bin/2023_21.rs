#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::grid_util::make_byte_grid;
use advent_of_code_2023::{Cli, Parser};
use ndarray::Array2;
use std::fs;
use ahash::{AHashSet, AHashMap};

fn parse(raw_inp: &str) -> Array2<u8> {
    make_byte_grid(raw_inp.trim())
}

fn calculate_p1<const N: usize>(data: &Array2<u8>) -> usize {
    let mut reachable = AHashSet::default();
    
    let start = data.indexed_iter()
        .filter(|(_, itm)| itm == &&b'S')
        .map(|(idx, _)| idx)
        .next()
        .expect("no start");
        
    reachable.insert(start);
    
    for _ in 0..N {
        let mut newly_reachable = AHashSet::default();
        
        for itm in reachable.iter() {
            let x = itm.1;
            let y = itm.0;
            if data.get((y.wrapping_add_signed(-1), x)) == Some(&b'.') {
                newly_reachable.insert((y.wrapping_add_signed(-1), x));
            }
            if data.get((y, x.wrapping_add_signed(-1))) == Some(&b'.') {
                newly_reachable.insert((y, x.wrapping_add_signed(-1)));
            }
            if data.get((y+1, x)) == Some(&b'.') {
                newly_reachable.insert((y.wrapping_add_signed(1), x));
            }
            if data.get((y, x+1)) == Some(&b'.') {
                newly_reachable.insert((y, x.wrapping_add_signed(1)));
            }
        }
        
        std::mem::swap(&mut reachable, &mut newly_reachable);
    }
    
    reachable.len() + 1
}

fn floodfill(data: &Array2<u8>, start: (usize, usize)) -> Array2<Option<usize>> {
    let mut costs = Array2::from_elem(data.dim(), None);
    
    let mut q = vec![];
    q.push((start, 0));
    
    while let Some((pos, cost)) = q.pop() {
        let y = pos.0;
        let x = pos.1;
        
        let old_cost = costs[pos].unwrap_or(usize::MAX);
        let new_cost = old_cost.min(cost);
        
        costs[pos] = Some(new_cost);
        
        if new_cost < old_cost {
            if let Some(b'.') = data.get((y.wrapping_sub(1), x)) {
                q.push(((y-1, x), cost + 1));
            }
            if let Some(b'.') = data.get((y+1, x)) {
                q.push(((y+1, x), cost + 1));
            }
            if let Some(b'.') = data.get((y, x.wrapping_sub(1))) {
                q.push(((y, x-1), cost + 1));
            }
            if let Some(b'.') = data.get((y, x+1)) {
                q.push(((y, x+1), cost + 1));
            }
        }
    }
    
    costs
}

fn calculate_p2<const N: i64>(data: &Array2<u8>) -> i64 {
    
    let start = data.indexed_iter()
        .filter(|(_, itm)| itm == &&b'S')
        .map(|(idx, _)| idx)
        .next()
        .expect("no start");
        
    let mut data = data.clone();
    data[start] = b'.';
    
    println!("start: {:?}", start);
    
    let score_per_complete_even_tile = data.indexed_iter()
        .filter(|(_, t)| t == &&b'.')
        .filter(|(idx, _)| (idx.0 + idx.1) % 2 == 0)
        .count() as i64;
        
    let score_per_complete_odd_tile = data.indexed_iter()
        .filter(|(_, t)| t == &&b'.')
        .filter(|(idx, _)| (idx.0 + idx.1) % 2 == 1)
        .count() as i64;
    
    let full_tile = data.dim().0 as i64;
    let half_tile = data.dim().0 / 2;
        
    println!("score per complete even tile: {}", score_per_complete_even_tile);
    println!("score per complete odd tile: {}", score_per_complete_odd_tile);
    
    println!("dims: {:?}", data.dim());
    
    let top_left_even_score = data.indexed_iter()
        .filter(|(_, t)| t == &&b'.')
        .filter(|(idx, _)| (idx.0 + idx.1) % 2 == 0)
        .filter(|((y, x), _)| y <= &half_tile && x <= &half_tile)
        .count() as i64;
    
    let bottom_left_even_score = data.indexed_iter()
        .filter(|(_, t)| t == &&b'.')
        .filter(|(idx, _)| (idx.0 + idx.1) % 2 == 0)
        .filter(|((y, x), _)| y >= &half_tile && x <= &half_tile)
        .count() as i64;
    
    let top_right_even_score = data.indexed_iter()
        .filter(|(_, t)| t == &&b'.')
        .filter(|(idx, _)| (idx.0 + idx.1) % 2 == 0)
        .filter(|((y, x), _)| y <= &half_tile && x >= &half_tile)
        .count() as i64;
    
    let bottom_right_even_score = data.indexed_iter()
        .filter(|(_, t)| t == &&b'.')
        .filter(|(idx, _)| (idx.0 + idx.1) % 2 == 0)
        .filter(|((y, x), _)| y >= &half_tile && x >= &half_tile)
        .count() as i64;
        
    println!("top left even score: {}", top_left_even_score);
    println!("bottom left even score: {}", bottom_left_even_score);
    println!("top right even score: {}", top_right_even_score);
    println!("bottom right even score: {}", bottom_right_even_score);
    
    let top = (N - start.0 as i64) / full_tile;
    
    let permiteter_score = (N / full_tile) * (top_left_even_score + bottom_right_even_score + top_right_even_score + bottom_left_even_score) +
        (N / full_tile) * ((score_per_complete_even_tile - top_left_even_score) 
                            + (score_per_complete_even_tile - bottom_right_even_score) 
                            + (score_per_complete_even_tile - top_right_even_score) 
                            + (score_per_complete_even_tile - bottom_left_even_score)) / 2;
    
    println!("permiteter score: {}", permiteter_score);
    
    // Note: not including centre tile at this point.
    //
    // sum_(n=1)^n 2 n = n (n + 1)
    
    let t = N / (2*full_tile);
    
    println!("t={}, 2*t={}", t, 2*t);
    
    let even_tiles_score = if t > 1 { (t-1) * (t) * score_per_complete_even_tile } else { 0 };
    
    let odd_tiles_score = if t > 1 { (t) * (t) * score_per_complete_odd_tile } else { 0 };
    
    println!("even tiles score: {} (4x: {})", even_tiles_score, 4*even_tiles_score);
    println!("odd tiles score: {} (4x: {})", odd_tiles_score, 4*odd_tiles_score);
    
    println!("estimated answer: {}", ((score_per_complete_even_tile as f64 + score_per_complete_odd_tile as f64)) / (full_tile as f64 * full_tile as f64) * N as f64 * N as f64);
    
    let ans = 4 * (even_tiles_score + odd_tiles_score) + permiteter_score + score_per_complete_even_tile;
    
    println!("ans {}", ans);
    ans
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
    fn test_p2_example() {
        // assert_eq!(calculate_p2::<6>(&parse(EXAMPLE_DATA)), 16);
        assert_eq!(calculate_p2::<10>(&parse(EXAMPLE_DATA)), 50);
        assert_eq!(calculate_p2::<50>(&parse(EXAMPLE_DATA)), 1594);
        assert_eq!(calculate_p2::<100>(&parse(EXAMPLE_DATA)), 6536);
        assert_eq!(calculate_p2::<500>(&parse(EXAMPLE_DATA)), 167004);
        assert_eq!(calculate_p2::<1000>(&parse(EXAMPLE_DATA)), 668697);
        assert_eq!(calculate_p2::<5000>(&parse(EXAMPLE_DATA)), 16733044);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1::<64>(&parse(REAL_DATA)), 3649);
    }

    #[test]
    fn test_p2_real() {
        assert!(calculate_p2::<26501365>(&parse(REAL_DATA)) < 614257985123921);
        assert!(calculate_p2::<26501365>(&parse(REAL_DATA)) < 614251892657121);
        assert!(calculate_p2::<26501365>(&parse(REAL_DATA)) < 613473106432011);
        assert!(calculate_p2::<26501365>(&parse(REAL_DATA)) != 613456484430265);
    }

    /*
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
    }*/
}
