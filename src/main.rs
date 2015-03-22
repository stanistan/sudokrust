#![feature(collections)]
extern crate getopts;
use getopts::{Options};
use std::env;

extern crate sudokrust;
use sudokrust::difficulty::{Difficulty};
use sudokrust::grid::{Grid};
use sudokrust::solver::{solve};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show usage.");
    opts.optflag("u", "usage", "Show usage.");
    opts.optflag("g", "generate", "Generate a new sudoku grid.");
    opts.optopt("d", "difficulty", "Make the generated thing a puzzle", "[easy|medium|hard]");
    opts.optopt("w", "write", "Write the output to", "path/to/file");
    opts.optopt("r", "read", "Read and solve a puzzle.", "path/to/file");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // is this help, or usage?
    if matches.opt_present("h") || matches.opt_present("u") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("g") {
        let mut solved = match solve(Grid::new()) {
            Ok(v) => v,
            _ => panic!("could not generate puzzle")
        };

        match matches.opt_str("d") {
            Some(v) => match Difficulty::from_string(v) {
                Some(difficulty) => solved.remove_values_for_difficulty(difficulty),
                _ => panic!("invalid difficulty specified")
            },
            _ => ()
        };

        println!("{}", solved);
        return;
    }

}
