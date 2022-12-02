extern crate regex;

use regex::Regex;
use std::fs;
use std::marker::PhantomData;
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
        let contents = fs::read_to_string(filename).unwrap();
        let re = Regex::new("(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<letter>[a-z]): (?P<password>[a-z]*)").unwrap();
        Ok(re.captures_iter(&contents).map(|caps|
            Input {
                password: caps.name("password").unwrap().as_str().to_string(),
                letter: caps.name("letter").unwrap().as_str().chars().next().unwrap(),
                min: caps.name("min").parse().unwrap(),
                max: caps.name("max").parse().unwrap()
            }
        ).collect())
    }
}

trait Policy {
    fn check(input: &Input) -> bool;
}

struct Policy1;
impl Policy for Policy1 {
    fn check(input: &Input) -> bool {
        let mut ch_count = 0;
        for ch in input.password.chars() {
            if ch == input.letter {
                ch_count += 1;
            }
        }
        return ch_count >= input.min && ch_count <= input.max;
    }
}

struct Policy2;
impl Policy for Policy2 {
    fn check(input: &Input) -> bool {
        let min_char = input.password.chars().nth(input.min - 1).unwrap();
        let max_char = input.password.chars().nth(input.max - 1).unwrap();
        return (min_char == input.letter) ^ (max_char == input.letter)
    }
}

struct PolicyCounter<T: Policy> {
    count: usize,
    policy: PhantomData<T>
}

impl<T: Policy> PolicyCounter<T> {
    fn new() -> PolicyCounter<T> {
        return PolicyCounter { count: 0, policy: PhantomData }
    }

    fn check(&mut self, input: &Input) {
        if T::check(input) {
            self.count += 1;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut p1: PolicyCounter::<Policy1> = PolicyCounter::new();
    let mut p2: PolicyCounter::<Policy2> = PolicyCounter::new();

    for input in Input::get_from_file("input.txt")? {
        p1.check(&input);
        p2.check(&input);
    }
    println!("Policy 1 valid: {}", p1.count);
    println!("Policy 2 valid: {}", p2.count);
    Ok(())
}
