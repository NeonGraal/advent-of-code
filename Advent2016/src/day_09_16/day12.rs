use std::{collections::HashMap, convert::Infallible, fmt, str::FromStr};

use crate::shared::input_parse;

const DAY_NAME: &str = "day12";

pub fn run(suffix: &str) {
    println!("** Day 12");

    let lines = input_parse(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn execute(program: &Vec<Ins>, regs: &mut HashMap<char, i32>) {
    let mut pc = 0_usize;
    while pc < program.len() {
        let step = program[pc].exec(regs);
        if step < 0 && (-step) as usize > pc {
            return;
        }
        pc = (pc as i32 + step) as usize;
    }
}

fn part1(program: &Vec<Ins>) -> i32 {
    let mut regs = HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)]);

    execute(program, &mut regs);

    regs[&'a']
}

fn part2(program: &Vec<Ins>) -> i32 {
    let mut regs = HashMap::from([('a', 0), ('b', 0), ('c', 1), ('d', 0)]);

    execute(program, &mut regs);

    regs[&'a']
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ins {
    CpyReg(char, char),
    CpyVal(i32, char),
    Inc(char),
    Dec(char),
    Jnz(char, i32),
}

impl FromStr for Ins {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = s.split_ascii_whitespace();
        let cmd = p.next().unwrap();
        let first = p.next().unwrap();
        let reg = to_reg(first);
        let second = p.next();
        Ok(match cmd {
            "inc" => Ins::Inc(reg),
            "dec" => Ins::Dec(reg),
            "jnz" => second
                .unwrap()
                .parse::<i32>()
                .map(|j| Ins::Jnz(reg, j))
                .unwrap(),
            "cpy" => {
                let second_reg = to_reg(second.unwrap());
                first
                    .parse::<i32>()
                    .map_or(Ins::CpyReg(reg, second_reg), |v| Ins::CpyVal(v, second_reg))
            }
            _ => panic!("Invalid Ins"),
        })
    }
}

fn to_reg(first: &str) -> char {
    first.chars().next().unwrap()
}

impl fmt::Display for Ins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ins::CpyReg(r, t) => write!(f, "cpy {} {}", r, t),
            Ins::CpyVal(v, t) => write!(f, "cpy {} {}", v, t),
            Ins::Inc(r) => write!(f, "inc {}", r),
            Ins::Dec(r) => write!(f, "dec {}", r),
            Ins::Jnz(r, j) => write!(f, "jnz {} {}", r, j),
        }
    }
}

impl Ins {
    fn exec(self, regs: &mut HashMap<char, i32>) -> i32 {
        if let Ins::Jnz(k, j) = self {
            if let Some(v) = k.to_digit(10) {
                if v != 0 {
                    return j;
                }
            } else if regs[&k] != 0 {
                return j;
            }
        }
        match self {
            Ins::CpyReg(r, k) => regs.insert(k, regs[&r]),
            Ins::CpyVal(v, k) => regs.insert(k, v),
            Ins::Inc(k) => regs.insert(k, regs[&k] + 1),
            Ins::Dec(k) => regs.insert(k, regs[&k] - 1),
            Ins::Jnz(_, _) => None,
        };
        1
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_parse, part1, part2, Ins, DAY_NAME};

    #[test]
    fn test_parse() {
        let lines: Vec<Ins> = input_parse(DAY_NAME, ".input");

        for l in lines {
            println!("{}", l);
        }
    }

    #[test]
    fn first() {
        let lines = input_parse(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 42);
    }
}
