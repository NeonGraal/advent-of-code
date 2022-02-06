use std::fmt;

use crate::shared::{input_lines, input_string};

const DAY_NAME: &str  = "day??";

pub fn run(suffix: &str) {
    let s = input_string(DAY_NAME, suffix);
    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&s);
    println!("Part1: {}", result);

    let result = part2(&lines);
    println!("Part2: {}", result);
}

fn part1(lines: &String) -> String {
    todo!()
}

fn part2(lines: &Vec<String>) -> String {
    todo!()
}

mod tests {
    #[allow(unused_imports)]
    use super::{DAY_NAME, input_string, input_lines, part1, part2};

    #[test]
    fn first() {
        let lines = input_string(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, "??");
    }

    #[test]
    fn second() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part2(&lines);

        assert_eq!(result, "??");
    }
}
