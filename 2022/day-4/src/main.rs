use lazy_static::lazy_static;
use ranges::Domain;
use ranges::GenericRange;
use ranges::OperationResult;
use regex::Regex;
use std::fs;


type Assignment = (GenericRange<u32>, GenericRange<u32>);

fn parse_input(input: &str) -> Vec<Assignment>
{
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
  }

  input.lines().map(|line| {
    let caps = RE.captures(line.trim()).unwrap();
    let cap_ints = caps.iter().skip(1).map(|s| {
      s.unwrap().as_str().parse().unwrap()
    }).collect::<Vec<u32>>();
  
    (GenericRange::new_closed(cap_ints[0], cap_ints[1]),
     GenericRange::new_closed(cap_ints[2], cap_ints[3]))
  }).collect()
}

// Regrettably, GenericRange's built in method for this doesn't result true if
// the bounds touch, so we have to bring our own.
fn is_subset<T>(left: GenericRange<T>, right: GenericRange<T>) -> bool where T: Domain + Copy
{
  match left.union(right) {
    OperationResult::Empty => panic!(),
    OperationResult::Double(_, _) => false,
    OperationResult::Single(union) => union == left || union == right
  }
}

fn count_fully_contained(assignments: &Vec<Assignment>) -> u32
{
  assignments.iter().map(|a| 
    if is_subset(a.0, a.1) { 1 } else { 0 }
  ).sum()
}

fn count_any_overlap(assignments: &Vec<Assignment>) -> u32
{
  assignments.iter().map(|a| {
    let intersect = a.0.intersect(a.1);
    match intersect {
      OperationResult::Single(_) => 1,
      _ => 0
    }
  }).sum()
}

fn main() {
  let a = parse_input(&fs::read_to_string("input.txt").unwrap());
  println!("{}", count_fully_contained(&a));
  println!("{}", count_any_overlap(&a));
}


#[test]
fn test_sample() {
  const SAMPLE: &str = "\
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

  let assignments = parse_input(&SAMPLE);
  assert_eq!(count_fully_contained(&assignments), 2);
  assert_eq!(count_any_overlap(&assignments), 4);
}
