
extern crate sudokrust;

use sudokrust::traits::{Validatable};
use sudokrust::grid::{Grid};
use sudokrust::position::{Position};
use sudokrust::solver::{solve};

fn main() {

    //
    let mut grid = Grid::new();
    println!("value at 1-1 = {:?}", grid.value_at_coordinates(1, 1));

    grid.insert_at_coordinates(1, 1, Some(2));
    println!("value at 1-1 = {:?}", grid.value_at_coordinates(1, 1));

    // grid.insert(Position::new(1,1), None);
    // println!("value at 1-1 = {:?}", grid.value_at_coordinates(1, 1));

    // we want to have an arbitrary grid with values/
    println!("is this shit valid? {:?}", grid.is_valid());

    let position = Position::new(1,1);
    println!("regions: {:?}", position.relevant_regions());

    match solve(grid) {
        Some(v) => println!("{}", v),
        _ => println!("unsolved")
    };

}
