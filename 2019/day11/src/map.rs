use std::collections::BTreeMap;
use std::cmp::{min, max};

use crate::point::Point;

#[derive(Debug)]
pub struct Map {
    pub data: BTreeMap<i32, BTreeMap<i32, char>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    pub fn get(&self, point: &Point) -> Option<char> {
        self.data.get(&point.y).and_then(|chars|
                chars.get(&point.x)).or(Some(&'.')).cloned()
    }

    pub fn set_to_char(&mut self, point: &Point, ch: char) {
        *self.data.entry(point.y).or_insert(BTreeMap::new())
            .entry(point.x).or_insert('.') = ch;
    }

    pub fn empty(&mut self, point: &Point) {
        self.set_to_char(point, '.');
    }

    fn get_bounds(&self) -> (Point, Point) {
        let mut top_left = Point { x: std::i32::MAX, y: std::i32::MAX };
        let mut bottom_right = Point { x: std::i32::MIN, y: std::i32::MIN };

        for (y, row) in self.data.iter() {
            for x in row.keys() {
                top_left.x = min(*x, top_left.x);
                top_left.y = min(*y, top_left.y);

                bottom_right.x = max(*x, bottom_right.x);
                bottom_right.y = max(*y, bottom_right.y);
            }
        }

        (top_left, bottom_right)
    }

    pub fn print(&self) {
        let (top_left, bottom_right) = self.get_bounds();

        for y in top_left.y..bottom_right.y + 1 {
            for x in top_left.x..bottom_right.x + 1 {
                print!("{}", match self.get(&Point { x, y }).unwrap() {
                    '#' => '#',
                    _ => ' ',
                });
            }
            println!("");
        }
    }
}
