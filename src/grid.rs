use std::fmt;
use std::collections::{HashMap,HashSet};

use config::{is_valid, full_range};
use position::{Position,Positions,Regions};

pub trait Validatable {
    fn is_valid(&self) -> bool;
}

impl Validatable for i8 {
    fn is_valid(&self) -> bool {
        is_valid(*self)
    }
}

pub type GridValue = Option<i8>;

impl Validatable for GridValue {
    fn is_valid(&self) -> bool {
        match self {
            &Some(value) => value.is_valid(),
            _ => false
        }
    }
}

pub type GridValues = Vec<GridValue>;

impl Validatable for GridValues {

    fn is_valid(&self) -> bool {

        for value in self {
            if !value.is_valid() {
                return false;
            }
        }

        let mut values = HashSet::new();
        for value in self {
            values.insert(value.unwrap());
        }

        let valid_values: HashSet<_> = full_range().iter().cloned().collect();
        let intersection: HashSet<_> = valid_values.intersection(&values).collect();

        intersection.len() == valid_values.len()
    }

}

#[derive(Debug)]
pub struct Grid {
    map: HashMap<Position,GridValue>
}

impl Grid {

    pub fn new() -> Grid {

        let mut map = HashMap::new();
        for position in Position::all() {
            map.insert(position, None);
        }

        Grid { map: map }
    }

    pub fn insert(&mut self, position: Position, value: GridValue) -> GridValue {
        self.map.insert(position, value);
        value
    }

    pub fn get(&self, position: Position) -> GridValue {
        match self.map.get(&position) {
            Some(&Some(v)) => Some(v),
            _ => None
        }
    }

    pub fn value_at_coordinates(&self, x: i8, y: i8) -> GridValue {
        let position = Position::new(x, y);
        self.get(position)
    }

    pub fn insert_at_coordinates(&mut self, x: i8, y: i8, value: GridValue) -> GridValue {
        let position = Position::new(x, y);
        self.insert(position, value)
    }

    pub fn values(&self, positions: Positions) -> GridValues {
        let mut values = Vec::new();
        for position in positions {
            values.push(self.get(position));
        }
        values
    }

    pub fn values_for_column(&self, x: i8) -> GridValues {
        self.values(Position::for_column(x))
    }

    pub fn values_for_row(&self, y: i8) -> GridValues {
        self.values(Position::for_row(y))
    }

    pub fn values_for_square(&self, n: i8) -> GridValues {
        self.values(Position::for_square(n))
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid.map{:?}", self.map)
    }
}

impl Validatable for Grid {

    fn is_valid(&self) -> bool {

        for i in full_range() {
            if !self.values_for_square(i).is_valid() {
                return false;
            }
            if !self.values_for_row(i).is_valid() {
                return false;
            }
            if !self.values_for_column(i).is_valid() {
                return false;
            }
        }
        true
    }

}