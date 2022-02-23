use std::{num::ParseIntError, str::FromStr};

use crate::shared::input_parse;

const DAY_NAME: &str = "day15";

pub fn run(suffix: &str) {
    println!("** Day 15");

    let mut disks = input_parse(DAY_NAME, suffix);

    let result = part1(&disks);
    println!("- Part1: {}", result);

    let result = part2(&mut disks);
    println!("- Part2: {}", result);
}

fn check_time(disks: &Vec<Disk>, time: usize) -> bool {
    disks
        .iter()
        .all(|d| (d.id + time + d.start) % d.num == 0)
}

fn part1(disks: &Vec<Disk>) -> usize {
    let mut t = 0;

    loop {
        if check_time(disks, t) {
            return t;
        }
        t = t + 1;
    }
}

fn part2(disks: &mut Vec<Disk>) -> usize {
    disks.push(Disk {
        id: disks.len()+1,
        num: 11,
        start: 0,
    });

    let mut t = 0;

    loop {
        if check_time(disks, t) {
            return t;
        }
        t = t + 1;
    }
}

#[derive(Debug, Clone, Copy)]
struct Disk {
    id: usize,
    num: usize,
    start: usize,
}

impl FromStr for Disk {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(&[' ', '#', '=', ',', '.'])
            .filter_map(|w| w.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let id = nums[0];
        let num = nums[1];
        let start = nums[3];

        Ok(Disk { id, num, start })
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_parse, part1, part2, Disk, DAY_NAME};

    #[test]
    fn parse_disk() {
        let disk = "Disc #2 has 2 positions; at time=0, it is at position 1."
            .parse::<Disk>()
            .unwrap();

        assert_eq!(disk.id, 2);
        assert_eq!(disk.num, 2);
        assert_eq!(disk.start, 1);
    }

    #[test]
    fn first() {
        let lines = input_parse(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 5);
    }

    #[test]
    fn second() {
        let mut lines = input_parse(DAY_NAME, ".sample");

        let result = part2(&mut lines);
        println!("{:?}", lines);

        assert_eq!(result, 85);
    }
}
