use std::collections::HashSet;
use std::fs;

fn find_marker(signal: &str, len: usize) -> usize
{
  // I feel like this vector shouldn't have been necessary, but I couldn't
  // figure out how to call windows() on the &str or Chars directly.
  let chars = signal.chars().collect::<Vec<_>>();
  for (index, window) in chars.windows(len).enumerate() {
    let set = window.iter().collect::<HashSet<_>>();
    if set.len() == len {
      return index + len;
    }
  }
  panic!()
}

fn main() {
  let input = fs::read_to_string("input.txt").unwrap();
  println!("{}", find_marker(&input, 4));
  println!("{}", find_marker(&input, 14));
}


#[test]
fn test_samples() {
  assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
  assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
  assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
  assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
  assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

  assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
  assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
  assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
  assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
  assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
}
