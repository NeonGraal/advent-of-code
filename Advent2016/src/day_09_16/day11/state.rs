use super::equip::Equip;
use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, BinaryHeap, HashMap},
    hash::{Hash, Hasher},
};

#[derive(Debug, Default)]
pub struct State {
    equip: Vec<Equip>,
    floors: HashMap<Equip, usize>,
    elevator: usize,
    pub steps: usize,
    pub score: usize,
    pub taken: Vec<String>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Equal => self.steps.cmp(&other.steps),
            d @ _ => d,
        }
        .reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.score.partial_cmp(&other.score) {
            Some(o) => match o {
                Ordering::Equal => self.steps.partial_cmp(&other.steps),
                _ => Some(o),
            },
            None => None,
        }
        .map(|o| o.reverse())
    }
}

impl Eq for State {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.elevator.hash(state);
        for e in &self.equip {
            self.floors[e].hash(state);
        }
    }
}

impl State {
    pub fn new(lines: &Vec<String>) -> State {
        let mut result = State::default();

        for i in 0..4 {
            result.parse(i, &lines[i]);
        }

        result
    }

    pub fn insert(&mut self, equip: Equip, floor: usize) {
        self.equip.push(equip.clone());
        self.floors.insert(equip, floor);
        self.equip.sort();
        self.score = self.elevator + State::calc_score(self.equip.len(), &self.floors);
    }

    fn parse(&mut self, floor: usize, line: &String) {
        let mut iter = line.split_ascii_whitespace();

        while let Some(w) = iter.next() {
            if w == "a" {
                if let Some(n) = iter.next() {
                    self.insert(n.parse().unwrap(), floor);
                }
            }
        }
        self.equip.sort();
    }

    pub fn show(&self) {
        for i in 0..4 {
            let idx = 3 - i;
            print!(
                "F{} {} ",
                4 - i,
                if self.elevator == idx { "E" } else { "." }
            );
            for e in self.equip.iter() {
                if self.floors[e] == idx {
                    print!(" {}", e);
                } else {
                    print!("  .  ");
                }
            }
            println!();
        }
        println!(" - {}, {}, {:0X}", self.steps, self.score, self.get_hash());
    }

    pub fn get_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    pub fn finished(&self) -> bool {
        self.valid() && self.floors.values().all(|&v| v == 3)
    }

    fn chip_valid(&self, name: &String, floor: usize) -> bool {
        let gen = Equip::Gen(name.to_string());
        if let Some(f) = self.floors.get(&gen) {
            if f == &floor {
                return true;
            }
        }
        self.floors.iter().all(|(e, f)| {
            f != &floor
                || match e {
                    Equip::Gen(n) => n == name,
                    _ => true,
                }
        })
    }

    pub fn valid(&self) -> bool {
        self.equip.iter().all(|e| match e {
            Equip::Chip(n) => self.chip_valid(&n, self.floors[e]),
            _ => true,
        })
    }

    fn steps_to(&self, elevator: usize) -> usize {
        self.steps
            + if self.elevator < elevator {
                elevator - self.elevator
            } else {
                self.elevator - elevator
            }
    }

    pub fn step(&self, elevator: usize, equip: &[&Equip]) -> State {
        let mut floors = self.floors.clone();

        for &f in equip {
            if self.floors[f] != self.elevator {
                panic!(
                    "Can't move {} as not on same floor {} as elevator {}",
                    f, self.floors[f], self.elevator
                );
            }
            floors.insert(f.clone(), elevator);
        }

        let score = elevator + State::calc_score(self.equip.len(), &floors);
        let steps = self.steps_to(elevator);
        let mut taken = self.taken.clone();
        taken.push(format!("-> F{}({}) {:?}", elevator + 1, steps, equip));
        State {
            equip: self.equip.clone(),
            floors,
            elevator,
            steps,
            score,
            taken,
        }
    }

    fn calc_score(len: usize, floors: &HashMap<Equip, usize>) -> usize {
        floors
            .values()
            .map(|&f| len.pow(4 - f as u32))
            .sum()
    }

    pub fn possibles(&self) -> Vec<Self> {
        let mut result = Vec::new();
        let equip: Vec<_> = self
            .equip
            .iter()
            .filter(|&e| self.floors[e] == self.elevator)
            .collect();

        if self.elevator < 3 {
            let elevator = self.elevator + 1;
            for (i, &f) in equip.iter().enumerate() {
                let mut next = self.step(elevator, &[f]);
                if next.valid() {
                    result.push(next);
                }
                for &s in equip.iter().skip(i + 1) {
                    next = self.step(elevator, &[f, s]);
                    if next.valid() {
                        result.push(next);
                    }
                }
            }
        }
        if self.elevator > 0 {
            let elevator = self.elevator - 1;
            for (i, &f) in equip.iter().enumerate() {
                let next = self.step(elevator, &[f]);
                if next.valid() {
                    result.push(next);
                }
            }
        }
        result
    }

    pub fn solve(self, report: usize) -> State {
        let mut search = BinaryHeap::from([self]);
        let mut visited = HashMap::new();

        let mut count = 0;
        let line = 50 * report;
        while let Some(next) = search.pop() {
            if next.finished() {
                println!("! {} ({})", count, visited.len());
                return next;
            }
            count = count + 1;
            if count % report == 0 {
                print!(".");
                if count % line == 0 {
                    println!(" {} ({})", search.len(), visited.len());
                }
            }
            visited.insert(next.get_hash(), next.steps);
            for p in next.possibles() {
                if visited
                    .get(&p.get_hash())
                    .map_or(true, |s| s > &p.steps)
                {
                    search.push(p);
                }
            }
        }

        println!("? {} ({})", count, visited.len());
        State::default()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::{Equip, State};

    #[test]
    fn same_chip_and_gen_on_floor_valid() {
        let al = "Al";
        let mut result = State::default();
        result.insert(Equip::Chip(al.to_string()), 1);
        assert!(result.valid());
        result.insert(Equip::Gen(al.to_string()), 1);
        assert!(result.valid());
        result.insert(Equip::Gen("Li".to_string()), 1);
        result.show();
        assert!(result.valid());
        println!("{:?}", result);
    }

    #[test]
    fn diff_chip_and_gen_on_floor_invalid() {
        let mut result = State::default();
        result.insert(Equip::Chip("Al".to_string()), 1);
        assert!(result.valid());
        result.insert(Equip::Gen("Li".to_string()), 1);
        result.show();
        assert!(!result.valid());
    }

    #[test]
    fn move_diff_chip_and_gen_on_floor_invalid() {
        let gen = Equip::Gen("Li".to_string());
        let mut result = State::default();
        result.insert(Equip::Chip("Al".to_string()), 2);
        assert!(result.valid());
        result.insert(gen.clone(), 1);
        assert!(result.valid());
        result.elevator = 1;
        result.show();
        result = result.step(2, &[&gen]);
        result.show();
        assert!(!result.valid());
    }
}
