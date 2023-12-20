#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2023::{Cli, Parser};
use ahash::AHashMap;
use num::Integer;
use std::cell::RefCell;
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

    let mut pulse_queue = VecDeque::default();

    let mut flipflop_states = AHashMap::default();
    let mut conj_states: AHashMap<(&str, &str), Pulse> = AHashMap::default();

    modules.iter().for_each(|(&name, m)| {
        for &tgt in &m.outputs {
            if let Some(tgt_module) = modules.get(tgt) {
                if tgt_module.typ == ModuleType::Conjunction {
                    conj_states.insert((name, tgt), Pulse::Low);
                }
            }
        }
    });

    while should_continue(total_pushes) {
        total_pushes += 1;

        for o in &broadcast.outputs {
            pulse_queue.push_back((Pulse::Low, o, "broadcaster"));
        }

        while let Some((pulse, dest, src)) = pulse_queue.pop_front() {
            on_pulse_received(pulse, dest, total_pushes);

            if let Some(module) = modules.get(dest) {
                let flipflop_state = *flipflop_states.get(&dest).unwrap_or(&FlipFlopState::Off);

                match module.typ {
                    ModuleType::Flipflop => {
                        if pulse == Pulse::Low {
                            if flipflop_state == FlipFlopState::Off {
                                for d in &module.outputs {
                                    pulse_queue.push_back((Pulse::High, d, dest));
                                }
                                flipflop_states.insert(dest, FlipFlopState::On);
                            } else {
                                for d in &module.outputs {
                                    pulse_queue.push_back((Pulse::Low, d, dest));
                                }
                                flipflop_states.insert(dest, FlipFlopState::Off);
                            }
                        }
                    }
                    ModuleType::Conjunction => {
                        conj_states.insert((src, dest), pulse);

                        if conj_states
                            .iter()
                            .filter(|((_, module_name), _)| module_name == dest)
                            .all(|(_, v)| v == &Pulse::High)
                        {
                            for d in &module.outputs {
                                pulse_queue.push_back((Pulse::Low, d, dest));
                            }
                        } else {
                            for d in &module.outputs {
                                pulse_queue.push_back((Pulse::High, d, dest));
                            }
                        }
                    }
                    _ => unreachable!(),
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

fn calculate_p2(modules: &AHashMap<&str, Module>) -> usize {
    let need_low_pulses = RefCell::new(AHashMap::default());

    let final_module_name = modules
        .iter()
        .find(|(_, m)| m.outputs.contains(&END_MODULE))
        .map(|(name, _)| name)
        .expect("can't find module that outputs to rx");

    modules.iter().for_each(|(&name, m)| {
        for tgt in &m.outputs {
            if tgt == final_module_name {
                assert!(m.typ == ModuleType::Conjunction);
                need_low_pulses.borrow_mut().insert(name, None);
            }
        }
    });

    assert!(
        need_low_pulses.borrow().len() >= 1,
        "didn't find conjunction modules"
    );

    simulate(
        modules,
        |_| need_low_pulses.borrow().values().any(|p| p.is_none()),
        |pulse, dest, button_presses| {
            let mut need_low_pulses = need_low_pulses.borrow_mut();
            if pulse == Pulse::Low {
                if let Some(c) = need_low_pulses.get(dest) {
                    if c.is_none() {
                        need_low_pulses.insert(dest, Some(button_presses));
                    }
                }
            }
        },
    );

    let p2 = need_low_pulses
        .borrow()
        .values()
        .filter_map(|&v| v)
        .fold(1, |acc, e| acc.lcm(&e));
    p2
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let data = parse(&inp);
    let (p1, p2) = rayon::join(|| calculate_p1(&data), || calculate_p2(&data));
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
