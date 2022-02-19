use std::collections::HashMap;

use crate::shared::input_lines;

const DAY_NAME: &str = "day10";

pub fn run(suffix: &str) {
    println!("** Day 10");

    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines, 17, 61);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

struct Bots {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, usize>,
}

impl Bots {
    fn new() -> Bots {
        Bots {
            bots: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    fn give(&mut self, num: usize, val: usize) {
        match self.bots.get_mut(&num) {
            None => self.bots.insert(
                num,
                Bot {
                    chips: vec![val],
                    low: Dest::Unknown,
                    high: Dest::Unknown,
                },
            ),
            Some(b) => {
                b.chips.push(val);

                let bc = b.clone();

                self.distrib(bc);
                None
            }
        };
    }

    fn dest(&mut self, num: usize, low: Dest, high: Dest) {
        let bot = self.bots.get_mut(&num);
        match bot {
            None => self.bots.insert(
                num,
                Bot {
                    chips: Vec::new(),
                    low,
                    high,
                },
            ),
            Some(b) => {
                b.low = low;
                b.high = high;

                let bc = b.clone();

                self.distrib(bc);
                None
            }
        };
    }

    fn distrib(&mut self, bot: Bot) {
        if bot.chips.len() < 2 || bot.low == Dest::Unknown || bot.high == Dest::Unknown {
            return;
        }
        let (low_val, high_val) = bot.low_high();
        match bot.low {
            Dest::Bot(l) => {
                self.give(l, low_val);
                None
            }
            Dest::Output(o) => self.outputs.insert(o, low_val),
            _ => None,
        };
        match bot.high {
            Dest::Bot(h) => {
                self.give(h, high_val);
                None
            }
            Dest::Output(o) => self.outputs.insert(o, high_val),
            _ => None,
        };
    }

    fn process(&mut self, line: &String) {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let nums: Vec<usize> = parts
            .iter()
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();

        if parts[0] == "value" {
            self.give(nums[1], nums[0]);
        } else if parts[0] == "bot" {
            self.dest(
                nums[0],
                Dest::new(parts[5], nums[1]),
                Dest::new(parts[10], nums[2]),
            );
        }
    }
}

fn part1(lines: &Vec<String>, low: usize, high: usize) -> usize {
    let mut bots = Bots::new();
    for line in lines.iter() {
        bots.process(line);

        for (n, b) in bots.bots.iter() {
            if b.chips.contains(&low) && b.chips.contains(&high) {
                return *n;
            }
        }
    }
    0
}

fn part2(lines: &Vec<String>) -> usize {
    let mut bots = Bots::new();
    for line in lines.iter() {
        bots.process(line);
    }

    bots.outputs[&0] * bots.outputs[&1] * bots.outputs[&2]
}

#[derive(Debug, Clone)]
struct Bot {
    chips: Vec<usize>,
    low: Dest,
    high: Dest,
}

impl Bot {
    fn new() -> Bot {
        Bot {
            chips: Vec::new(),
            low: Dest::Unknown,
            high: Dest::Unknown,
        }
    }

    fn low_high(&self) -> (usize, usize) {
        let fst = self.chips[0];
        let snd = self.chips[1];
        if fst < snd {
            (fst, snd)
        } else {
            (snd, fst)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Dest {
    Unknown,
    Bot(usize),
    Output(usize),
}

impl Dest {
    fn new(sort: &str, num: usize) -> Dest {
        if sort == "output" {
            Dest::Output(num)
        } else if sort == "bot" {
            return Dest::Bot(num);
        } else {
            Dest::Unknown
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part1(&lines, 2, 5);

        assert_eq!(result, 2);
    }
}
// cspell:words distrib
