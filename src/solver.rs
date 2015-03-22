use std::collections::HashSet;
use std::collections::btree_map::{BTreeMap, Entry};

use config::{full_range};
use grid::{Grid,GridValues};
use position::{Position,Positions};
use traits::{Validatable,AsSet};

#[derive(Debug)]
struct Possibilities {
    position: Position,
    values: HashSet<i8>
}

impl Possibilities {

    pub fn remove_possible_values(&mut self, values: GridValues) {
        for value in values.as_set() {
            if self.values.contains(&value) {
                self.values.remove(&value);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    // gets the first one, we can go random if we really
    // feel like it.
    pub fn pick_value(&self) -> Option<i8> {
        for val in self.values.clone() {
            return Some(val);
        }
        None
    }

}

pub fn solve(grid: Grid) -> Option<Grid> {

    // pretty easy to start with,
    // check if the puzzle is already solved.
    if grid.is_valid() {
        return Some(grid);
    }

    // we need to find the position in the grid with the
    // least amount of possibilities
    let positions = grid.empty_positions();
    let ordered = ordered_positions(positions, grid.clone());
    for (_, possibilities) in ordered.iter() {
        for possibility in possibilities {
            let value = possibility.pick_value();
            if !value.is_none() {
                let mut next_grid: Grid = grid.clone();
                next_grid.insert(possibility.position, value);
                let maybe_solved = solve(next_grid);
                if maybe_solved.is_none() {
                    continue;
                } else {
                    return maybe_solved;
                }
            }
        }
    }
    // println!("Ordered Positions: {}",ordered.len());


    // if we cant solve it, or something.
    None
}

fn ordered_positions(positions: Positions, grid: Grid) -> BTreeMap<usize,Vec<Possibilities>> {

    let mut map = BTreeMap::new();
    let full_range_values = full_range().as_set();

    for position in positions {

        let mut possibilities = Possibilities {
            position: position,
            values: full_range_values.clone()
        };

        let regions = possibilities.position.relevant_regions();
        for region in regions {
            possibilities.remove_possible_values(grid.values(region));
        }

        let mut possibilities_at_len = match map.entry(possibilities.len()) {
            Entry::Vacant(entry) => entry.insert(Vec::new()),
            Entry::Occupied(entry) => entry.into_mut()
        };

        possibilities_at_len.push(possibilities);
    }
    map
}