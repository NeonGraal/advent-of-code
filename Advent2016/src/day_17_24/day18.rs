use std::{convert::Infallible, fmt, str::FromStr};

use crate::shared::input_string;

const DAY_NAME: &str = "day18";

pub fn run(suffix: &str) {
    println!("** Day 18");

    let start = input_string(DAY_NAME, suffix);

    let result = part1(&start, 40);
    println!("- Part1: {}", result);

    let result = part1(&start, 400000);
    println!("- Part2: {}", result);
}

fn part1(start: &str, lines: usize) -> usize {
    let mut row = start.parse::<Row>().unwrap();
    let mut safe = row.safe();

    for i in 1..lines {
        row = row.gen();
        safe = safe + row.safe();
    }

    safe
}

#[derive(Debug)]
struct Row {
    traps: Vec<bool>,
}

impl FromStr for Row {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let traps = s
            .trim()
            .chars()
            .map(|c| c == '^')
            .collect();
        Ok(Row { traps })
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let traps = String::from_iter(
            self.traps
                .iter()
                .map(|&b| if b { '^' } else { '.' }),
        );
        write!(f, "{}", traps)
    }
}

impl Row {
    fn gen(&self) -> Row {
        let mut traps = Vec::new();
        let mut i = self.traps.iter();
        let mut l = false;
        let mut c = *i.next().unwrap();
        for &r in i {
            traps.push(l ^ r);
            l = c;
            c = r;
        }
        traps.push(l);
        Row { traps }
    }

    fn safe(&self) -> usize {
        self.traps
            .iter()
            .filter(|&&b| !b)
            .count()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_string, part1, Row, DAY_NAME};

    #[test]
    fn check_gen() {
        let mut row = "..^^.".parse::<Row>().unwrap();

        row = row.gen();
        assert_eq!(row.to_string(), ".^^^^");

        row = row.gen();
        assert_eq!(row.to_string(), "^^..^");
    }

    #[test]
    fn first() {
        let start = input_string(DAY_NAME, ".sample");

        let result = part1(&start, 10);

        assert_eq!(result, 38);
    }
}
