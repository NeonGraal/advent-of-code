use std::fmt;

use crate::shared::input_string;

const DAY_NAME: &str = "day16";

pub fn run(suffix: &str) {
    println!("** Day 16");

    let s = input_string(DAY_NAME, suffix);

    let result = part1(&s, 272);
    println!("- Part1: {}", result);

    let result = part1(&s, 35651584);
    println!("- Part2: {}", result);
}

fn part1(initial: &String, size: usize) -> String {
    let mut dragon = Dragon::from(initial);

    while dragon.data.len() < size {
        dragon = dragon.extend();
    }

    dragon.checksum(size).to_string()
}

#[derive(Debug, Clone)]
struct Dragon {
    data: Vec<bool>,
}

impl Dragon {
    fn from(input: &str) -> Dragon {
        let data = input
            .chars()
            .map(|c| c == '1')
            .collect();
        Dragon { data }
    }

    fn extend(&self) -> Dragon {
        let mut b = self
            .data
            .iter()
            .rev()
            .map(|&b| !b)
            .collect::<Vec<_>>();
        let mut data = self.data.clone();
        data.push(false);
        data.append(&mut b);
        Dragon { data }
    }

    fn checksum(&self, len: usize) -> Dragon {
        let mut data = self.data.clone();
        data.truncate(len);

        while data.len() % 2 == 0 {
            data = data
                .chunks(2)
                .map(|c| c[0] == c[1])
                .collect();
        }
        Dragon { data }
    }
}

impl fmt::Display for Dragon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from_iter(
            self.data
                .iter()
                .map(|&b| if b { "1" } else { "0" }),
        );
        write!(f, "{}", s)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{part1, Dragon, DAY_NAME};

    #[test]
    fn check_dragon() {
        let mut dragon = Dragon::from("1").extend();
        assert_eq!(dragon.to_string(), "100");

        dragon = Dragon::from("0").extend();
        assert_eq!(dragon.to_string(), "001");

        dragon = Dragon::from("11111").extend();
        assert_eq!(dragon.to_string(), "11111000000");

        dragon = Dragon::from("111100001010").extend();
        assert_eq!(dragon.to_string(), "1111000010100101011110000");
    }

    #[test]
    fn check_checksum() {
        let dragon = Dragon::from("110010110100101").checksum(12);
        assert_eq!(dragon.to_string(), "100");
    }

    #[test]
    fn first() {
        let lines = "10000".to_string();

        let result = part1(&lines, 20);

        assert_eq!(result, "01100");
    }
}
