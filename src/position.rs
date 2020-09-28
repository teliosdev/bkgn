#[derive(Debug, Copy, Clone)]
pub struct Position(u32, u32);

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position(x, y)
    }
    pub fn x(&self) -> u32 {
        self.0
    }
    pub fn y(&self) -> u32 {
        self.1
    }
}

impl From<(u32, u32)> for Position {
    fn from((x, y): (u32, u32)) -> Self {
        Position(x, y)
    }
}

pub struct Region(Position, Position);

impl Region {
    pub fn new(top_left: Position, bottom_right: Position) -> Option<Region> {
        if top_left.x() <= bottom_right.x() && top_left.y() <= bottom_right.y() {
            Some(Region(top_left, bottom_right))
        } else {
            None
        }
    }

    pub fn dimensions(&self) -> Position {
        self.bottom_right() - self.top_left()
    }

    pub fn top_left(&self) -> Position {
        self.0
    }
    pub fn bottom_right(&self) -> Position {
        self.1
    }

    pub fn contains(&self, position: Position) -> bool {
        // tlx <= px <= brx, tly <= py <= bry

        (position.x() >= self.0.x() && position.x() < self.1.x())
            && (position.y() >= self.0.y() && position.y() < self.1.y())
    }
}

impl From<(Position, Position)> for Region {
    fn from((tl, br): (Position, Position)) -> Self {
        Region(tl, br)
    }
}

macro_rules! op {
    ($base:ident, $trait:ident, $n:ident) => {
        impl $trait for $base {
            type Output = Self;
            fn $n(self, other: Self) -> Self {
                $base(self.0.$n(other.0), self.1.$n(other.1))
            }
        }

        impl $trait<u32> for $base {
            type Output = Self;
            fn $n(self, other: u32) -> Self {
                $base(self.0.$n(other), self.1.$n(other))
            }
        }
    };
}

use std::ops::{Add, Div, Mul, Rem, Sub};

op!(Position, Add, add);
op!(Position, Mul, mul);
op!(Position, Sub, sub);
op!(Position, Div, div);
op!(Position, Rem, rem);

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: u32, y: u32) -> Position {
        Position::new(x, y)
    }
    fn r(a: Position, b: Position) -> Option<Region> {
        Region::new(a, b)
    }

    #[test]
    fn test_region_bounding() {
        let region = r(p(5, 5), p(15, 15)).unwrap();

        assert!(region.contains(p(5, 5)));
        assert!(region.contains(p(5, 6)));
        assert!(region.contains(p(6, 5)));
        assert!(region.contains(p(6, 6)));
        assert!(region.contains(p(14, 14)));
        assert!(!region.contains(p(15, 14)));
        assert!(!region.contains(p(14, 15)));
        assert!(!region.contains(p(15, 15)));
        assert!(!region.contains(p(5, 15)));
        assert!(!region.contains(p(15, 5)));
        assert!(!region.contains(p(0, 0)));
        assert!(!region.contains(p(4, 4)));
        assert!(!region.contains(p(4, 5)));
        assert!(!region.contains(p(5, 4)));
        assert!(!region.contains(p(4, 15)));
        assert!(!region.contains(p(15, 4)));
        assert!(!region.contains(p(16, 16)));
        assert!(!region.contains(p(16, 5)));
        assert!(!region.contains(p(5, 16)));
    }
}
