extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

lazy_static! {
    static ref RE_FIELDS: Regex = Regex::new("([a-z]+):([^\\s]+)").unwrap();
    static ref RE_YR: Regex = Regex::new("([0-9]{4})$").unwrap();
    static ref RE_HGT: Regex = Regex::new("([0-9]+)(cm|in)$").unwrap();
    static ref RE_HCL: Regex = Regex::new("#[0-9a-f]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex::new("(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_PID: Regex = Regex::new("#[0-9]{9}$").unwrap();
}

struct Passport {
    fields: HashMap<String, String>
}
impl Passport {
    fn from_lines(mut lines: &mut std::str::Lines<'_>) -> Option<Self> {
        let mut passport = Self { fields: HashMap::new() };
        for line in &mut lines {
            if line.trim().is_empty() {
                // Found a linebreak, this passport is done.
                break;
            }
            for cap in RE_FIELDS.captures_iter(&line) {
                passport.fields.insert(cap[1].to_string(), cap[2].to_string());
            }
        }
        if passport.fields.is_empty() {
            return None;
        } 
        Some(passport)
    }

    fn check_fields(&self) -> bool {
        let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        for key in keys.iter() {
            if !self.fields.contains_key(*key) {
                return false;
            }
        }
        return true;
    }

    fn check_data(&self) -> bool {
        true
    }
}

fn passports_from_file(filename: &str) -> Result<Vec<Passport>, std::io::Error> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.lines();
    let mut passports = Vec::new();
    loop {
        match Passport::from_lines(&mut lines) {
            Some(passport) => passports.push(passport),
            None => break
        }
    }
    Ok(passports)
}

fn main() {
    let passports = passports_from_file("input.txt").unwrap();

    let mut valid_fields = 0;
    let mut valid_data = 0;
    for passport in passports.iter() { 
        if passport.check_fields() {
            valid_fields += 1;
            if passport.check_data() {
                valid_data += 1;
            }
        }
    }
    println!("Valid Fields: {}", valid_fields);
    println!("Valid Data: {}", valid_data);
}
