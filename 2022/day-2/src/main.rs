use std::fs;


fn process_input(input_str: &str) -> Vec<String> {
  input_str.lines().map(|line|
    line.trim().to_string()
  ).collect()
}

fn get_total_score(input: &Vec<String>) -> u32 {
  input.iter().map(|line| {
    match line.as_str() {
      "A X" => 1 + 3,
      "A Y" => 2 + 6,
      "A Z" => 3,
      "B X" => 1,
      "B Y" => 2 + 3,
      "B Z" => 3 + 6,
      "C X" => 1 + 6,
      "C Y" => 2,
      "C Z" => 3 + 3,
      _ => panic!()
    }
  }).sum()
}

fn get_total_score2(input: &Vec<String>) -> u32 {
  input.iter().map(|line| {
    match line.as_str() {
      "A X" => 3,
      "A Y" => 1 + 3,
      "A Z" => 2 + 6,
      "B X" => 1,
      "B Y" => 2 + 3,
      "B Z" => 3 + 6,
      "C X" => 2,
      "C Y" => 3 + 3,
      "C Z" => 1 + 6,
      _ => panic!()
    }
  }).sum()
}

fn main() {
  let input = process_input(&fs::read_to_string("input.txt").unwrap());
  println!("{}", get_total_score(&input));
  println!("{}", get_total_score2(&input));
}


#[test]
fn test_sample() {
  const SAMPLE: &str = "\
    A Y
    B X
    C Z";

  let input = process_input(&SAMPLE);
  assert_eq!(get_total_score(&input), 15);
  assert_eq!(get_total_score2(&input), 12);
}
