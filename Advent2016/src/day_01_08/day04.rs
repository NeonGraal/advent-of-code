use std::{collections::BTreeMap, num::ParseIntError, str::FromStr};

use crate::shared::input_lines;

const DAY_NAME: &str = "day04";

pub fn run(suffix: &str) {
    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("Part1: {}", result);

    let result = part2(&lines);
    println!("Part2: {}", result);
}

fn part1(lines: &Vec<String>) -> i32 {
    let rooms = lines
        .into_iter()
        .filter_map(|r| Room::from_str(&r).ok())
        .collect::<Vec<Room>>();

    rooms
        .iter()
        .filter(|r| r.calc_checksum() == r.checksum)
        .map(|r| r.sector)
        .sum()
}

fn part2(lines: &Vec<String>) -> i32 {
    let rooms = lines
        .into_iter()
        .filter_map(|r| Room::from_str(&r).ok())
        .collect::<Vec<Room>>();

    let room = rooms
        .iter()
        .filter(|r| r.calc_checksum() == r.checksum)
        .filter(|r| r.decrypt().contains("north"))
        .next()
        .unwrap();

    println!("{:?} - {}", room, room.decrypt());

    room.sector
}

#[derive(Debug)]
struct Room {
    name: String,
    sector: i32,
    pub checksum: String,
}

impl Room {
    pub fn calc_checksum(&self) -> String {
        let mut b = BTreeMap::<char, i32>::new();
        for c in self.name.chars().filter(|c| c.is_ascii_lowercase()) {
            if b.contains_key(&c) {
                b.insert(c, b[&c] + 1);
            } else {
                b.insert(c, 1);
            }
        }

        let mut counts = Vec::from_iter(b);
        counts.sort_by_key(|&(k, v)| (-v, k));
        counts.iter().take(5).map(|&(k, v)| k).collect()
    }

    fn sector_if_real(&self) -> i32 {
        if self.calc_checksum() == self.checksum {
            self.sector
        } else {
            0
        }
    }

    fn decrypt_char(&self, c: char) -> char {
        if c == '-' {
            return ' ';
        }
        ((c as i32 - 'a' as i32 + self.sector) % 26 + 'a' as i32) as u8 as char
    }

    pub fn decrypt(&self) -> String {
        self.name.chars().map(|c| self.decrypt_char(c)).collect()
    }
}

impl FromStr for Room {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(['-', '[', ']']).collect();
        let len = parts.len();
        let name = parts[..len - 3].join("-");
        let sector = parts[len - 3].parse::<i32>()?;
        let checksum = parts[len - 2].to_string();
        Ok(Room {
            name,
            sector,
            checksum,
        })
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2, FromStr, Room, DAY_NAME};

    #[test]
    fn checksum_1() {
        let room = Room::from_str("aaaaa-bbb-z-y-x-123[abxyz]").unwrap();

        println!("{:?}", room);

        let result = room.calc_checksum();

        assert_eq!(result, room.checksum);
    }

    #[test]
    fn first() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 1514);
    }

    #[test]
    fn decrypt_1() {
        let room = Room::from_str("qzmt-zixmtkozy-ivhz-343[zimth]").unwrap();

        println!("{:?}", room);

        let result = room.decrypt();

        assert_eq!(result, "very encrypted name");
    }
}
