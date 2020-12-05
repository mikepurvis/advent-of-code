extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

type Passport = HashMap<String, String>;
type Passports = Vec<Passport>;

fn passports_from_file(filename: &str) -> Result<Passports, std::io::Error> {
    let re = Regex::new("([a-z]+):([^\\s]+)").unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.lines();
    let mut passports: Passports = Passports::new();
    loop {
        let mut passport = Passport::new();
        for line in &mut lines {
            if line.trim().is_empty() {
                // Found a linebreak, this passport is done.
                break;
            }
            for cap in re.captures_iter(&line) {
                passport.insert(cap[1].to_string(), cap[2].to_string());
            }
        }
        if passport.is_empty() {
            break;
        }
        passports.push(passport);
    }
    Ok(passports)
}

fn check_passport(passport: &Passport, keys: &Vec<&str>) -> bool {
    for key in keys.iter() {
        if !passport.contains_key(*key) {
            return false;
        }
    }
    return true;
}

fn main() {
    let passports = passports_from_file("input.txt").unwrap();
    let mut valid_count = 0;
    let check_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for passport in passports.iter() { 
        if check_passport(&passport, &check_keys) {
            valid_count += 1;
        }
    }

    println!("Valid: {}", valid_count);
}
