use utils::cli::Cli;

use std::{str::Lines, collections::HashSet};

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  println!("score: {}", score_duplicate_item(content.lines()));
  println!("score groups: {}", score_groups_unique_elements(&content));
}

pub fn score_duplicate_item(lines: Lines<'_>) -> i32 {
  let scores: Vec<_> = lines.map(|l| {
    let items: Vec<_> = l.chars().collect();
    let left: HashSet<_> = items[..(items.len() / 2)].iter().collect();
    let right: HashSet<_> = items[(items.len() / 2) .. (items.len())].iter().collect();
    let unique = left.intersection(&right).copied().next().unwrap();
    score(unique)
  }).collect();

  scores.iter().sum()
}

pub fn score_groups_unique_elements(content: &str) -> i32 {
  content.lines().collect::<Vec<_>>().chunks(3)
         .map(|lines| {
          // NOTE: blindly assumes there are always 3
          let mut union: HashSet<_> = lines[0].chars().collect();
          for line in lines {
            let c = line.chars().collect::<HashSet<_>>();
            union = union.intersection(&c).copied().collect::<HashSet<_>>();
          }
          let unique = union.iter().next().unwrap();
          score(unique)
         }).sum()
}

pub fn score(c: &char) -> i32 {
  if *c < 'a' {
    ((*c as u32) - ('A' as u32) + 27) as i32
  } else {
    ((*c as u32) - ('a' as u32) + 1) as i32
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_duplicate_item() {
    assert_eq!(157, score_duplicate_item("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".lines()));
  }

  #[test]
  pub fn test_score_groups_unique_item() {
    assert_eq!(70, score_groups_unique_elements("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"));
  }
}
