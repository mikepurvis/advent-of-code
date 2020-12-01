extern crate itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;

use itertools::Itertools;


fn get_numbers() -> Result<Vec<u32>, std::io::Error> {
    let mut numbers = Vec::new();

    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        numbers.push(line.trim_end().parse::<u32>().unwrap());
        line.clear();
    }
    Ok(numbers)
}


fn main() -> std::io::Result<()> {
    let numbers = get_numbers()?;

    for num in 2..4 {
        for combination in numbers.iter().copied().combinations(num) {
            let sum: u32 = combination.iter().sum();
            if sum == 2020 {
                let product: u32 = combination.iter().product();
                println!("{}", product);
                break;
            }
        }
    }
    Ok(())
}
