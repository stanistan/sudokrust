use std::hash::{Hash, Hasher};
use std::fmt;

use config::{full_range};
use traits::{Validatable,are_many_valid};

#[derive(PartialEq, Eq, Debug, Copy)]
pub struct Position {
    x: i8,
    y: i8
}

pub type Positions = Vec<Position>;

pub type Regions = Vec<Positions>;

impl Position {

    pub fn new(x: i8, y: i8) -> Position {
        Position { x: x, y: y }
    }

    pub fn for_xs_ys(xs: Vec<i8>, ys: Vec<i8>) -> Positions {
        let mut positions = Vec::new();
        for x in xs.iter() {
            for y in ys.iter() {
                positions.push(Position::new(x.clone(), y.clone()));
            }
        }
        positions
    }

    pub fn all() -> Positions {
        Position::for_xs_ys(full_range(), full_range())
    }

    pub fn for_column(column: i8) -> Positions {
        Position::for_xs_ys(vec![column], full_range())
    }

    pub fn column(&self) -> i8{
        self.x
    }

    pub fn for_row(row: i8) -> Positions {
        Position::for_xs_ys(full_range(), vec![row])
    }

    pub fn row(&self) -> i8 {
        self.y
    }

    pub fn for_square(square: i8) -> Positions {
        Position::for_xs_ys(
            match square {
                1|4|7 => vec![1, 2, 3],
                2|5|8 => vec![4, 5, 6],
                3|6|9 => vec![7, 8, 9],
                _ => unreachable!()
            },
            match square {
                1|2|3 => vec![1, 2, 3],
                4|5|6 => vec![4, 5, 6],
                7|8|9 => vec![7, 8, 9],
                _ => unreachable!()
            }
        )
    }

    pub fn square(&self) -> i8 {
        for i in full_range() {
            for position in Position::for_square(i) {
                if position == *self {
                    return i
                }
            }
        }
        unreachable!()
    }

    // we need a bunch of regions to know how to generate
    // a grid, and validate its contents
    pub fn relevant_regions(&self) -> Regions {
        vec![
            Position::for_column(self.column()),
            Position::for_row(self.row()),
            Position::for_square(self.square())
        ]
    }

}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position(x: {}, y: {})", self.x, self.y)
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Validatable for Position {
    fn is_valid(&self) -> bool {
        self.x.is_valid() && self.y.is_valid()
    }
}

impl Validatable for Positions {
    fn is_valid(&self) -> bool {
        are_many_valid(self)
    }
}

impl Validatable for Regions {
    fn is_valid(&self) -> bool {
        are_many_valid(self)
    }
}
