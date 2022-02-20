use crate::shared::input_lines;

mod equip;
mod state;

use state::State;

const DAY_NAME: &str = "day11";

pub fn run(suffix: &str) {
    println!("** Day 11");

    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn part1(lines: &Vec<String>) -> usize {
    let start = State::new(lines);
    start.show();

    let result = start.solve(100);

    result.steps
}

fn part2(lines: &Vec<String>) -> usize {
    let mut start = State::new(lines);
    start.insert("elerium".parse().unwrap(), 0);
    start.insert("elerium-compatible".parse().unwrap(), 0);
    start.insert("dilithium".parse().unwrap(), 0);
    start.insert("dilithium-compatible".parse().unwrap(), 0);

    start.show();

    let result = start.solve(100);

    result.steps
}

mod tests {
    #[allow(unused_imports)]
    use super::{equip::Equip, input_lines, part1, part2, state::State, DAY_NAME};

    #[test]
    fn first() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 11);
    }

    #[test]
    fn first_possibles() {
        let lines = input_lines(DAY_NAME, ".sample");
        let mut start = State::new(&lines);
        let hyc = Equip::Chip("hy".to_string());
        let hyg = Equip::Gen("hy".to_string());
        let lic = Equip::Chip("li".to_string());
        let lig = Equip::Gen("li".to_string());

        start = start.step(1, &[&hyc]);
        assert!(start.valid());
        start = start.step(2, &[&hyc, &hyg]);
        assert!(start.valid());
        start = start.step(0, &[&hyc]);
        assert!(start.valid());
        start = start.step(3, &[&hyc, &lic]);
        assert!(start.valid());
        start = start.step(2, &[&hyc]);
        assert!(start.valid());
        start = start.step(3, &[&hyg, &lig]);
        assert!(start.valid());
        start = start.step(2, &[&lic]);
        assert!(start.valid());
        start = start.step(3, &[&lic, &hyc]);

        start.show();

        if !start.finished() {
            for p in start.possibles().iter() {
                p.show();
            }
        }

        assert!(start.valid());
    }
}
