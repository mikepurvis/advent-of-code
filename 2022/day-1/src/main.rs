use std::fs;


fn process_input(input_str: &str) -> Vec<Vec<u32>> {
  input_str.split("\n\n").map(|group_str|
    group_str.lines().map(|s|
      s.trim().parse::<_>().unwrap()
    ).collect()
  ).collect()
}

fn find_largest_group(input: &Vec<Vec<u32>>) -> u32 {
  input.iter().map(|v| v.iter().sum()).max().unwrap() 
}

fn find_largest_three(input: &Vec<Vec<u32>>) -> u32 {
  let mut sums = input.iter().map(|v| v.iter().sum()).collect::<Vec<u32>>();
  sums.sort();
  sums.iter().rev().take(3).into_iter().sum()
}

fn main() {
  let input = process_input(&fs::read_to_string("input.txt").unwrap());
  println!("{}", find_largest_group(&input));
  println!("{}", find_largest_three(&input));
}


#[test]
fn test_sample() {
  const SAMPLE: &str = "\
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000";

  let input = process_input(&SAMPLE);
  assert_eq!(find_largest_group(&input), 24000);
  assert_eq!(find_largest_three(&input), 45000);
}
