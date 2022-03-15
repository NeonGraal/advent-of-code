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
pub enum Ins {
    Cpy(Val, Val),
    Inc(char),
    Dec(char),
    Jnz(Val, Val),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Val {
    Num(i32),
    Reg(char),
}

impl FromStr for Val {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<i32>()
            .map_or(Val::Reg(to_reg(&s)), |v| Val::Num(v)))
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Num(n) => write!(f, "{}", n),
            Val::Reg(r) => write!(f, "${}", r),
        }
    }
}

impl Val {
    fn val(&self, regs: &HashMap<char, i32>) -> i32 {
        match self {
            Val::Num(n) => *n,
            Val::Reg(r) => regs[&r],
        }
    }
}

impl FromStr for Ins {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = s.split_ascii_whitespace();
        let cmd = p.next().unwrap();
        let first = p.next().unwrap();
        let reg = to_reg(first);
        let first_val = first.parse::<Val>()?;
        let second = p.next();
        let second_val = second.map(|v| v.parse::<Val>());
        Ok(match cmd {
            "inc" => Ins::Inc(reg),
            "dec" => Ins::Dec(reg),
            "jnz" => Ins::Jnz(first_val, second_val.unwrap().unwrap()),
            "cpy" => Ins::Cpy(first_val, second_val.unwrap().unwrap()),
            _ => panic!("Invalid Ins"),
        })
    }
}

pub fn to_reg(first: &str) -> char {
    first.chars().next().unwrap()
}

impl fmt::Display for Ins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ins::Cpy(v, t) => write!(f, "cpy {} {}", v, t),
            Ins::Inc(r) => write!(f, "inc ${}", r),
            Ins::Dec(r) => write!(f, "dec ${}", r),
            Ins::Jnz(r, j) => write!(f, "jnz {} {}", r, j),
        }
    }
}

impl Ins {
    pub fn show(&self, loc: usize) -> String {
        match self {
            Ins::Jnz(r, j) => match j {
                Val::Num(n) => format!("{}: jnz {} :{}", loc, r, n + loc as i32),
                Val::Reg(d) => format!("{}: jnz {} {}", loc, r, d),
            },
            _ => format!("{}: {}", loc, self),
        }
    }

    pub fn exec(self, regs: &mut HashMap<char, i32>) -> i32 {
        if let Ins::Jnz(k, j) = self {
            if k.val(regs) != 0 {
                return j.val(regs);
            }
        }
        match self {
            Ins::Cpy(s, d) => match d {
                Val::Num(_) => None,
                Val::Reg(k) => regs.insert(k, s.val(regs)),
            },
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

        for (i, l) in lines.iter().enumerate() {
            println!("{}", l.show(i));
        }
    }

    #[test]
    fn first() {
        let lines = input_parse(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 42);
    }
}
