use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, &self.y).cmp(&(other.x, &other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    pub fn zero() -> Self {
        Self {
            x: 0, y: 0,
        }
    }

    pub fn min() -> Self {
        Self {
            x: std::i64::MIN,
            y: std::i64::MIN,
        }
    }

    pub fn max() -> Self {
        Self {
            x: std::i64::MAX,
            y: std::i64::MAX,
        }
    }

    pub fn from_tuple(tuple: &(i64, i64)) -> Self {
        Self {x: tuple.0, y: tuple.1}
    }

    pub fn as_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
