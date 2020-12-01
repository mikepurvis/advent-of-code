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
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0
        {
            break;
        }
        numbers.push(line.trim_end().parse::<u32>().unwrap());
    }
    Ok(numbers)
}


fn main() -> std::io::Result<()> {
    let numbers = get_numbers()?;

    for num in 2..4 {
        for combination in numbers.iter().combinations(num) {

            //let sum: u32 = combination.iter().sum();
            //let sum: u32 = combination.clone().into_iter().sum();
            let sum = combination.iter().fold(0, |sum, &val| sum + val);

            if sum == 2020 {
                let product = combination.iter().fold(1, |prod, &val| prod * val);
                println!("{}", product);
                break;
            }
        }
    }
    Ok(())
}
