use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pt {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Pt {
    type Output = Pt;

    fn add(self, rhs: Self) -> Self {
        Pt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Pt {
    type Output = Pt;

    fn mul(self, rhs: i32) -> Self::Output {
        Pt {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Pt {
    pub fn len(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn step(&self, dx: i32, dy: i32) -> Pt {
        let x = self.x as i32 + dx;
        let y = self.y as i32 + dy;
        Pt { x, y }
    }
}
