#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use std::fs;

fn dodgy_min_cut(edges: &[(&str, &str)]) -> Option<usize> {
    let mut vertices = AHashSet::<&str>::default();

    for (a, b) in edges {
        vertices.insert(a);
        vertices.insert(b);
    }

    let mut graph = AHashSet::<&str>::default();
    graph.insert(vertices.iter().next().expect("no vertices?"));

    while graph.len() < vertices.len() - 3 {
        let mut candidate_vertices = AHashMap::<&str, usize>::default();
        for (a, b) in edges.iter() {
            // If there's an edge linking any vertex in the graph so far to any vertex
            // outside the graph...
            let a_in_graph = graph.contains(a);
            let b_in_graph = graph.contains(b);
            if a_in_graph != b_in_graph {
                if !a_in_graph {
                    *candidate_vertices.entry(a).or_insert(0) += 1;
                }
                if !b_in_graph {
                    *candidate_vertices.entry(b).or_insert(0) += 1;
                }
            }
        }

        if candidate_vertices.values().sum::<usize>() == 3 {
            return Some((graph.len()) * (vertices.len() - graph.len()));
        }

        let &vertex = candidate_vertices
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _)| k)
            .expect("no vertices...");

        graph.insert(vertex);
    }
    None
}

fn parse(raw_inp: &str) -> Vec<(&str, &str)> {
    raw_inp
        .trim()
        .lines()
        .map(|line| line.split_once(": ").expect("invalid format"))
        .flat_map(|(k, v)| v.split(" ").map(move |v| (k, v)))
        .collect()
}

fn calculate(data: &[(&str, &str)]) -> usize {
    // Ok this is horribly hacky but we can literally just retry on failure and hope we pick a first
    // 3 nodes that happen to all be on the same side of the graph. Once that's true, it will work...
    loop {
        if let Some(result) = dodgy_min_cut(data) {
            return result;
        }
    }
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let p1 = calculate(&parse(&inp));
    println!("{}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_25");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_25");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate(&parse(EXAMPLE_DATA)), 54);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate(&parse(REAL_DATA)), 612945);
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
