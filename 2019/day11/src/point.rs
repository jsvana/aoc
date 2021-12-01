use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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
    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn reflect_x(&self) -> Point {
        Point {
            x: -self.x,
            y: self.y,
        }
    }

    pub fn reflect_y(&self) -> Point {
        Point {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn reflect(&self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn angle_to(&self, other: &Point) -> f64 {
        let delta_x = (other.x - self.x) as f64;
        let delta_y = (other.y - self.y) as f64;

        //(delta_y / delta_x).atan()

        delta_y.atan2(delta_x)
    }
}
