
extern crate sudokrust;

use sudokrust::grid::{Grid};
use sudokrust::solver::{solve};

fn main() {

    //
    let grid = Grid::new();
    // grid.insert_at_coordinates(1, 1, Some(2));

    match solve(grid) {
        Ok(v) => println!("\n{}", v),
        _ => println!("unsolved")
    };

}
