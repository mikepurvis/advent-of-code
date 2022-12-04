use std::fs;
use std::collections::HashSet;
use itertools::Itertools;


type Bag = (HashSet<char>, HashSet<char>);

fn process_input(input_str: &str) -> Vec<Bag> {
  input_str.lines().map(|line| {
    let mut left = line.trim().to_string();
    let right = left.split_off(left.len() / 2);
    (left.chars().collect(), right.chars().collect())
  }).collect()
}

fn item_priority(ch: char) -> u32 {
  if ch.is_uppercase() {
    ch as u32 - 'A' as u32 + 27
  } else {
    ch as u32 - 'a' as u32 + 1
  }
}

fn find_total_score(bags: &Vec<Bag>) -> u32 {
  bags.iter().map(|bag| {
    let intersect = bag.0.intersection(&bag.1).cloned().collect::<Vec<_>>();
    assert_eq!(intersect.len(), 1);
    let dup_item = *intersect.first().unwrap();
    item_priority(dup_item)
  }).sum()
}

fn find_total_groups_score(bags: &Vec<Bag>) -> u32 {
  bags.iter().chunks(3).into_iter().map(|bag_group| {
    let bag_vec = bag_group.map(|bag|
      bag.0.union(&bag.1).cloned().collect()
    ).collect::<Vec<HashSet<char>>>();

    // Should really be a fold() here.
    let intersect1 = bag_vec[0].intersection(&bag_vec[1]).cloned().collect::<HashSet<_>>();
    let intersect2 = intersect1.intersection(&bag_vec[2]).cloned().collect::<Vec<_>>();

    assert_eq!(intersect2.len(), 1);
    let dup_item = *intersect2.first().unwrap();
    item_priority(dup_item)
  }).sum()
}

fn main() {
  let bags = process_input(&fs::read_to_string("input.txt").unwrap());
  println!("{}", find_total_score(&bags));
  println!("{}", find_total_groups_score(&bags));
}

#[test]
fn test_prio() {
  assert_eq!(item_priority('p'), 16);
  assert_eq!(item_priority('L'), 38);
}

#[test]
fn test_sample() {
  const SAMPLE: &str = "\
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

  let bags = process_input(&SAMPLE);
  assert_eq!(find_total_score(&bags), 157);
  assert_eq!(find_total_groups_score(&bags), 70);
}
