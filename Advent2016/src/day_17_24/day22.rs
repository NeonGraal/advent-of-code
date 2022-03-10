use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
    num::ParseIntError,
    str::FromStr,
};

use crate::shared::{input_parse, pt::Pt};

const DAY_NAME: &str = "day22";

pub fn run(suffix: &str) {
    println!("** Day 22");

    let nodes = input_parse(DAY_NAME, suffix);

    let result = part1(&nodes);
    println!("- Part1: {}", result);

    let result = part2(&nodes);
    println!("- Part2: {}", result);
}

fn part1(nodes: &Vec<Node>) -> usize {
    nodes
        .iter()
        .filter(|&n| n.used > 0)
        .map(|a| {
            nodes
                .iter()
                .filter(|&b| a.name != b.name && a.used <= b.avail)
                .count()
        })
        .sum()
}

fn part2(nodes: &Vec<Node>) -> i32 {
    let start = State::from(nodes);
    println!("{}", start);

    let mut search = BinaryHeap::new();
    search.push(start);

    while let Some(curr) = search.pop() {
        if curr.finished() {
            println!("{}", curr);
            return curr.moves;
        }
        for p in curr.possible() {
            search.push(p);
        }
    }

    0
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    name: String,
    size: usize,
    used: usize,
    avail: usize,
}

impl FromStr for Node {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(&[' ', 'T', '%']);
        let name = parts.next().unwrap().to_string();
        if !name.starts_with("/dev/") {
            "a".parse::<usize>()?;
        }
        let mut nums = parts.filter_map(|p| p.parse::<usize>().ok());
        let size = nums.next().unwrap();
        let used = nums.next().unwrap();
        let avail = nums.next().unwrap();

        Ok(Node {
            name,
            size,
            used,
            avail,
        })
    }
}

impl Node {
    fn to_point(&self) -> Pt {
        let mut nums = self
            .name
            .split(&['-', 'x', 'y'])
            .filter_map(|p| p.parse::<i32>().ok());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        Pt { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    points: HashMap<Pt, Grid>,
    visited: HashSet<(Pt, Pt)>, 
    max_avail: usize,
    goal: Pt,
    empty: Pt,
    end: Pt,
    moves: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Grid {
    size: usize,
    used: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.score().partial_cmp(&other.score())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.end.y {
            for x in 0..=self.end.x {
                let pt = Pt { x, y };
                let c = if pt == self.goal {
                    "G"
                } else if pt == self.empty {
                    "_"
                } else {
                    let g = &self.points[&pt];
                    if g.used > self.max_avail {
                        "#"
                    } else {
                        "."
                    }
                };
                write!(f, "{} ", c)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "+ {} # {}", self.moves, self.score())
    }
}

impl State {
    fn from(nodes: &Vec<Node>) -> State {
        let mut points = HashMap::new();
        let visited = HashSet::new();
        let mut goal = Pt::default();
        let mut end = Pt::default();
        let mut empty = Pt::default();
        let mut max_avail = 0_usize;

        for n in nodes {
            let pt = n.to_point();
            if n.avail == n.size {
                max_avail = n.avail;
                empty = pt.clone();
            }
            if pt.y == 0 && pt.x > goal.x {
                goal.x = pt.x;
            }
            if pt > end {
                end = pt;
            }
            points.insert(
                pt,
                Grid {
                    size: n.size,
                    used: n.used,
                },
            );
        }

        let moves = 0;

        State {
            points,
            visited,
            max_avail,
            goal,
            empty,
            end,
            moves,
        }
    }

    fn finished(&self) -> bool {
        self.goal.x == 0
    }

    fn score(&self) -> i32 {
        self.moves - self.goal.x
    }

    fn try_move(&self, dx: i32, dy: i32) -> Option<State> { 

        let x = self.empty.x + dx;
        let y = self.empty.y + dy;
        let next = Pt { x, y };
        let goal = if self.goal == next { self.empty } else { self.goal };
        let visit = (goal, next);
        if next.negative() || next.beyond(&self.end) || self.visited.contains(&visit) {
            return None;
        }
        let points = self.points.clone();
        let mut next_grid = points[&next];
        let mut empty_grid = points[&self.empty];
        if next_grid.used > empty_grid.size {
            return None;
        }
        empty_grid.used = next_grid.used;
        next_grid.used = 0;
        let mut visited = self.visited.clone();
        visited.insert(visit);
        Some(State {
            points,
            visited,
            max_avail: self.max_avail,
            goal,
            empty: next,
            end: self.end,
            moves: self.moves + 1,
        })
    }

    fn possible(&self) -> Vec<State> {
        let mut result = Vec::new();

        if self.empty.x < self.goal.x {
            if let Some(up) = self.try_move(0, -1) {
                return vec![up];
            }
            if let Some(left) = self.try_move(-1, 0) {
                result.push(left);
            }
            if let Some(right) = self.try_move(1, 0) {
                result.push(right);
            }
        } else {
            if let Some(left) = self.try_move(-1, 0) {
                return vec![left];
            }
            if let Some(down) = self.try_move(0, 1) {
                return vec![down];
            }
            if let Some(right) = self.try_move(1, 0) {
                result.push(right);
            }
        }

        return result;
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_parse, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        let nodes = input_parse(DAY_NAME, ".sample");

        for n in &nodes {
            println!("{:?}", n);
        }

        let result = part1(&nodes);

        assert_eq!(result, 7);
    }

    #[test]
    fn second() {
        let nodes = input_parse(DAY_NAME, ".sample");

        let result = part2(&nodes);

        assert_eq!(result, 7);
    }
}
