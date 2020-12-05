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

fn process_entry(entry: String) -> bool {
    let preamble_body: Vec<&str> = entry.split(": ").collect();
    if preamble_body.len() != 2 {
        panic!("Could not parse {}", entry);
    }
    let preamble = preamble_body[0].clone();
    let password = preamble_body[1].clone();
    let min_max_char:Vec<&str> = preamble.split(' ').collect();
    if min_max_char.len() != 2 {
        panic!("Could not parse preamble {}", preamble);
    }

    let character:char = match min_max_char[1].clone().parse() {
        Ok(c) => {c},
        Err(_) => {panic!("Could not extract the target char from {}", min_max_char[1])}
    };
    let min_max:Vec<&str> = min_max_char[0].clone().split('-').collect();    
    if min_max.len() != 2 {
        panic!("Could not parse min_max {:?}", min_max_char);
    }

    let min:u32 = match min_max[0].parse() {
        Ok(n) => {n},
        Err(_) => {panic!("Could not parse {} to a u32", min_max[0])}
    };

    let max:u32 = match min_max[1].parse() {
        Ok(n) => {n},
        Err(_) => {panic!("Could not parse {} to a u32", min_max[1])}
    };

    v2(min, max, character, password)
}

fn v1(min: u32, max: u32, character: char, password: &str) -> bool {
    let mut matches = 0;
    for pw_character in password.chars() {
        if pw_character == character {
            matches += 1;
        }
    }

    println!("Min: {}, Max: {}, Character: {} -> {}", min, max, character, password);

    matches >= min && matches <= max    
}

fn v2(first_pos: u32, second_pos: u32, character: char, password: &str) -> bool {
    let pw_chars: Vec<char> = password.chars().collect();

    let first_match = pw_chars[(first_pos - 1) as usize] == character;
    let second_match = pw_chars[(second_pos - 1) as usize] == character;

    first_match ^ second_match
}