use std::{num::ParseIntError, str::FromStr};

use crate::shared::input_parse;

const DAY_NAME: &str = "day21";

pub fn run(suffix: &str) {
    println!("** Day 21");

    let ops = input_parse(DAY_NAME, suffix);

    let result = part1(&ops, "abcdefgh");
    println!("- Part1: {}", result);

    let result = part2(&ops, "fbgdceah");
    println!("- Part2: {}", result);
}

fn part1(ops: &Vec<Op>, input: &str) -> String {
    let mut result = input.to_string();

    for o in ops {
        result = o.act(&result);
    }

    result
}

fn part2(ops: &Vec<Op>, input: &str) -> String {
    let mut result = input.to_string();

    for o in ops.iter().rev() {
        result = o.rev(&result);
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    SwapPos(usize, usize),
    SwapLet(String, String),
    RotLeft(usize),
    RotRight(usize),
    RotBased(String),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_ascii_whitespace()
            .collect::<Vec<_>>();
        let nums = parts
            .iter()
            .filter_map(|&f| f.parse::<usize>().ok())
            .collect::<Vec<_>>();
        if parts[0] == "swap" && parts[1] == "position" {
            return Ok(Op::SwapPos(nums[0], nums[1]));
        }
        if parts[0] == "swap" && parts[1] == "letter" {
            return Ok(Op::SwapLet(parts[2].to_string(), parts[5].to_string()));
        }
        if parts[0] == "rotate" && parts[1] == "left" {
            return Ok(Op::RotLeft(nums[0]));
        }
        if parts[0] == "rotate" && parts[1] == "right" {
            return Ok(Op::RotRight(nums[0]));
        }
        if parts[0] == "rotate" && parts[1] == "based" {
            return Ok(Op::RotBased(parts[6].to_string()));
        }
        if parts[0] == "reverse" {
            return Ok(Op::Reverse(nums[0], nums[1]));
        }
        if parts[0] == "move" {
            return Ok(Op::Move(nums[0], nums[1]));
        }
        panic!("Invalid Op: {}", s)
    }
}

fn swap_locs(input: &str, a: usize, b: usize) -> String {
    if b < a {
        swap_locs(input, b, a)
    } else if b == a {
        input.to_string()
    } else {
        format!(
            "{}{}{}{}{}",
            &input[..a],
            &input[b..=b],
            &input[a + 1..b],
            &input[a..=a],
            &input[b + 1..]
        )
    }
}

fn rot_left(input: &str, r: usize) -> String {
    let by = r % input.len();
    format!("{}{}", &input[by..], &input[..by])
}

fn rot_right(input: &str, r: usize) -> String {
    let len = input.len();
    let by = len - r % len;
    format!("{}{}", &input[by..], &input[..by])
}

fn rot_based(input: &str, a: &str) -> String {
    let p = input.find(a).unwrap();
    if p > 3 {
        rot_right(input, p + 2)
    } else {
        rot_right(input, p + 1)
    }
}

fn rev_based(input: &str, a: &str) -> String {
    for i in 0..input.len() {
        let result = rot_left(input, i);
        if rot_based(&result, a) == input {
            return result;
        }
    }
    input.to_string()
}

fn reverse(input: &str, a: usize, b: usize) -> String {
    if a == b {
        input.to_string()
    } else if b < a {
        reverse(input, b, a)
    } else {
        let middle = input[a..=b]
            .chars()
            .rev()
            .collect::<String>();
        format!("{}{}{}", &input[..a], middle, &input[b + 1..])
    }
}

fn displace(input: &str, a: usize, b: usize) -> String {
    let mut r = input.to_string();
    let t = r.remove(a);
    r.insert(b, t);
    r
}

fn rev_displace(input: &str, a: usize, b: usize) -> String {
    let mut r = input.to_string();
    let t = r.remove(b);
    r.insert(a, t);
    r
}

impl Op {
    fn act(&self, input: &str) -> String {
        match self {
            Op::SwapPos(a, b) => swap_locs(input, *a, *b),
            Op::SwapLet(a, b) => swap_locs(input, input.find(a).unwrap(), input.find(b).unwrap()),
            Op::RotLeft(r) => rot_left(input, *r),
            Op::RotRight(r) => rot_right(input, *r),
            Op::RotBased(a) => rot_based(input, a),
            Op::Reverse(a, b) => reverse(input, *a, *b),
            Op::Move(a, b) => displace(input, *a, *b),
        }
    }

    fn rev(&self, input: &str) -> String {
        match self {
            Op::SwapPos(a, b) => swap_locs(input, *a, *b),
            Op::SwapLet(a, b) => swap_locs(input, input.find(a).unwrap(), input.find(b).unwrap()),
            Op::RotLeft(r) => rot_right(input, *r),
            Op::RotRight(r) => rot_left(input, *r),
            Op::RotBased(a) => rev_based(input, a),
            Op::Reverse(a, b) => reverse(input, *a, *b),
            Op::Move(a, b) => rev_displace(input, *a, *b),
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_parse, part1, part2, Op, DAY_NAME};

    #[test]
    fn first() {
        let ops: Vec<Op> = input_parse(DAY_NAME, ".sample");
        let mut result = "abcde".to_string();

        for o in &ops {
            println!("{} -> {:?}", result, o);
            result = o.act(&result);
        }

        assert_eq!(result, "decab");
    }

    #[test]
    fn based() {
        let before = "ecabd".to_string();
        let op = Op::RotBased("b".to_string());

        let after = op.act(&before);

        let result = op.rev(&after);

        assert_eq!(result, before, "{}", after);
    }

    #[test]
    fn swap_pos() {
        let before = "abcde".to_string();
        let op = Op::SwapPos(0, 4);

        let after = op.act(&before);

        let result = op.rev(&after);

        assert_eq!(result, before, "{}", after);
    }

    #[test]
    fn second() {
        let ops: Vec<Op> = input_parse(DAY_NAME, ".sample");
        let mut result = "decab".to_string();

        for o in ops.iter().rev() {
            println!("{} -> {:?}", result, o);
            result = o.rev(&result);
        }

        assert_eq!(result, "acbde");
    }
}
