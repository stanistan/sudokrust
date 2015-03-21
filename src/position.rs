
use std::hash::{Hash, Hasher};
use std::fmt;

use config::{MIN_POS,MAX_POS};

#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    x: i8,
    y: i8
}

impl Position {

    pub fn new(x: i8, y: i8) -> Position {
        Position {
            x: Position::assert_valid_value(x),
            y: Position::assert_valid_value(y)
        }
    }

    fn assert_valid_value(value: i8) -> i8 {
        assert!(value >= MIN_POS);
        assert!(value < MAX_POS);
        value
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
