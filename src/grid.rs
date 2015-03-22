use std::fmt;
use std::collections::{HashMap,HashSet};

use config::{full_range};
use traits::{AsSet,Validatable,are_many_valid,are_sets_equal};
use position::{Position,Positions};
use difficulty::{Difficulty};

pub type GridValue = Option<i8>;

impl Validatable for GridValue {
    fn is_valid(&self) -> bool {
        match self {
            &Some(value) => value.is_valid(),
            _ => false
        }
    }
}

impl AsSet for GridValue {
    fn as_set(&self) -> HashSet<i8> {
        match self {
            &Some(value) => value.as_set(),
            _ => HashSet::new()
        }
    }
}

pub type GridValues = Vec<GridValue>;

impl Validatable for GridValues {
    fn is_valid(&self) -> bool {
        if are_many_valid(self) {
            let full_range_values = full_range();
            return are_sets_equal(&full_range_values.as_set(), &self.as_set());
        }
        false
    }
}

impl AsSet for GridValues {
    fn as_set(&self) -> HashSet<i8> {
        let mut set = HashSet::new();
        for value in self {
            if value.is_some() {
                set.insert(value.unwrap());
            }
        }
        set
    }
}

#[derive(Debug)]
pub struct Grid {
    map: HashMap<Position,GridValue>
}

impl Clone for Grid {

    fn clone(&self) -> Grid {
        let mut map = HashMap::new();
        for position in Position::all() {
            map.insert(position, self.get(position));
        }
        Grid { map: map }
    }

    fn clone_from(&mut self, source: &Grid) {
        let mut map = HashMap::new();
        for position in Position::all() {
            map.insert(position, source.get(position));
        }
        self.map = map;
    }

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

    pub fn empty_positions(&self) -> Positions {
        let mut positions = Vec::new();
        for position in Position::all() {
            if self.get(position).is_none() {
                positions.push(position);
            }
        }
        positions
    }

    pub fn is_full(&self) -> bool {
        self.empty_positions().len() == 0
    }

    pub fn remove_values_for_difficulty(&mut self, difficulty: Difficulty) {

        let mut num_to_remove = difficulty.num_to_remove();
        while num_to_remove > 0 {
            num_to_remove -= 1;
        }
    }

}

// I hate everything about this.
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut last_y = 1;
        for y in 1..10 {
            if y != last_y {
                try!(write!(f, "\n"));
                if y % 3 == 1 && y > 1 {
                    try!(write!(f, "-----------\n"));
                }
                last_y = y
            }
            for x in 1..10 {
                if x % 3 == 1 && x > 1 {
                    try!(write!(f, "|"));
                }
                let value = self.value_at_coordinates(x, y);
                if value.is_none() {
                    try!(write!(f, "."));
                } else {
                    try!(write!(f, "{}", value.unwrap()));
                }
            }
        }
        Ok(())
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
