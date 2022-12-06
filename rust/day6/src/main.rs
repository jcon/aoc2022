use utils::cli::Cli;

use std::collections::{HashSet, VecDeque};
use std::ops::ControlFlow;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  println!("start of packet: {}", first_unique_chars(&content, 4));
  println!("start of message: {}", first_unique_chars(&content, 14));
}

pub fn first_unique_chars(content: &str, n: usize) -> usize {
  let mut seen: VecDeque<char> = VecDeque::new();
  for (i, c) in content.chars().enumerate() {
    seen.push_back(c);
    if seen.iter().copied().collect::<HashSet<_>>().len() == n {
      return i + 1;
    }
    if seen.len() == n {
      seen.pop_front();
    }
  }
  return 0;
}

pub fn first_unique_chars_with_try_for_each(content: &str, n: usize) -> usize {
  let mut last_n: VecDeque<char> = VecDeque::from(
    content[0..n].chars().collect::<Vec<char>>()
  );
  let mut count = n - 1;
  content.chars().skip(n - 1).try_for_each(|c| {
    last_n.push_back(c);
    count = count + 1;
    let s: HashSet<char> = last_n.iter().copied().collect();
    if s.len() == n {
      return ControlFlow::Break(());
    }
    last_n.pop_front();
    return ControlFlow::Continue(());
  });

  count
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_start_of_packet() {
    assert_eq!(7, first_unique_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    assert_eq!(19, first_unique_chars("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    assert_eq!(23, first_unique_chars("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    assert_eq!(23, first_unique_chars("nppdvjthqldpwncqszvftbrmjlhg", 14));
    assert_eq!(29, first_unique_chars("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    assert_eq!(26, first_unique_chars("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
  }
}
