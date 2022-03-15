use std::{collections::HashMap, convert::Infallible, fmt, str::FromStr};

use crate::{
    day_09_16::day12::{to_reg, Ins},
    shared::input_parse,
};

const DAY_NAME: &str = "day23";

pub fn run(suffix: &str) {
    println!("** Day 23");

    let lines = input_parse(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn part1(lines: &Vec<ToggleIns>) -> i32 {
    let mut computer = Computer::new(lines.to_vec());
    computer.regs.insert('a', 7);

    computer.execute();

    computer.regs[&'a']
}

fn part2(lines: &Vec<ToggleIns>) -> i32 {
    let mut computer = Computer::new(lines.to_vec());
    computer.regs.insert('a', 12);

    computer.execute();

    computer.regs[&'a']
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToggleIns {
    Normal(Ins),
    Toggle(Ins),
    NormalTgl(char),
    ToggleTgl(char),
}

impl FromStr for ToggleIns {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("tgl") {
            let reg = to_reg(&s[4..5]);
            Ok(ToggleIns::NormalTgl(reg))
        } else {
            let ins = s.parse::<Ins>()?;
            Ok(ToggleIns::Normal(ins))
        }
    }
}

impl fmt::Display for ToggleIns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToggleIns::Normal(i) => write!(f, " {}", i),
            ToggleIns::Toggle(i) => write!(f, "!{}", i.toggle()),
            ToggleIns::NormalTgl(r) => write!(f, " tgl ${}", r),
            ToggleIns::ToggleTgl(r) => write!(f, "!inc ${}", r),
        }
    }
}

impl Ins {
    fn toggle(self) -> Ins {
        match self {
            Ins::Cpy(a, b) => Ins::Jnz(a, b),
            Ins::Inc(r) => Ins::Dec(r),
            Ins::Dec(r) => Ins::Inc(r),
            Ins::Jnz(a, b) => Ins::Cpy(a, b),
        }
    }
}

impl ToggleIns {
    pub fn show(&self, loc: usize) -> String {
        match self {
            ToggleIns::Normal(i) => format!(" {}", i.show(loc)),
            ToggleIns::Toggle(i) => format!("!{}", i.toggle().show(loc)),
            ToggleIns::NormalTgl(r) => format!(" {}: tgl ${}", loc, r),
            ToggleIns::ToggleTgl(r) => format!("!{}: tgl ${}", loc, r),
        }
    }

    fn toggle(self) -> Self {
        match self {
            ToggleIns::Normal(i) => ToggleIns::Toggle(i),
            ToggleIns::Toggle(i) => ToggleIns::Normal(i),
            ToggleIns::NormalTgl(r) => ToggleIns::ToggleTgl(r),
            ToggleIns::ToggleTgl(r) => ToggleIns::NormalTgl(r),
        }
    }
}

struct Computer {
    program: Vec<ToggleIns>,
    regs: HashMap<char, i32>,
}

impl Computer {
    fn new(program: Vec<ToggleIns>) -> Self {
        let regs = HashMap::from([('a', 0), ('b', 0), ('c', 0), ('d', 0)]);
        Self { program, regs }
    }

    fn execute(&mut self) {
        let mut pc = 0_usize;
        while pc < self.program.len() {
            let step = match self.program[pc] {
                ToggleIns::Normal(i) => i.exec(&mut self.regs),
                ToggleIns::Toggle(i) => i.toggle().exec(&mut self.regs),
                ToggleIns::NormalTgl(r) => self.toggle(pc as i32 + self.regs[&r]),
                ToggleIns::ToggleTgl(r) => Ins::Inc(r).exec(&mut self.regs),
            };
            if step < 0 && (-step) as usize > pc {
                return;
            }
            pc = (pc as i32 + step) as usize;
        }
    }

    fn toggle(&mut self, idx: i32) -> i32 {
        if idx < 0 {
            return 1;
        }
        let i = idx as usize;
        if i < self.program.len() {
            self.program[i] = self.program[i].toggle();
        }
        1
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_parse, part1, part2, ToggleIns, DAY_NAME};

    #[test]
    fn test_parse() {
        let lines: Vec<ToggleIns> = input_parse(DAY_NAME, ".input");

        for (i, l) in lines.iter().enumerate() {
            println!("{}", l.show(i));
        }

        for (i, l) in lines.iter().enumerate() {
            println!("{}", l.toggle().show(i));
        }
    }

    #[test]
    fn first() {
        let lines = input_parse(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 3);
    }
}
