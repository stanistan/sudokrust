use std::collections::{HashSet};
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
            self.remove_possible_value(value);
        }
    }

    pub fn remove_possible_value(&mut self, value: i8) {
        if self.values.contains(&value) {
            self.values.remove(&value);
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

}

#[derive(Debug)]
pub enum SolverError {
    Contradiction,
    OutOfOptions,
    Unknown
}

pub fn solve(grid: Grid) -> Result<Grid,SolverError> {
    solve_grid(grid, 0)
}

fn solve_grid(grid: Grid, level: usize) -> Result<Grid,SolverError> {

    if grid.is_valid() {
        return Ok(grid);
    }

    if grid.is_full() {
        return Err(SolverError::Unknown);
    }

    let ordered = ordered_positions(grid.empty_positions(), grid.clone());
    if ordered.is_none() {
        return Err(SolverError::Contradiction);
    }

    let unwrapped_ordered = ordered.unwrap();
    let possible_values = unwrapped_ordered.values();

    for possibilities in possible_values {
        for possibility in possibilities {
            let position: Position = possibility.position;
            for value in possibility.values.iter() {

                let mut next_grid = grid.clone();
                next_grid.insert(position, Some(*value));

                match solve_grid(next_grid, level + 1) {
                    Ok(grid) => return Ok(grid),
                    Err(SolverError::Unknown) => return Err(SolverError::Unknown),
                    Err(SolverError::OutOfOptions) => match level {
                        0 => continue,
                        _ => return Err(SolverError::OutOfOptions)
                    },
                    _ => continue
                };
            }
        }
    }

    Err(SolverError::OutOfOptions)
}

fn ordered_positions(positions: Positions, grid: Grid) -> Option<BTreeMap<usize,Vec<Possibilities>>> {

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

        // this means we've hit a contradictory situtation
        if possibilities.len() == 0 {
            return None;
        }

        let mut possibilities_at_len = match map.entry(possibilities.len()) {
            Entry::Vacant(entry) => entry.insert(Vec::new()),
            Entry::Occupied(entry) => entry.into_mut()
        };

        possibilities_at_len.push(possibilities);
    }

    Some(map)
}
