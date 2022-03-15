use std::{collections::HashMap, convert::Infallible, fmt, str::FromStr};

use crate::shared::timing::Timing;

use super::{
    day_09_16::day12::{to_reg, Ins},
    shared::input_parse,
};

const DAY_NAME: &str = "day25";

pub fn run(suffix: &str) {
    println!("** Day 25");

    let lines = input_parse(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);
    println!("- Part1: 696495 way too high");

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn part1(lines: &Vec<ClockIns>) -> i32 {
    let mut t = Timing::start(500, 30);

    for i in 0..696495 {
        let mut computer = Computer::new(lines.to_vec(), i);
        if let Some(c) = computer.execute() {
            if c > 4 {
                println!("\n{} {:?}", i, computer.valid);
            }
            t.check(|| format!(" {} {}", i, c));
        } else {
            println!("\n{} {:?}", i, computer.valid);
            return i;
        }
    }
    0
}

fn part2(lines: &Vec<ClockIns>) -> i32 {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClockIns {
    Normal(Ins),
    Out(char),
}

impl FromStr for ClockIns {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("out") {
            let reg = to_reg(&s[4..5]);
            Ok(ClockIns::Out(reg))
        } else {
            let ins = s.parse::<Ins>()?;
            Ok(ClockIns::Normal(ins))
        }
    }
}

impl fmt::Display for ClockIns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClockIns::Normal(i) => write!(f, "{}", i),
            ClockIns::Out(r) => write!(f, "out ${}", r),
        }
    }
}

impl ClockIns {
    fn show(&self, loc: usize) -> String {
        match self {
            ClockIns::Normal(i) => format!("{}", i.show(loc)),
            ClockIns::Out(r) => format!("out ${}", r),
        }
    }
}

struct Computer {
    program: Vec<ClockIns>,
    regs: HashMap<char, i32>,
    expected: i32,
    valid: Vec<(i32, i32)>,
}

impl Computer {
    fn new(program: Vec<ClockIns>, a_val: i32) -> Self {
        Self {
            program,
            regs: HashMap::from([('a', a_val), ('b', 0), ('c', 0), ('d', 0)]),
            expected: 0,
            valid: Vec::new(),
        }
    }

    fn execute(&mut self) -> Option<usize> {
        let mut pc = 0_usize;
        while pc < self.program.len() {
            let result = match self.program[pc] {
                ClockIns::Normal(i) => Some(i.exec(&mut self.regs)),
                ClockIns::Out(r) => self.out(r),
            };
            if let Some(step) = result {
                if step < 0 && (-step) as usize > pc {
                    break;
                }
                pc = (pc as i32 + step) as usize;
            } else {
                return None;
            }
        }
        Some(self.valid.len())
    }

    fn out(&mut self, r: char) -> Option<i32> {
        if self.expected != self.regs[&r] {
            return Some(99);
        }
        self.expected = 1 - self.expected;
        let valid = (self.regs[&'a'], self.regs[&'b']);
        if self.valid.contains(&valid) {
            self.valid.push(valid);
            return None;
        }
        self.valid.push(valid);
        Some(1)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_parse, part1, part2, ClockIns, Computer, DAY_NAME};

    #[test]
    fn test_parse() {
        let lines: Vec<ClockIns> = input_parse(DAY_NAME, ".input");

        for (i, l) in lines.iter().enumerate() {
            println!("{}: {}", i, l.show(i));
        }
    }

    #[test]
    fn first() {
        let lines: Vec<ClockIns> = input_parse(DAY_NAME, ".input");
        let mut i = 0;

        while i < 60 {
            let mut computer = Computer::new(lines.to_vec(), i);
            if let Some(c) = computer.execute() {
                println!("{} {:?}", i, computer.valid);
                if c == 0 {
                    i = i + 1;
                } else {
                    i = i + 2_usize.pow((c - 1) as u32) as i32 + 1;
                }
            } else {
                println!("{} Success {:?}", i, computer.valid);
                i = i + 1;
            }
        }
    }
}
