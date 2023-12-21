use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::{io, iter};
use std::io::Write;
use crate::common::lcm;

#[derive(Debug)]
enum ModuleState<'a> {
    Broadcast,
    FlipFlop { active: bool },
    Conjunction {
        memory: HashMap<&'a str, bool>,
    },
    Noop,
}

#[derive(Debug)]
struct Module<'a> {
    dest: Vec<&'a str>,
    state: ModuleState<'a>,
}

struct Pulse<'a> {
    src: &'a str,
    dest: &'a str,
    value: bool,
}

impl<'a> Debug for Pulse<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -- {} --> {}",
               self.src,
               if self.value == HIGH { "HIGH" } else { "LOW" },
               self.dest)
    }
}

const HIGH: bool = true;
const LOW: bool = false;

impl<'a> Pulse<'a> {
    fn new(src: &'a str, dest: &'a str, value: bool) -> Self {
        Pulse {
            src,
            dest,
            value,
        }
    }
}

struct ModuleSetup<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> ModuleSetup<'a> {
    fn push_button(&mut self, target: &str) -> (u32, u32, &'a str) {
        // println!("PUSH BUTTON PUSH BUTTON");
        let mut low_count = 0;
        let mut high_count = 0;
        let mut target_sent_low = "";
        let mut queue = VecDeque::new();
        queue.push_back(Pulse::new("button", "broadcaster", LOW));
        while let Some(pulse) = queue.pop_front() {
            match pulse.value {
                HIGH => high_count += 1,
                LOW => {
                    low_count += 1;
                }
            };
            // println!("{:?}", pulse);
            let module = self.modules
                .entry(pulse.dest)
                .or_insert_with(|| Module {
                    dest: vec![],
                    state: ModuleState::Noop,
                });
            // let module = self.modules.get_mut(pulse.dest).unwrap();
            match &mut module.state {
                ModuleState::Noop => {}
                ModuleState::Broadcast => {
                    send_pulse(&mut queue, pulse.dest, &module.dest, pulse.value);
                }
                ModuleState::FlipFlop { active } => {
                    if pulse.value == LOW {
                        *active = !*active;
                        send_pulse(&mut queue, pulse.dest, &module.dest, *active);
                    }
                }
                ModuleState::Conjunction { memory } => {
                    memory.entry(pulse.src)
                        .and_modify(|v| *v = pulse.value)
                        .or_insert(LOW);
                    if pulse.dest == target && pulse.value {
                        target_sent_low = pulse.src;
                    }

                    let new_pulse = !memory.values().all(|x| *x);
                    send_pulse(&mut queue, pulse.dest, &module.dest, new_pulse);
                    // send_pulse(&mut queue, pulse.dest, &module.dest, !pulse.value);
                }
            }
        }
        (low_count, high_count, target_sent_low)
    }
}

fn send_pulse<'a>(queue: &mut VecDeque<Pulse<'a>>, src: &'a str, dest: &Vec<&'a str>, value: bool) {
    dest.iter()
        .for_each(|d| queue.push_back(Pulse::new(src, d, value)))
}

fn parse_modules(input: &str) -> HashMap<&str, Module> {
    let mut conjunctions = HashSet::new();
    let mut modules = input
        .lines()
        .map(|l| {
            let (raw_name, raw_dest) = l.split_once(" -> ").unwrap();
            let dest = raw_dest.split(", ").collect();
            match raw_name.chars().next().unwrap() {
                'b' => (raw_name, Module {
                    dest,
                    state: ModuleState::Broadcast,
                }),
                '%' => (&raw_name[1..], Module {
                    dest,
                    state: ModuleState::FlipFlop { active: false },
                }),
                '&' => {
                    conjunctions.insert(&raw_name[1..]);
                    (&raw_name[1..], Module {
                        dest,
                        state: ModuleState::Conjunction {
                            memory: HashMap::new(),
                        },
                    })
                }
                _ => (raw_name, Module {
                    dest,
                    state: ModuleState::Noop,
                })
            }
        })
        .collect::<HashMap<_, _>>();

    let keys = modules.keys().copied().collect::<Vec<_>>();
    for name in keys {
        let dest = &modules.get(name).unwrap().dest;
        let c_updates = dest.iter().filter(|x| conjunctions.contains(**x)).copied().collect::<Vec<_>>();
        for d in c_updates {
            let c = modules.get_mut(d).unwrap();
            if let ModuleState::Conjunction { memory } = &mut c.state {
                memory.insert(name, LOW);
            }
        }
    }
    modules
}

pub fn part_01() {
    let input = include_str!("../inputs/input_20");
    let modules = parse_modules(input);
    let mut setup = ModuleSetup {
        modules,
    };
    let counts = (0..1000)
        .map(|_| setup.push_button(""))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    println!("Day 20 : Part 1 : {:?} {}", counts, counts.0 * counts.1);
}


pub fn part_02() {
    let input = include_str!("../inputs/input_20");
    let modules = parse_modules(input);
    let mut setup = ModuleSetup {
        modules,
    };
    let s = &setup.modules.get("vr").unwrap().state;
    let expected = match s {
        ModuleState::Conjunction { memory } => memory.len(),
        _ => panic!("shouldn't happen")
    };
    let mut count = 0;
    let mut first_counts = HashMap::new();
    loop {
        count += 1;
        let (_, _, found) = setup.push_button("vr");
        if found.len() > 0 {
            if !first_counts.contains_key(found) {
                println!("{} {}", found, count);
                first_counts.insert(found, count);
                if first_counts.len() == expected {
                    break;
                }
            }
        }
    }
    println!("{:?}", first_counts);

    // let counts = (0..1000)
    //     .map(|_| setup.push_button(""))
    //     .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    let total = first_counts
        .values()
        .fold(1u64, |acc, x| lcm(acc, *x as u64));
    println!("Day 20 : Part 2 : {}", total);
}

