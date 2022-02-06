use std::num::ParseIntError;
use std::str::FromStr;
use std::{collections::HashSet, fmt};

use crate::shared::{input_string, pt::Pt};

pub fn run(suffix: &str) {
    let s = input_string("day01", suffix);
    let s = match s {
        Ok(s) => s,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let result = part1(&s);
    println!("Part1 Distance: {}", result);

    let result = part2(&s);
    println!("Part2 Distance: {}", result);
}

fn part1(s: &str) -> i32 {
    let turns = s.split(", ").map(|s| Turn::from_str(s));

    let mut pos = Pos::default();

    println!("Start: {}", pos);

    for turn in turns {
        pos = match turn {
            Ok(t) => pos.turn(t),
            Err(e) => panic!("Problem with turn {:?}", e),
        };
    }

    println!("End: {}", pos);

    pos.curr.len()
}

fn part2(s: &str) -> i32 {
    let turns = s.split(", ").map(|s| Turn::from_str(s));

    let mut pos = Pos::default();
    let mut result = Pt::default();
    let mut points = HashSet::new();

    println!("Start: {}", pos);

    'turns: for turn in turns {
        let w = match turn {
            Ok(t) => pos.walk(t),
            Err(e) => panic!("Problem with turn {:?}", e),
        };
        pos = w.0;
        for p in w.1 {
            if points.contains(&p) {
                result = p;
                break 'turns;
            } else {
                points.insert(p);
            }
        }
    }

    println!("End: {} - {}", pos, result);

    result.len()
}

// 144 is too high

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Default for Dir {
    fn default() -> Self {
        Dir::North
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Dir {
    fn turn(&self, turn_left: bool) -> Dir {
        match self {
            Dir::North => {
                if turn_left {
                    Dir::West
                } else {
                    Dir::East
                }
            }
            Dir::East => {
                if turn_left {
                    Dir::North
                } else {
                    Dir::South
                }
            }
            Dir::South => {
                if turn_left {
                    Dir::East
                } else {
                    Dir::West
                }
            }
            Dir::West => {
                if turn_left {
                    Dir::South
                } else {
                    Dir::North
                }
            }
        }
    }

    fn step(&self) -> Pt {
        let x = match self {
            Dir::West => -1,
            Dir::East => 1,
            _ => 0,
        };
        let y = match self {
            Dir::North => -1,
            Dir::South => 1,
            _ => 0,
        };
        Pt { x, y }
    }
}

#[derive(Debug, Default)]
struct Pos {
    dir: Dir,
    curr: Pt,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.dir, self.curr)
    }
}

impl Pos {
    fn turn(&self, turn: Turn) -> Pos {
        let dir = self.dir.turn(turn.turn_left);
        let step = dir.step();
        let curr = self.curr + (step * turn.walk);
        Pos { dir, curr }
    }

    fn walk(&self, turn: Turn) -> (Pos, Vec<Pt>) {
        let dir = self.dir.turn(turn.turn_left);
        let step = dir.step();
        let curr = self.curr + (step * turn.walk);
        let points: Vec<Pt> = (1..=turn.walk).map(|i| self.curr + step * i).collect();
        (Pos { dir, curr }, points)
    }
}

struct Turn {
    turn_left: bool,
    walk: i32,
}

#[derive(Debug)]
enum ParseTurnError {
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
        let walk = match walk {
            Ok(w) => w,
            Err(e) => return Err(ParseTurnError::InvalidWalk(e)),
        };
        Ok(Turn {
            turn_left: turn,
            walk: walk,
        })
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{part1, part2};

    #[test]
    fn first() {
        let result = part1("R2, L3");

        assert_eq!(result, 5);
    }

    #[test]
    fn second() {
        let result = part1("R2, R2, R2");

        assert_eq!(result, 2);
    }

    #[test]
    fn third() {
        let result = part1("R5, L5, R5, R3");

        assert_eq!(result, 12);
    }

    #[test]
    fn forth() {
        let result = part2("R8, R4, R4, R8");

        assert_eq!(result, 4);
    }
}
