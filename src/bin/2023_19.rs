#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use std::fs;

#[derive(Debug)]
struct Rule<'a> {
    elem: &'a str,
    op: &'a str,
    n: i64,
    to: &'a str,
}

#[derive(Debug)]
struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Item {
    fn get(&self, dim: &str) -> i64 {
        match dim {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("invalid dim in get()"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HypercubeRange {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl HypercubeRange {
    fn permutations(&self) -> i64 {
        (self.x.1 - self.x.0)
            * (self.m.1 - self.m.0)
            * (self.a.1 - self.a.0)
            * (self.s.1 - self.s.0)
    }

    fn split_at(&self, dim: &str, n: i64) -> (HypercubeRange, HypercubeRange) {
        debug_assert!(dim == "x" || dim == "m" || dim == "a" || dim == "s");

        (
            HypercubeRange {
                x: if dim == "x" { (self.x.0, n) } else { self.x },
                m: if dim == "m" { (self.m.0, n) } else { self.m },
                a: if dim == "a" { (self.a.0, n) } else { self.a },
                s: if dim == "s" { (self.s.0, n) } else { self.s },
            },
            HypercubeRange {
                x: if dim == "x" { (n, self.x.1) } else { self.x },
                m: if dim == "m" { (n, self.m.1) } else { self.m },
                a: if dim == "a" { (n, self.a.1) } else { self.a },
                s: if dim == "s" { (n, self.s.1) } else { self.s },
            },
        )
    }
}

#[derive(Debug)]
struct Data<'a> {
    workflows: AHashMap<&'a str, Vec<Rule<'a>>>,
    items: Vec<Item>,
}

const UNCONDITIONAL_MAP: &str = "!";

fn parse(raw_inp: &str) -> Data {
    let workflows = raw_inp
        .trim()
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once('{').unwrap();
            let rest = rest.strip_suffix('}').unwrap();

            let rules = rest
                .split(',')
                .map(|rule| {
                    if let Some((cond, dest)) = rule.split_once(':') {
                        Rule {
                            elem: &cond[0..1],
                            op: &cond[1..2],
                            n: cond[2..].parse().unwrap(),
                            to: dest,
                        }
                    } else {
                        Rule {
                            elem: "",
                            op: UNCONDITIONAL_MAP,
                            n: 0,
                            to: rule,
                        }
                    }
                })
                .collect::<Vec<_>>();

            (name, rules)
        })
        .collect();

    let items = raw_inp
        .trim()
        .split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let line = line.strip_prefix('{').unwrap();
            let line = line.strip_suffix('}').unwrap();

            let xmas = line
                .split(',')
                .map(|l| l.split_once('=').unwrap())
                .map(|(_, v)| v.parse().unwrap())
                .collect::<Vec<i64>>();

            Item {
                x: xmas[0],
                m: xmas[1],
                a: xmas[2],
                s: xmas[3],
            }
        })
        .collect::<Vec<_>>();

    Data { workflows, items }
}

fn calculate_p1(data: &Data) -> i64 {
    let mut p1 = 0;

    for item in &data.items {
        let mut curr_workflow = "in";

        while curr_workflow != "A" && curr_workflow != "R" {
            let rules = data.workflows.get(curr_workflow).unwrap();

            for rule in rules {
                match rule.op {
                    "<" => {
                        if item.get(rule.elem) < rule.n {
                            curr_workflow = rule.to;
                            break;
                        }
                    }
                    ">" => {
                        if item.get(rule.elem) > rule.n {
                            curr_workflow = rule.to;
                            break;
                        }
                    }
                    UNCONDITIONAL_MAP => {
                        curr_workflow = rule.to;
                        break;
                    }
                    _ => panic!("unknown op {}", rule.op),
                }
            }
        }

        if curr_workflow == "A" {
            p1 += item.x + item.m + item.a + item.s;
        }
    }

    p1
}

fn calculate_p2(data: &Data) -> i64 {
    let mut p2 = 0;
    let mut queue = vec![];

    queue.push((
        "in",
        HypercubeRange {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        },
    ));

    while let Some((workflow_name, range)) = queue.pop() {
        if workflow_name == "A" {
            p2 += range.permutations();
            continue;
        } else if workflow_name == "R" {
            continue;
        }

        let rules = data.workflows.get(workflow_name).unwrap();

        let mut split_off = None;
        let mut unmapped_dest = None;

        for rule in rules {
            if rule.op == UNCONDITIONAL_MAP {
                unmapped_dest = Some(rule.to);
                break;
            }

            let part = match rule.elem {
                "x" => range.x,
                "m" => range.m,
                "a" => range.a,
                "s" => range.s,
                _ => continue,
            };

            let can_split_lt = rule.op == "<" && rule.n > part.0 && rule.n < part.1;
            let can_split_gt = rule.op == ">" && rule.n > part.0 && rule.n + 1 < part.1;

            if can_split_lt || can_split_gt {
                split_off = Some(if can_split_lt {
                    let (r1, r2) = range.split_at(rule.elem, rule.n);
                    ((rule.to, r1), (workflow_name, r2))
                } else {
                    let (r1, r2) = range.split_at(rule.elem, rule.n + 1);
                    ((workflow_name, r1), (rule.to, r2))
                });

                break;
            }
        }

        if let Some((r1, r2)) = split_off {
            queue.push(r1);
            queue.push(r2);
        } else if let Some(unmapped_dest) = unmapped_dest {
            queue.push((unmapped_dest, range));
        } else {
            panic!("should have been either split or unmapped");
        }
    }

    p2
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_19");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_19");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 19114);
    }

    #[test]
    fn test_p2_example() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA)), 167409079868000);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 350678);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 124831893423809);
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
