use std::str::FromStr;

use crate::shared::input_lines;

const DAY_NAME: &str = "day08";

pub fn run(suffix: &str) {
    println!("** Day 8");

    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

#[derive(Debug)]
enum Ins {
    Rect { wid: usize, hgt: usize },
    RotRow { row: usize, by: usize },
    RotCol { col: usize, by: usize },
}

impl FromStr for Ins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(&[' ', 'x', '=']).collect();
        let nums: Vec<usize> = parts
            .iter()
            .filter_map(|&n| n.parse::<usize>().ok())
            .collect();

        if parts[0] == "rect" {
            return Ok(Ins::Rect {
                wid: nums[0],
                hgt: nums[1],
            });
        }
        if parts[1] == "column" {
            return Ok(Ins::RotCol {
                col: nums[0],
                by: nums[1],
            });
        }
        if parts[1] == "row" {
            return Ok(Ins::RotRow {
                row: nums[0],
                by: nums[1],
            });
        }
        Err(s.to_string())
    }
}

type Screen = [[bool; 6]; 50];

fn apply(scrn: &mut Screen, ins: Ins) {
    match ins {
        Ins::Rect { wid, hgt } => {
            for x in 0..wid {
                for y in 0..hgt {
                    scrn[x][y] = true;
                }
            }
        }
        Ins::RotRow { row, by } => {
            for i in 0..by {
                let end = scrn[49][row];
                for x in 1..50 {
                    scrn[50 - x][row] = scrn[49 - x][row];
                }
                scrn[0][row] = end;
            }
        }
        Ins::RotCol { col, by } => {
            for i in 0..by {
                let bot = scrn[col][5];
                for y in 1..6 {
                    scrn[col][6 - y] = scrn[col][5 - y];
                }
                scrn[col][0] = bot;
            }
        }
    }
}

fn show(scrn: &Screen) {
    for y in 0..6 {
        for x in 0..50 {
            if scrn[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn part1(lines: &Vec<String>) -> usize {
    let cmds = lines.iter().map(|l| Ins::from_str(l));

    let mut scrn = [[false; 6]; 50];

    for cmd in cmds {
        match cmd {
            Ok(c) => apply(&mut scrn, c),
            Err(e) => panic!("Invalid command {}", e),
        }
    }
    show(&scrn);
    scrn.concat()
        .iter()
        .filter(|&b| *b)
        .count()
}

fn part2(lines: &Vec<String>) -> usize {
    0
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 6);
    }

    #[test]
    fn second() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part2(&lines);

        assert_eq!(result, 0);
    }
}
