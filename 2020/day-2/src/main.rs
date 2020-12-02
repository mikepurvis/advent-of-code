extern crate regex;

use regex::Regex;
use std::fs;
use std::vec::Vec;

#[derive(Debug)]
struct Input {
    password: String,
    letter: char,
    min: usize,
    max: usize,
}

impl Input {
    fn get_from_file(filename: &str) -> Result<Vec<Input>, std::io::Error> {
        let mut inputs: Vec<Input> = Vec::new();

        let contents = fs::read_to_string(filename).unwrap();

        let re = Regex::new("(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<char>[a-z]): (?P<password>[a-z]*)").unwrap();
        for caps in re.captures_iter(&contents) {
            let input = Input {
                password: caps.name("password").unwrap().as_str().to_string(),
                letter: caps.name("char").unwrap().as_str().chars().next().unwrap(),
                min: caps.name("min").unwrap().as_str().parse::<usize>().unwrap(),
                max: caps.name("max").unwrap().as_str().parse::<usize>().unwrap()
            };
            inputs.push(input);
        }
        Ok(inputs)
    }

    fn is_valid_policy_1(&self) -> bool {
        let mut ch_count = 0;
        for ch in self.password.chars() {
            if ch == self.letter {
                ch_count += 1;
            }
        }
        return ch_count >= self.min && ch_count <= self.max;
    }

    fn is_valid_policy_2(&self) -> bool {
        let min_char = self.password.chars().nth(self.min - 1).unwrap();
        let max_char = self.password.chars().nth(self.max - 1).unwrap();
        return (min_char == self.letter) ^ (max_char == self.letter)
    }
}

fn main() -> std::io::Result<()> {
    let mut valid1_count = 0;
    let mut valid2_count = 0;
    for input in Input::get_from_file("input.txt")? {
        if input.is_valid_policy_1() {
            valid1_count += 1;
        }
        if input.is_valid_policy_2() {
            valid2_count += 1;
        }
    }
    println!("Policy 1 valid: {}", valid1_count);
    println!("Policy 2 valid: {}", valid2_count);
    Ok(())
}
