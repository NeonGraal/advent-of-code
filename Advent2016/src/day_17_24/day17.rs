use std::{cmp, collections::BinaryHeap};

use crate::shared::{input_string, pt::Pt};

const DAY_NAME: &str = "day17";

pub fn run(suffix: &str) {
    println!("** Day 17");

    let passcode = input_string(DAY_NAME, suffix);

    let result = part1(&passcode);
    println!("- Part1: {}", result);

    let result = part2(&passcode);
    println!("- Part2: {}", result);
}

fn part1(passcode: &str) -> String {
    let mut pending = BinaryHeap::from([State::default()]);
    let end = Pt { x: 3, y: 3 };

    while let Some(next) = pending.pop() {
        if next.curr == end {
            return next.path;
        }
        for p in next.possibles(passcode) {
            pending.push(p);
        }
    }

    "".to_string()
}

fn part2(passcode: &str) -> usize {
    let mut pending = BinaryHeap::from([State::default()]);
    let end = Pt { x: 3, y: 3 };
    let mut max = 0_usize;

    while let Some(next) = pending.pop() {
        if next.curr == end {
            if next.len > max {
                max = next.len
            }
            continue;
        }
        for p in next.possibles(passcode) {
            pending.push(p);
        }
    }

    max
}

#[derive(Debug, Eq, Default)]
struct State {
    curr: Pt,
    path: String,
    len: usize,
}

impl State {
    fn possibles(&self, passcode: &str) -> Vec<State> {
        let mut result = Vec::new();
        let data = passcode.to_string() + &self.path;
        let hash = format!("{:x}", md5::compute(data))
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .collect::<Vec<_>>();

        if self.curr.y > 0 && hash[0] > 10 {
            result.push(self.step(0, -1, "U"));
        }

        if self.curr.y < 3 && hash[1] > 10 {
            result.push(self.step(0, 1, "D"));
        }

        if self.curr.x > 0 && hash[2] > 10 {
            result.push(self.step(-1, 0, "L"));
        }

        if self.curr.x < 3 && hash[3] > 10 {
            result.push(self.step(1, 0, "R"));
        }

        result
    }

    fn step(&self, dx: i32, dy: i32, dp: &str) -> State {
        let curr = self.curr.step(dx, dy);
        let path = self.path.clone() + dp;
        let len = self.len + 1;
        State { curr, path, len }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        other.len.partial_cmp(&self.len)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{part1, part2, Pt, State};

    #[test]
    fn state_ord() {
        let less = State {
            curr: Pt::default(),
            path: "UD".to_string(),
            len: 2,
        };
        let great = State {
            curr: Pt::default(),
            path: "LRUD".to_string(),
            len: 4,
        };

        assert!(less > great);
    }

    #[test]
    fn first_fail() {
        let result = part1("hijkl");
        assert_eq!(result, "");
    }

    #[test]
    fn first() {
        let result = part1("ihgpwlah");
        assert_eq!(result, "DDRRRD");
    }

    #[test]
    fn first_1() {
        let result = part1("kglvqrro");
        assert_eq!(result, "DDUDRLRRUDRD");
    }

    #[test]
    fn first_2() {
        let result = part1("ulqzkmiv");
        assert_eq!(result, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn second() {
        let result = part2("ihgpwlah");

        assert_eq!(result, 370);
    }

    #[test]
    fn second_1() {
        let result = part2("kglvqrro");

        assert_eq!(result, 492);
    }

    #[test]
    fn second_2() {
        let result = part2("ulqzkmiv");

        assert_eq!(result, 830);
    }
}
