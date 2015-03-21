
extern crate sudokrust;

use sudokrust::grid::{Grid,Validatable};
use sudokrust::position::{Position};

fn main() {

    //
    let mut grid = Grid::new();
    println!("value at 1-1 = {:?}", grid.value_at_coordinates(1, 1));

    grid.insert_at_coordinates(1, 1, Some(2));
    println!("value at 1-1 = {:?}", grid.value_at_coordinates(1, 1));

    grid.insert(Position::new(1,1), None);
    println!("value at 1-1 = {:?}", grid.value_at_coordinates(1, 1));

    // we want to have an arbitrary grid with values/
    println!("is this shit valid? {:?}", grid.is_valid());

    let position = Position::new(1,1);
    println!("regions: {:?}", position.relevant_regions());

}
