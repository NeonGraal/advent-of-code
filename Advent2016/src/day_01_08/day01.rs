use std::fmt::Debug;
use std::str::FromStr;
use std::num::ParseIntError;

use crate::shared;

pub fn run(suffix: &str) {
    let s = shared::input_string("day01", suffix);
    let s = match s {
        Ok(s) => s,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let result = walk(&s);

    println!("Distance: {}", result);
}

fn walk(s: &str) -> i32 {
    let turns = s.split(", ").map(|s| Turn::from_str(s));

    let mut pos = Pos { dir: Dir::North, x: 0, y: 0 };

    println!("Start: {:?}", pos);

    for turn in turns {
        pos = match turn {
            Ok(t) => pos.turn(t),
            Err(e) => panic!("Problem with turn {:?}", e),
        };
    };

    println!("End: {:?}", pos);

    pos.x.abs() + pos.y.abs()
}

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn turn(&self, turn_left: bool) -> Dir {
        match self {
            Dir::North => if turn_left { Dir::West } else { Dir::East },
            Dir::East => if turn_left { Dir::North } else { Dir::South },
            Dir::South => if turn_left { Dir::East } else { Dir::West },
            Dir::West => if turn_left { Dir::South } else { Dir::North },
        }
    }
}

#[derive(Debug)]
struct Pos {
    dir: Dir,
    x: i32,
    y: i32,
}

impl Pos {
    fn turn(&self, turn: Turn) -> Pos {
        let dir = self.dir.turn(turn.turn_left);
        let x = match dir {
            Dir::West => self.x - turn.walk,
            Dir::East => self.x + turn.walk,
            _ => self.x,
        };
        let y = match dir {
            Dir::North => self.y - turn.walk,
            Dir::South => self.y + turn.walk,
            _ => self.y,
        };
        Pos {dir: dir, x: x, y: y}
    }
}

struct Turn {
    turn_left: bool,
    walk: i32,
}

#[derive(Debug)]enum ParseTurnError {
    InvalidTurn,
    InvalidWalk(ParseIntError),
}

impl FromStr for Turn {
    type Err = ParseTurnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn = match &s[..1] {
            "L" => true,
            "R" => false,
            _ => return Err(ParseTurnError::InvalidTurn),
        };
        let walk = s[1..].parse::<i32>();
        let walk =  match walk {
            Ok(w) => w,
            Err(e) => return Err(ParseTurnError::InvalidWalk(e)),
        };
        Ok(Turn {turn_left: turn, walk: walk})
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::walk;

    #[test]
    fn first() {
        let result = walk("R2, L3");

        assert_eq!(result, 5);
    }

    #[test]
    fn second() {
        let result = walk("R2, R2, R2");

        assert_eq!(result, 2);
    }

    #[test]
    fn third() {
        let result = walk("R5, L5, R5, R3");

        assert_eq!(result, 12);
    }
}