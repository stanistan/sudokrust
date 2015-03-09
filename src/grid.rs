
use std::fmt;
use std::collections::HashMap;

// extern crate sudokrust;

use position::{Position};

#[derive(Debug)]
pub struct Grid {
    map: HashMap<Position,Option<i8>>
}

impl Grid {

    pub fn new() -> Grid {

        let mut map = HashMap::new();
        for x in 1..10 {
            for y in 1..10 {
                let position = Position::new(x, y);
                map.insert(position, None);
            }
        }

        Grid { map: map }
    }

    pub fn insert(&mut self, position: Position, value: Option<i8>) -> Option<i8> {
        self.map.insert(position, value);
        value
    }

    pub fn get(&self, position: Position) -> Option<i8> {
        match self.map.get(&position) {
            Some(&Some(v)) => Some(v),
            _ => None
        }
    }

    pub fn value_at_coordinates(&self, x: i8, y: i8) -> Option<i8> {
        let position = Position::new(x, y);
        self.get(position)
    }

    pub fn insert_at_coordinates(&mut self, x: i8, y: i8, value: Option<i8>) -> Option<i8> {
        let position = Position::new(x, y);
        self.insert(position, value)
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid.map{:?}", self.map)
    }
}
