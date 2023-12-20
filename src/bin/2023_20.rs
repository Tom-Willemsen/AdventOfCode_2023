#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use num::Integer;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ModuleType {
    Broadcaster,
    Flipflop,
    Conjunction,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum FlipFlopState {
    Off,
    On,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Eq, PartialEq)]
struct Module<'a> {
    outputs: Vec<&'a str>,
    typ: ModuleType,
}

const BROADCASTER: &str = "broadcaster";

fn parse(raw_inp: &str) -> AHashMap<&str, Module> {
    raw_inp
        .trim()
        .lines()
        .map(|line| {
            let (src, dest) = line.split_once(" -> ").unwrap();

            let (typ, name) = match &src[0..1] {
                "b" => (ModuleType::Broadcaster, BROADCASTER),
                "%" => (ModuleType::Flipflop, &src[1..]),
                "&" => (ModuleType::Conjunction, &src[1..]),
                _ => unreachable!(),
            };

            let outputs = dest.split(',').map(|s| s.trim()).collect::<Vec<_>>();

            (name, Module { outputs, typ })
        })
        .collect()
}

fn simulate<'a, FB, FP>(
    modules: &AHashMap<&'a str, Module<'a>>,
    should_continue: FB,
    mut on_pulse_received: FP,
) where
    FB: Fn(usize) -> bool,
    FP: FnMut(Pulse, &'a str, usize),
{
    let mut total_pushes = 0;

    let broadcast = modules.get(BROADCASTER).unwrap();

    let mut pulse_queue = VecDeque::with_capacity(512);

    let mut flipflop_states = AHashMap::default();
    let mut conj_states: AHashMap<&str, AHashMap<&str, Pulse>> = AHashMap::default();

    modules.iter().for_each(|(&name, m)| {
        for &tgt in &m.outputs {
            if let Some(tgt_module) = modules.get(tgt) {
                if tgt_module.typ == ModuleType::Conjunction {
                    conj_states
                        .entry(tgt)
                        .and_modify(|e| {
                            e.insert(name, Pulse::Low);
                        })
                        .or_insert_with(|| {
                            let mut m = AHashMap::default();
                            m.insert(name, Pulse::Low);
                            m
                        });
                }
            }
        }
    });

    while should_continue(total_pushes) {
        total_pushes += 1;

        for o in &broadcast.outputs {
            pulse_queue.push_back((Pulse::Low, o, BROADCASTER));
        }

        while let Some((pulse, dest, src)) = pulse_queue.pop_front() {
            on_pulse_received(pulse, dest, total_pushes);

            if let Some(module) = modules.get(dest) {
                let flipflop_state = *flipflop_states.get(&dest).unwrap_or(&FlipFlopState::Off);

                match module.typ {
                    ModuleType::Flipflop => {
                        if pulse == Pulse::Low {
                            let (new_pulse, new_state) = if flipflop_state == FlipFlopState::Off {
                                (Pulse::High, FlipFlopState::On)
                            } else {
                                (Pulse::Low, FlipFlopState::Off)
                            };

                            for d in &module.outputs {
                                pulse_queue.push_back((new_pulse, d, dest));
                            }
                            flipflop_states.insert(dest, new_state);
                        }
                    }
                    ModuleType::Conjunction => {
                        conj_states.get_mut(dest).unwrap().insert(src, pulse);

                        let new_pulse = if conj_states
                            .get(dest)
                            .unwrap()
                            .values()
                            .all(|v| v == &Pulse::High)
                        {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };

                        for d in &module.outputs {
                            pulse_queue.push_back((new_pulse, d, dest));
                        }
                    }
                    ModuleType::Broadcaster => unreachable!(),
                }
            }
        }
    }
}

fn calculate_p1(modules: &AHashMap<&str, Module>) -> i64 {
    let mut total_low_pulses = 1000;
    let mut total_high_pulses = 0;

    simulate(
        modules,
        |p| p < 1000,
        |pulse, _, _| match pulse {
            Pulse::Low => total_low_pulses += 1,
            Pulse::High => total_high_pulses += 1,
        },
    );

    total_low_pulses * total_high_pulses
}

const END_MODULE: &str = "rx";

// Aggressively optimized for the AoC inputs:
// - The structure of the inputs is 4 12-bit cycling binary counters
// - All of them must "line up" for the final output to fire (which
//   is the final puzzle answer we want)
//
// This method extracts those binary counters directly, never needing
// to "push the button".
//
// Should be general enough for any AoC input. The whole AoC problem
// is intractable for "general" inputs anyway.
fn calculate_p2(modules: &AHashMap<&str, Module>) -> usize {
    let final_module_name = modules
        .iter()
        .find(|(_, m)| m.outputs.contains(&END_MODULE))
        .map(|(name, _)| name)
        .expect("can't find module that outputs to rx");

    let accumulators = modules
        .iter()
        .filter(|(_, m)| m.outputs.contains(final_module_name))
        .map(|(name, _)| name)
        .collect::<Vec<_>>();

    let checkers = modules
        .iter()
        .filter(|(_, m)| accumulators.iter().any(|a| m.outputs.contains(a)))
        .map(|(name, _)| name)
        .collect::<Vec<_>>();

    modules
        .get(BROADCASTER)
        .unwrap()
        .outputs
        .iter()
        .map(|m| {
            let mut r = 0;
            let mut current_mod = m;
            let mut f = 1;

            loop {
                match modules.get(current_mod).unwrap().outputs.len() {
                    1 => {}
                    2 => r += f,
                    _ => unreachable!(),
                }

                if let Some(next_mod) = modules
                    .get(current_mod)
                    .unwrap()
                    .outputs
                    .iter()
                    .find(|o| !checkers.contains(o))
                {
                    current_mod = next_mod;
                } else {
                    r += f; // Last module always connected to checker
                    break r;
                }

                f *= 2;
            }
        })
        .fold(1, |acc, e| acc.lcm(&e))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2023_20");
    const EXAMPLE_DATA_2: &str = include_str!("../../inputs/examples/2023_20_2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2023_20");

    #[test]
    fn test_p1_example() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA)), 32000000);
    }

    #[test]
    fn test_p1_example_2() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA_2)), 11687500);
    }

    #[test]
    fn test_p1_real() {
        assert_eq!(calculate_p1(&parse(REAL_DATA)), 788081152);
    }

    #[test]
    fn test_p2_real() {
        assert_eq!(calculate_p2(&parse(REAL_DATA)), 224602011344203);
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
