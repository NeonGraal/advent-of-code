use std::collections::HashMap;

use crate::shared::input_lines;

const DAY_NAME: &str = "day06";

pub fn run(suffix: &str) {
    println!("** Day 6");

    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn char_counts(lines: &Vec<String>) -> Vec<HashMap<char, i32>> {
    let mut counts = Vec::<HashMap<char, i32>>::new();
    for _ in lines[0].chars() {
        let map = HashMap::<char, i32>::new();
        counts.push(map);
    }

    for (i, c) in lines.iter().flat_map(|l| l.chars().enumerate()) {
        let count = *counts[i].get(&c).unwrap_or(&0);
        let map = &mut counts[i];
        map.insert(c, count + 1);
    }

    counts
}

fn part1(lines: &Vec<String>) -> String {
    let counts = char_counts(lines);

    let result: String = counts
        .iter()
        .map(|m| m.iter().max_by_key(|(&k, &v)| v).unwrap().0)
        .collect();

    result
}

fn part2(lines: &Vec<String>) -> String {
    let counts = char_counts(lines);

    let result: String = counts
        .iter()
        .map(|m| m.iter().min_by_key(|(&k, &v)| v).unwrap().0)
        .collect();

    result
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, "easter");
    }

    #[test]
    fn second() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part2(&lines);

        assert_eq!(result, "advent");
    }
}
