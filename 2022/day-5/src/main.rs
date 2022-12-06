use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

type Stack = Vec<char>;

#[derive(Debug)]
struct Step {
  count: usize,
  from: usize,
  to: usize
}

fn parse_input(input: &str) -> (Vec<Stack>, Vec<Step>)
{
  let (stacks, steps) = input.split_once("\n\n").unwrap();
  (parse_stacks(stacks), parse_steps(steps))
}

fn parse_stacks(input: &str) -> Vec<Stack>
{
  let mut stacks = Vec::new();
  let mut lines = input.lines().collect::<Vec<_>>();
  let label_row = lines.pop().unwrap();
  let num_stacks = ((label_row.len() + 1) / 4) as usize;
  stacks.resize(num_stacks, vec![]);

  lines.reverse();
  for line in lines {
    for i in 1..=num_stacks {
      let ch = line.as_bytes()[i * 4 - 3] as char;
      if ch != ' ' {
        stacks[i - 1].push(ch);
      }
    }
  }
  return stacks;
}

fn parse_steps(input: &str) -> Vec<Step>
{
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
  }
  input.lines().map(|line| {
    let caps = RE.captures(line).unwrap();
    Step {
      count: caps.get(1).unwrap().as_str().parse().unwrap(),
      from: caps.get(2).unwrap().as_str().parse().unwrap(),
      to: caps.get(3).unwrap().as_str().parse().unwrap()
    }
  }).collect()
}

fn execute_steps_9000(stacks: &mut Vec<Stack>, steps: &Vec<Step>)
{
  for step in steps.iter() {
    for _ in 0..step.count {
      let item = stacks[step.from - 1].pop().unwrap();
      stacks[step.to - 1].push(item);
    }
  }
}

fn execute_steps_9001(stacks: &mut Vec<Stack>, steps: &Vec<Step>)
{
  for step in steps.iter() {
    let drain_range = stacks[step.from - 1].len() - step.count..;
    let items = stacks[step.from - 1].drain(drain_range).collect::<Vec<_>>();
    stacks[step.to - 1].extend(items.iter());
  }
}

fn get_result(stacks: &Vec<Stack>) -> String
{
  stacks.iter().map(|s| {
    s.last().unwrap()
  }).collect()
}

fn main() {
  let text = fs::read_to_string("input.txt").unwrap();
  let (stacks, steps) = parse_input(&text);

  let mut stacks1 = stacks.clone();
  execute_steps_9000(&mut stacks1, &steps);
  println!("{}", get_result(&stacks1));

  let mut stacks2 = stacks.clone();
  execute_steps_9001(&mut stacks2, &steps);
  println!("{}", get_result(&stacks2));
}


#[test]
fn test_sample() {
  const SAMPLE: &str = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
  let (stacks, steps) = parse_input(&SAMPLE);

  let mut stacks1 = stacks.clone();
  execute_steps_9000(&mut stacks1, &steps);
  assert_eq!(get_result(&stacks1), "CMZ");

  let mut stacks2 = stacks.clone();
  execute_steps_9001(&mut stacks2, &steps);
  assert_eq!(get_result(&stacks2), "MCD");
}
