use std::fmt;

use crate::shared::input_lines;

pub fn run(suffix: &str) {
    println!("** Day 2");

    let lines = input_lines("day02", suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn part2(lines: &Vec<String>) -> String {
    let mut result = String::new();
    let mut key = Key2::K5;

    for l in lines {
        for c in l.chars() {
            key = key.step(c);
        }
        result += &key.to_string();
    }

    return result;
}

fn part1(lines: &Vec<String>) -> String {
    let mut result = String::new();
    let mut key = Key1::K5;

    for l in lines {
        for c in l.chars() {
            key = key.step(c);
        }
        result += &key.to_string();
    }

    return result;
}

fn select<T>(dir: char, left: T, down: T, right: T, up: T) -> T {
    match dir {
        'L' => left,
        'D' => down,
        'R' => right,
        'U' => up,
        _ => panic!("Incorrect direction character"),
    }
}

#[derive(Debug)]
enum Key1 {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}

impl Key1 {
    pub fn step(self, dir: char) -> Key1 {
        match self {
            Key1::K1 => select(dir, Key1::K1, Key1::K4, Key1::K2, Key1::K1),
            Key1::K2 => select(dir, Key1::K1, Key1::K5, Key1::K3, Key1::K2),
            Key1::K3 => select(dir, Key1::K2, Key1::K6, Key1::K3, Key1::K3),
            Key1::K4 => select(dir, Key1::K4, Key1::K7, Key1::K5, Key1::K1),
            Key1::K5 => select(dir, Key1::K4, Key1::K8, Key1::K6, Key1::K2),
            Key1::K6 => select(dir, Key1::K5, Key1::K9, Key1::K6, Key1::K3),
            Key1::K7 => select(dir, Key1::K7, Key1::K7, Key1::K8, Key1::K4),
            Key1::K8 => select(dir, Key1::K7, Key1::K8, Key1::K9, Key1::K5),
            Key1::K9 => select(dir, Key1::K8, Key1::K9, Key1::K9, Key1::K6),
        }
    }
}

impl fmt::Display for Key1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key1::K1 => write!(f, "1"),
            Key1::K2 => write!(f, "2"),
            Key1::K3 => write!(f, "3"),
            Key1::K4 => write!(f, "4"),
            Key1::K5 => write!(f, "5"),
            Key1::K6 => write!(f, "6"),
            Key1::K7 => write!(f, "7"),
            Key1::K8 => write!(f, "8"),
            Key1::K9 => write!(f, "9"),
        }
    }
}

#[derive(Debug)]
enum Key2 {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
    KB,
    KC,
    KD,
}

impl Key2 {
    pub fn step(self, dir: char) -> Key2 {
        match self {
            Key2::K1 => select(dir, Key2::K1, Key2::K3, Key2::K1, Key2::K1),
            Key2::K2 => select(dir, Key2::K2, Key2::K6, Key2::K3, Key2::K2),
            Key2::K3 => select(dir, Key2::K2, Key2::K7, Key2::K4, Key2::K1),
            Key2::K4 => select(dir, Key2::K3, Key2::K8, Key2::K4, Key2::K4),
            Key2::K5 => select(dir, Key2::K5, Key2::K5, Key2::K6, Key2::K5),
            Key2::K6 => select(dir, Key2::K5, Key2::KA, Key2::K7, Key2::K2),
            Key2::K7 => select(dir, Key2::K6, Key2::KB, Key2::K8, Key2::K3),
            Key2::K8 => select(dir, Key2::K7, Key2::KC, Key2::K9, Key2::K4),
            Key2::K9 => select(dir, Key2::K8, Key2::K9, Key2::K9, Key2::K9),
            Key2::KA => select(dir, Key2::KA, Key2::KA, Key2::KB, Key2::K6),
            Key2::KB => select(dir, Key2::KA, Key2::KD, Key2::KC, Key2::K7),
            Key2::KC => select(dir, Key2::KB, Key2::KC, Key2::KC, Key2::K8),
            Key2::KD => select(dir, Key2::KD, Key2::KD, Key2::KD, Key2::KB),
        }
    }
}

impl fmt::Display for Key2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key2::K1 => write!(f, "1"),
            Key2::K2 => write!(f, "2"),
            Key2::K3 => write!(f, "3"),
            Key2::K4 => write!(f, "4"),
            Key2::K5 => write!(f, "5"),
            Key2::K6 => write!(f, "6"),
            Key2::K7 => write!(f, "7"),
            Key2::K8 => write!(f, "8"),
            Key2::K9 => write!(f, "9"),
            Key2::KA => write!(f, "A"),
            Key2::KB => write!(f, "B"),
            Key2::KC => write!(f, "C"),
            Key2::KD => write!(f, "D"),
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2};

    #[test]
    fn first() {
        let lines = input_lines("day02", ".sample");

        let result = part1(&lines);

        assert_eq!(result, "1985");
    }

    #[test]
    fn second() {
        let lines = input_lines("day02", ".sample");

        let result = part2(&lines);

        assert_eq!(result, "5DB3");
    }
}
