use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: day_03 input-file");
    } else {
        let input_fn = &args[1];
        run(input_fn);
    }
}

fn run(input_fn: &String) {
    let lines = match File::open(input_fn) {
        Ok(f) => {io::BufReader::new(f).lines()},
        Err(_) => {panic!("Could not open file {}", input_fn)},
    };

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        if let Ok(row) = line {
            let row_chars:Vec<char> = row.chars().collect();
            grid.push(row_chars);
        }
    }

    let results_1x1 = run_slope(&grid, 1, 1);
    let results_1x3 = run_slope(&grid, 1, 3);
    let results_1x5 = run_slope(&grid, 1, 5);
    let results_1x7 = run_slope(&grid, 1, 7);
    let results_2x1 = run_slope(&grid, 2, 1);

    println!("trees = {}", results_1x1 * results_1x3 * results_1x5 * results_1x7 * results_2x1);
}

fn run_slope(grid: &Vec<Vec<char>>, down: usize, right: usize) -> u32 {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;

    while row < grid.len() {
        let row_len = grid[row].len();
        if col >= row_len {
            col = col % row_len;
        }
        if grid[row][col] == '#' {
            trees += 1;
        }
        row += down;
        col += right;
    }

    trees
}
