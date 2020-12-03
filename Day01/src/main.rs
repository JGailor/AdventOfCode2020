use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: day_01 input");
    }

    let input_fn = args[1].clone();
    let numbers = parse_input_to_numbers(input_fn);

    for (index, number) in numbers.iter().enumerate() {
        for next_number in &numbers[index+1..] {
            if number + next_number == 2020 {
                println!("{} + {} = {}", number, next_number, number + next_number);
                println!("{} * {} = {}", number, next_number, number * next_number);
            }
        }
    }
}

fn parse_input_to_numbers<P>(filename: P) -> Vec<u32> 
where P: AsRef<Path> {
    let result = File::open(filename);
    let lines = match result {
        Ok(file) => {io::BufReader::new(file).lines()},
        Err(_) => {panic!("Could not open file")}
    };

    let mut results:Vec<u32> = vec!();

    for line in lines {
        if let Ok(str_number) = line {
            if let Ok(number) = str_number.parse::<u32>() {
                results.push(number);
            }
        }
    }

    results
}
