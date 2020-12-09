extern crate itertools;

use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

fn numbers_from_contents(contents: &str) -> Vec<u64> {
    contents.lines()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect()
}

fn find_number(preamble_len: usize, numbers: &[u64]) -> Option<u64> {
    let mut iter = numbers.iter();
    let mut preamble: VecDeque<&u64> = iter.by_ref().take(preamble_len).collect();
    for value in iter {
        if preamble.iter().copied().combinations(2)
            .any(|c| c.iter().copied().sum::<u64>() == *value) {
            preamble.pop_front();
            preamble.push_back(value);
            continue;
        }
        preamble.pop_front();
        preamble.push_back(value);
        return Some(*value);
    }
    None
}

fn find_run(all: &[u64], target_sum: u64) -> Option<&[u64]> {
    for slice_start in 0..all.len() {
        let inner_slice = &all[slice_start..];
        let mut sum = 0;
        for (count, value) in inner_slice.iter().enumerate() {
            sum += value;
            if sum == target_sum {
                return Some(&inner_slice[..(count+1)]);
            } else if sum > target_sum {
                break;
            }
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let numbers = numbers_from_contents(&contents);
    let result = find_number(25, &numbers).unwrap();
    println!("Found: {}", result);

    let run = find_run(&numbers, result).unwrap();
    println!("Run: {:?}", run);
    println!("Sum: {}", run.iter().min().unwrap() +
                        run.iter().max().unwrap());
}

#[test]
fn test_sample() {
    const SAMPLE_DATA: &str = r#"35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576"#;

    let numbers = numbers_from_contents(SAMPLE_DATA);
    let result = find_number(5, &numbers).unwrap();
    assert_eq!(127, result);
    assert_eq!(vec![15, 25, 47, 40], find_run(&numbers, result).unwrap());
}
