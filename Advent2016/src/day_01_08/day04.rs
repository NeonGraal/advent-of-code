use std::{collections::BTreeMap, num::ParseIntError, str::FromStr};

use crate::shared::input_parse;

const DAY_NAME: &str = "day04";

pub fn run(suffix: &str) {
    println!("** Day 4");

    let rooms = input_parse(DAY_NAME, suffix);

    let result = part1(&rooms);
    println!("- Part1: {}", result);

    let result = part2(&rooms);
    println!("- Part2: {}", result);
}

fn part1(rooms: &Vec<Room>) -> i32 {
    rooms
        .iter()
        .filter(|r| r.calc_checksum() == r.checksum)
        .map(|r| r.sector)
        .sum()
}

fn part2(rooms: &Vec<Room>) -> i32 {
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
        for c in self
            .name
            .chars()
            .filter(|c| c.is_ascii_lowercase())
        {
            if b.contains_key(&c) {
                b.insert(c, b[&c] + 1);
            } else {
                b.insert(c, 1);
            }
        }

        let mut counts = Vec::from_iter(b);
        counts.sort_by_key(|&(k, v)| (-v, k));
        counts
            .iter()
            .take(5)
            .map(|&(k, v)| k)
            .collect()
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
        self.name
            .chars()
            .map(|c| self.decrypt_char(c))
            .collect()
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
    use super::{input_parse, part1, part2, FromStr, Room, DAY_NAME};

    #[test]
    fn checksum_1() {
        let room: Room = "aaaaa-bbb-z-y-x-123[abxyz]"
            .parse()
            .unwrap();

        println!("{:?}", room);

        let result = room.calc_checksum();

        assert_eq!(result, room.checksum);
    }

    #[test]
    fn first() {
        let lines = input_parse(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 1514);
    }

    #[test]
    fn decrypt_1() {
        let room: Room = "qzmt-zixmtkozy-ivhz-343[zimth]"
            .parse()
            .unwrap();

        println!("{:?}", room);

        let result = room.decrypt();

        assert_eq!(result, "very encrypted name");
    }
}
// cspell:words abxyz qzmt-zixmtkozy-ivhz zimth
