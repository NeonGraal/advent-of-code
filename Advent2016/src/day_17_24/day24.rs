use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::shared::{input_lines, pt::Pt};

const DAY_NAME: &str = "day24";

pub fn run(suffix: &str) {
    println!("** Day 24");

    let lines = input_lines(DAY_NAME, suffix);

    let result = part1(&lines);
    println!("- Part1: {}", result);
    println!("- Part1: 464 is too high, 383 too low ");

    let result = part2(&lines);
    println!("- Part2: {}", result);
}

fn part1(lines: &Vec<String>) -> usize {
    let mut map = Map::from(lines);
    map.show(map.show_basic());

    map.walk();
    map.show_costs();

    let p = map.path1();
    map.show(map.show_path(&p.moves));
    println!("{}", p);

    p.m_len
}

fn part2(lines: &Vec<String>) -> usize {
    let mut map = Map::from(lines);
    map.show(map.show_basic());

    map.walk();
    map.show_costs();

    let p = map.path2();
    map.show(map.show_path(&p.moves));
    println!("{}", p);

    p.m_len
}

pub struct Map {
    open: HashSet<Pt>,
    max: Pt,
    locs: HashMap<Pt, u32>,
    visited: HashSet<(u32, Pt)>,
    costs: HashMap<(u32, u32), HashSet<Pt>>,
}

mod path;
mod walk;

impl Map {
    fn from(lines: &Vec<String>) -> Map {
        let (x, y) = (lines[0].len() as i32, lines.len() as i32);
        let max = Pt { x, y };
        let mut open = HashSet::new();
        let mut locs = HashMap::new();

        for (i, l) in lines.iter().enumerate() {
            for (j, c) in l.chars().enumerate() {
                let (x, y) = (j as i32, i as i32);
                let pt = Pt { x, y };
                if let Some(n) = c.to_digit(10) {
                    locs.insert(pt, n);
                    open.insert(pt);
                } else if c == '.' {
                    open.insert(pt);
                }
            }
        }

        Map {
            open,
            max,
            locs,
            visited: HashSet::new(),
            costs: HashMap::new(),
        }
    }

    fn valid(&self, start: u32, curr: Pt) -> bool {
        self.open.contains(&curr)
    }

    fn loc(&self, curr: &Pt) -> Option<u32> {
        self.locs.get(curr).map(|&v| v)
    }

    fn show_basic(&self) -> impl Fn(&Pt) -> char + '_ {
        move |pt| if self.open.contains(pt) { '.' } else { ' ' }
    }

    fn show_path<'a>(&'a self, path: &'a HashSet<Pt>) -> impl Fn(&Pt) -> char + 'a {
        move |pt| {
            if path.contains(pt) {
                '*'
            } else if self.open.contains(pt) {
                '.'
            } else {
                ' '
            }
        }
    }

    fn show<F>(&self, show: F)
    where
        F: Fn(&Pt) -> char,
    {
        for y in 0..self.max.y {
            for x in 0..self.max.x {
                let pt = Pt { x, y };
                if let Some(n) = self.locs.get(&pt) {
                    print!("{}", n);
                } else {
                    let c = show(&pt);
                    print!("{}", c);
                }
            }
            println!("");
        }
        println!("");
    }

    fn walk(&mut self) {
        let mut states = BinaryHeap::from_iter(
            self.locs
                .iter()
                .map(|(&curr, &start)| walk::Walk::new(curr, start)),
        );

        while let Some(curr) = states.pop() {
            if let Some(n) = curr.finished {
                self.insert(curr.start, n, &curr.moves);
            }
            let visit = (curr.start, curr.curr);
            if !self.visited.contains(&visit) {
                self.visited.insert(visit);
                for p in curr.possibles(&self) {
                    states.push(p);
                }
            }
        }
    }

    fn insert(&mut self, from: u32, to: u32, cost: &HashSet<Pt>) {
        let k = (from, to);
        if let Some(d) = self.costs.get(&k) {
            if cost.len() >= d.len() {
                return;
            }
        }
        self.costs.insert(k, cost.clone());
    }

    fn show_costs(&self) {
        let len = self.locs.len();

        print!("  |");
        for t in 0..len as u32 {
            print!("    {}  |", t);
        }
        for f in 0..len as u32 {
            print!("\n{} |", f);
            for t in 0..len as u32 {
                let k = (f, t);
                if let Some(c) = self.costs.get(&k) {
                    print!("   {:3} |", c.len());
                } else {
                    print!("       |");
                }
            }
        }
        println!();
    }

    fn path1(&self) -> path::Path {
        let mut paths = BinaryHeap::from_iter([path::Path::default()]);
        let v_len = self.locs.len();

        while let Some(curr) = paths.pop() {
            if v_len == curr.v_len {
                return curr;
            }
            for (&(f, t), c) in &self.costs {
                if curr.can_add(f, t) {
                    let p = curr.add(f, t, c);
                    if p.m_len < 500 {
                        paths.push(p);
                    }
                }
            }
        }

        path::Path::default()
    }

    fn path2(&self) -> path::Path {
        let mut paths = BinaryHeap::from_iter([path::Path::default()]);
        let v_len = self.locs.len();

        while let Some(curr) = paths.pop() {
            if v_len == curr.v_len {
                if curr.end == 0 {
                    return curr;
                }
                if let Some(c) = self.costs.get(&(curr.end, 0)) {
                    let p = curr.add(curr.end, 0, c);
                    if p.m_len < 800 {
                        paths.push(p);
                    }
                }
            }
            for (&(f, t), c) in &self.costs {
                if curr.can_add(f, t) {
                    let p = curr.add(f, t, c);
                    if p.m_len < 800 {
                        paths.push(p);
                    }
                }
            }
        }

        path::Path::default()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{input_lines, part1, part2, DAY_NAME};

    #[test]
    fn first() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part1(&lines);

        assert_eq!(result, 14);
    }

    #[test]
    fn second() {
        let lines = input_lines(DAY_NAME, ".sample");

        let result = part2(&lines);

        assert_eq!(result, 20);
    }
}
