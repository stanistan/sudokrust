#![feature(collections)]
extern crate getopts;
use getopts::{Options};
use std::env;
use std::io::prelude::*;
use std::fs::File;

extern crate sudokrust;
use sudokrust::difficulty::{Difficulty};
use sudokrust::grid::{Grid};
use sudokrust::solver::{solve};

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let options = get_options();
    let matches = match options.parse(args.tail()) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") || matches.opt_present("u") {
        print_usage(&program, options);
        return;
    }

    let grid: Option<Grid> = match matches.opt_present("r") {
        true => match read_file(matches.opt_str("r").unwrap()) {
            Some(grid_string) => Some(Grid::from_string(grid_string)),
            _ => None
        },
        _ => Some(Grid::new())
    };

    if grid.is_none() {
        panic!("Could not generate grid.");
    }

    let mut solved: Grid = match solve(grid.unwrap()) {
        Ok(v) => v,
        _ => panic!("Could not solve puzzle.")
    };

    if matches.opt_present("d") {
        match Difficulty::from_string(matches.opt_str("d").unwrap()) {
            Some(difficulty) => solved.remove_values_for_difficulty(difficulty),
            _ => ()
        };
    }

    if matches.opt_present("w") {
        write_grid(&solved, matches.opt_str("w").unwrap());
        println!("Grid written to file: {}", matches.opt_str("w").unwrap());
    }

    println!("\n\n{}", solved);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn get_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show usage.");
    opts.optflag("u", "usage", "Show usage.");
    opts.optflag("g", "generate", "Generate a new sudoku grid.");
    opts.optopt("d", "difficulty", "Make the generated thing a puzzle", "[easy|medium|hard]");
    opts.optopt("w", "write", "Write the output to", "path/to/file");
    opts.optopt("r", "read", "Read and solve a puzzle.", "path/to/file");
    opts
}

fn read_file(path: String) -> Option<String> {
    let file = File::open(path);
    if file.is_err() {
        return None;
    }

    let mut contents = String::new();
    if file.unwrap().read_to_string(&mut contents).is_err() {
        return None;
    }

    Some(contents)
}

fn write_grid(grid: &Grid, to_path: String) {
    let grid_string = format!("{}", grid);
    let f = File::create(&to_path);
    match f.unwrap().write_all(grid_string.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!(e)
    };
}
