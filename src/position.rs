
use std::hash::{Hash, Hasher};
use std::fmt;

use config::{full_range,assert_valid_value};

#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    x: i8,
    y: i8
}

pub type Positions = Vec<Position>;

impl Position {

    pub fn new(x: i8, y: i8) -> Position {
        Position {
            x: assert_valid_value(x),
            y: assert_valid_value(y)
        }
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

    pub fn for_column(x: i8) -> Positions {
        Position::for_xs_ys(vec![x], full_range())
    }

    pub fn for_row(y: i8) -> Positions {
        Position::for_xs_ys(full_range(), vec![y])
    }

    pub fn for_square(n: i8) -> Positions {
        Position::for_xs_ys(
            match n {
                1|4|7 => vec![1, 2, 3],
                2|5|8 => vec![4, 5, 6],
                3|6|9 => vec![7, 8, 9],
                _ => vec![]
            },
            match n {
                1|2|3 => vec![1, 2, 3],
                4|5|6 => vec![4, 5, 6],
                7|8|9 => vec![7, 8, 9],
                _ => vec![]
            }
        )
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
