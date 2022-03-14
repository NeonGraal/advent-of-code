use std::collections::HashSet;

use crate::shared::pt::Pt;

use super::Map;

pub struct Walk {
    pub start: u32,
    pub curr: Pt,
    pub moves: HashSet<Pt>,
    len: usize,
    pub finished: Option<u32>,
}

impl PartialEq for Walk {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
            && self.curr == other.curr
            && self.moves == other.moves
            && self.finished == other.finished
    }
}

impl Eq for Walk {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Walk {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.len.partial_cmp(&self.len)
    }
}

impl Ord for Walk {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}

impl Walk {
    pub fn new(curr: Pt, start: u32) -> Walk {
        Walk {
            start,
            curr,
            moves: HashSet::new(),
            len: 0,
            finished: None,
        }
    }

    fn possible(&self, map: &Map, dx: i32, dy: i32) -> Option<Walk> {
        let moves = &self.moves | &HashSet::from([self.curr]);
        let curr = self.curr.step(dx, dy);
        if !map.valid(self.start, curr) {
            return None;
        }
        let finished = map.loc(&curr);
        if let Some(n) = finished {
            if n == self.start {
                return None;
            }
        }
        Some(Walk {
            start: self.start,
            len: moves.len(),
            curr,
            moves,
            finished,
        })
    }

    pub fn possibles(&self, map: &Map) -> Vec<Walk> {
        let mut results = Vec::new();

        if let Some(s) = self.possible(map, 0, 1) {
            results.push(s);
        }
        if let Some(s) = self.possible(map, 1, 0) {
            results.push(s);
        }
        if let Some(s) = self.possible(map, 0, -1) {
            results.push(s);
        }
        if let Some(s) = self.possible(map, -1, 0) {
            results.push(s);
        }

        results
    }
}
