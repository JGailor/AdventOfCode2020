use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const F_BYR: u8 = 0b00000001;
const F_IYR: u8 = 0b00000010;
const F_EYR: u8 = 0b00000100;
const F_HGT: u8 = 0b00001000;
const F_HCL: u8 = 0b00010000;
const F_ECL: u8 = 0b00100000;
const F_PID: u8 = 0b01000000;
const F_CID: u8 = 0b10000000;

const VALID_STATE_1: u8 = 0b01111111;
const VALID_STATE_2: u8 = 0b11111111;

fn main() {
    let args:Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("usage: day_04 input_file");
    } else {
        let valid_passports = run(&args[0]);
        println!("found {} valid passports", valid_passports);
    }
}

fn run(input_fn: &String) -> u32 {
    let lines = match File::open(input_fn) {
        Ok(file) => {io::BufReader::new(file).lines()},
        Err(_) => {panic!("Could not open file {}", input_fn)},
    };

    let mut passport_lines:Vec<String> = Vec::new();
    let mut valid_passports:u32 = 0;

    for line in lines {
        if let Ok(l) = line {
            if l != "" {
                passport_lines.push(l);
            } else {
                // Done with that passport, process it and empty the buffer
                if valid_passport(&passport_lines) {
                    valid_passports += 1;
                }
                passport_lines.clear();
            }
        }
    };

    valid_passports
}

fn valid_passport(lines: &Vec<String>) -> bool {
    let mut validation_checks: u8 = 0b00000000;

    for line in lines {
        let keyword_pairs: Vec<&str> = line.split(" ").collect();
        for keyword_pair in keyword_pairs {
            let key_value: Vec<&str> = keyword_pair.split(":").collect();
            if key_value.len() != 2 {
                return false
            }
            let (field, _valid) = lookup_field_by_name(&key_value[0]);
            validation_checks = validation_checks | field;
        };
    };

    validation_checks == VALID_STATE_1 || validation_checks == VALID_STATE_2
}

fn lookup_field_by_name(name: &str) -> (u8, bool) {
    match name {
        "byr" => {(F_BYR, true)},
        "iyr" => {(F_IYR, true)},
        "eyr" => {(F_EYR, true)},
        "hgt" => {(F_HGT, true)},
        "hcl" => {(F_HCL, true)},
        "ecl" => {(F_ECL, true)},
        "pid" => {(F_PID, true)},
        "cid" => {(F_CID, true)},
        _     => {(0, false)},
    }
}