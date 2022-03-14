use std::{collections::HashSet, fmt};

use crate::shared::pt::Pt;

#[derive(Debug)]
pub struct Path {
    pub end: u32,
    pub moves: HashSet<Pt>,
    pub m_len: usize,
    visited: HashSet<u32>,
    pub v_len: usize,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end && self.moves == other.moves && self.visited == other.visited
    }
}

impl Eq for Path {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.m_len.partial_cmp(&self.m_len)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.m_len.cmp(&self.m_len)
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            end: 0,
            moves: HashSet::new(),
            m_len: 0,
            visited: HashSet::from_iter([0]),
            v_len: 0,
        }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0 -> {} ({}) [{:?}]", self.end, self.m_len, self.visited)
    }
}

impl Path {
    pub fn can_add(&self, from: u32, to: u32) -> bool {
        self.end == from && !self.visited.contains(&to)
    }

    pub fn add(&self, from: u32, to: u32, cost: &HashSet<Pt>) -> Self {
        let visited = &self.visited | &HashSet::from_iter([to]);
        let m_len = self.m_len + cost.len();
        let moves = &self.moves | cost;
        Path {
            end: to,
            v_len: visited.len(),
            m_len,
            moves,
            visited,
        }
    }
}