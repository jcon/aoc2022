use utils::cli::Cli;

use std::{str::Lines, collections::HashSet};

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  print_result("score", score_duplicate_item(content.lines()));
  print_result("score groups", score_groups_unique_elements(&content));
}

pub fn score_duplicate_item(lines: Lines<'_>) -> Result<i32, ScoringError> {
  let scores: Result<Vec<_>, _> = lines.map(|l| {
    let items: Vec<_> = l.chars().collect();
    let left: HashSet<_> = items[..(items.len() / 2)].iter().collect();
    let right: HashSet<_> = items[(items.len() / 2) .. (items.len())].iter().collect();
    let union: Vec<_> = left.intersection(&right).copied().collect();
    match union.len() {
      1 => Ok(score(union[0])),
      _ => Err(ScoringError{ message: "Expected a single duplicate element".to_string() })
    }
  }).collect();

  scores.map(|s| s.iter().sum())
}

pub fn score_groups_unique_elements(content: &str) -> Result<i32, ScoringError> {
  let scores: Result<Vec<_>, _> = content.lines().collect::<Vec<_>>().chunks(3)
         .map(|lines| {
          if lines.len() != 3 {
            return Err(ScoringError {
              message: format!("Expected 3 elves rumsacks, but got {}", lines.len()).to_string()
            });
          }

          let mut union: HashSet<_> = lines[0].chars().collect();
          for line in lines {
            let c = line.chars().collect::<HashSet<_>>();
            union = union.intersection(&c).copied().collect::<HashSet<_>>();
          }
          let unique: Vec<_> = union.iter().collect();
          match unique.len() {
            1 => Ok(score(unique[0])),
            _ => Err(ScoringError{ message: "Expected a single duplicate element".to_string() })
          }
         }).collect();
  scores.map(|s| s.iter().sum())
}

pub fn score(c: &char) -> i32 {
  if *c < 'a' {
    ((*c as u32) - ('A' as u32) + 27) as i32
  } else {
    ((*c as u32) - ('a' as u32) + 1) as i32
  }
}

pub fn print_result(message: &str, r: Result<i32, ScoringError>) {
  match r {
    Ok(s) => println!("{} {}", message, s),
    Err(e) => println!("Error could not calculate: {}: {}", message, e.message)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoringError {
  message: String
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_duplicate_item() {
    assert_eq!(Ok(157), score_duplicate_item("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".lines()));
  }

  #[test]
  pub fn test_score_groups_unique_item() {
    assert_eq!(Ok(70), score_groups_unique_elements("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"));
  }
}
