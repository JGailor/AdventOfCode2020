use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: day_2 input_file");
    }

    println!("{:?}", args);
    let input_fn = args[1].clone();

    process_from_file(input_fn, process_entry);
}

fn process_from_file(fname: String, f: fn(String) -> bool) -> u32 {
    let lines = match File::open(fname.clone()) {
        Ok(f) => {io::BufReader::new(f).lines()},
        Err(_) => {panic!("Could not open file {}", fname)}
    };
    let mut successes = 0;
    let mut failures = 0;
    let mut total = 0;

    for line in lines {
        if let Ok(entry) = line {
            if f(entry) == true {
                successes += 1;
            } else {
                failures += 1;
            }
            total += 1;
        }
    }
    println!("Total number of entries: {}", total);
    println!("Total number of successes: {}", successes);
    println!("Total number of failures: {}", failures);
    
    successes
}

fn process_entry(_entry: String) -> bool {
    true
}
