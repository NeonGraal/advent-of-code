use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::shared::pt::Pt;

const DAY_NAME: &str = "day13";

pub fn run(suffix: &str) {
    println!("** Day 13");

    let result = part1(1364, Pt { x: 31, y: 39 });
    println!("- Part1: {}", result);

    let result = part2(1364);
    println!("- Part2: {}", result);
}

fn part1(seed: i32, end: Pt) -> usize {
    let mut acts = BinaryHeap::new();
    let mut visited = HashSet::new();
    let dest = Pt { x: 1, y: 1 };
    acts.push(Act { dest, steps: 0 });

    while let Some(next) = acts.pop() {
        if next.dest == end {
            return next.steps;
        }
        visited.insert(next.dest);

        for p in next.moves() {
            if !(p.is_wall(seed) || visited.contains(&p.dest)) {
                acts.push(p);
            }
        }
    }

    0
}

fn part2(seed: i32) -> usize {
    let mut acts = BinaryHeap::new();
    let mut visited = HashSet::new();
    let dest = Pt { x: 1, y: 1 };
    acts.push(Act { dest, steps: 0 });

    while let Some(next) = acts.pop() {
        visited.insert(next.dest);

        for p in next.moves() {
            if !(p.steps > 50 || p.is_wall(seed) || visited.contains(&p.dest)) {
                acts.push(p);
            }
        }
    }

    visited.len()
}

impl Pt {
    fn wall(&self) -> i32 {
        self.x * (3 + self.x + 2 * self.y) + self.y * (1 + self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
struct Act {
    dest: Pt,
    steps: usize,
}

impl PartialOrd for Act {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.steps.partial_cmp(&self.steps)
    }
}

impl Ord for Act {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl Act {
    fn step(&self, dx: i32, dy: i32) -> Act {
        let dest = self.dest.step(dx, dy);
        let steps = self.steps + 1;
        Act { dest, steps }
    }

    fn moves(&self) -> Vec<Act> {
        let mut result = vec![self.step(1, 0), self.step(0, 1)];

        if self.dest.x > 0 {
            result.push(self.step(-1, 0));
        }
        if self.dest.y > 0 {
            result.push(self.step(0, -1));
        }

        result
    }

    fn is_wall(&self, seed: i32) -> bool {
        let val = self.dest.wall() + seed;
        val.count_ones() % 2 == 1
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{part1, part2, Pt};

    #[test]
    fn first() {
        let result = part1(10, Pt { x: 7, y: 4 });

        assert_eq!(result, 11);
    }
}
