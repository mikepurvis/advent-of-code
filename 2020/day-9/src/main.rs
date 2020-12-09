extern crate itertools;

use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

fn numbers_from_contents(contents: &str) -> Vec<u64> {
    contents.lines()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect()
}

#[derive(Debug)]
struct NumberFinder<I> where I: Iterator<Item = u64> {
    iter: I,
    preamble: VecDeque<u64>
}

impl<I: Iterator<Item = u64>> NumberFinder<I> {
    fn start(preamble_len: usize, mut iter: I) -> NumberFinder<I> {
        NumberFinder {
            preamble: iter.by_ref().take(preamble_len).collect(),
            iter: iter
        }
    }
}

impl<I: Iterator<Item = u64>> Iterator for NumberFinder<I> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        fn rotate(preamble: &mut VecDeque<u64>, value: u64) {
            preamble.pop_front();
            preamble.push_back(value);
        }

        loop {
            let value = self.iter.next()?;
            if self.preamble.iter().copied().combinations(2)
                .any(|c| c.iter().sum::<u64>() == value) {
                rotate(&mut self.preamble, value);
                continue;
            }
            rotate(&mut self.preamble, value);
            return Some(value);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let numbers = numbers_from_contents(&contents);
    let mut nf = NumberFinder::start(25, numbers.into_iter());
    println!("{}", nf.next().unwrap());
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
    let nf = NumberFinder::start(5, numbers.into_iter());
    assert_eq!(vec![127], nf.collect::<Vec<_>>());
}
