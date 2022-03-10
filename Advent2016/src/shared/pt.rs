use std::{fmt, ops};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pt {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Pt {
    type Output = Pt;

    fn add(self, rhs: Self) -> Self {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Pt { x, y }
    }
}

impl ops::Sub for Pt {
    type Output = Pt;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Pt { x, y }
    }
}

impl ops::Neg for Pt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        Pt { x, y }
    }
}

impl ops::Mul<i32> for Pt {
    type Output = Pt;

    fn mul(self, rhs: i32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Pt { x, y }
    }
}

impl ops::Div<i32> for Pt {
    type Output = Pt;

    fn div(self, rhs: i32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Pt { x, y }
    }
}

impl ops::BitOr for Pt {
    type Output = i32;

    fn bitor(self, rhs: Self) -> Self::Output {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        dx * dx + dy * dy
    }
}

impl Pt {
    pub fn steps(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn step(&self, dx: i32, dy: i32) -> Pt {
        let x = self.x as i32 + dx;
        let y = self.y as i32 + dy;
        Pt { x, y }
    }

    pub fn negative(&self) -> bool {
        self.x < 0 || self.y < 0
    }

    pub fn beyond(&self, other: &Self) -> bool {
        self.x > other.x || self.y > other.y
    }
}
