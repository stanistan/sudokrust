# sudokrust

A sudoku puzzle solver/generator in Rust.

### Why?

I wanted to build a thing that is both easy and non-trivial in Rust.

This is that thing.

### Usage

There feels to be little reason for me to distribute this thing,
as it's only an exercize on. If you want to use it, you'll have to build
it from source. Luckily, that's super easy right now.

1. Cone it & Build it

    ```
    git clone git@github.com:stanistan/sudokrust.git
    cd sudokrust
    cargo build
    ```

2. Generating puzzles

    ```
    # through cargo
    cargo run -- -g

    # or using the binary directly
    ./target/debug/sudokrust -g
    ```

### Script usage

```
$ path/to/sudokrust --help
Options:
    -h --help           Show usage.
    -u --usage          Show usage.
    -g --generate       Generate a new sudoku grid.
    -d --difficulty [easy|medium|hard]
                        Make the generated thing a puzzle
    -w --write path/to/file
                        Write the output to
    -r --read path/to/file
                        Read and solve a puzzle.
```

*Note:* the difficulty option does not yet work.

I haven't decided how I'm going to be blanking out positions in the grid or what it
means for a difficulty to be difficult in this context.

Puzzles are read from and written in `.ss` sudoku format. (http://www.sudocue.net/fileformats.php)

### The representation of the grid

The grid itself is a hashmap of positions to maybe values.

### The solver

The puzzle solver is pretty naive, we find the positions in the grid which have the fewest
possible choices and pick a number from the remaining options.

I didn't implemenent backing out of incorrect decisions so the script will fail if
the generated grid is invalid.

The fun part is solving an existing puzzle and generating a fresh one is the same process,
generating a fresh one is just solving the empty puzzle.

### Notes on Rust

I found it difficult to get started initially. I'm still not sure where I want to be
passing values/references/pointers- so most of what I'm doing is by value, except for
where the compiler told me that I was wrong, so I fixed it.

I still don't have a very strong intuition for the memory model.
