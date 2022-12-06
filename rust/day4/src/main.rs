use utils::cli::Cli;

use std::{str::Lines};
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  print_result("fully contained pairs", count_fully_contained_pairs(content.lines()));
  print_result("overlapping pairs", count_overlapping_pairs(content.lines()));
}

pub fn count_fully_contained_pairs(lines: Lines<'_>) -> Result<u32, ParseRangeError> {
  let scores: Result<Vec<bool>, ParseRangeError> = lines.map(|line| {
    let (r1, r2) = line.split_once(",")
      .ok_or(ParseRangeError{ message: format!("expected two ranges in: {}", line) })?;
    let r1 = Range::from_str(r1)?;
    let r2 = Range::from_str(r2)?;

    Ok(r1.fully_contains(&r2) || r2.fully_contains(&r1))
  }).collect();

  scores.map(|s| s.iter().filter_map(|b| if *b { Some(0) } else { None }).count() as u32)
        .map_err(|_e| ParseRangeError { message: "oops".to_string() })
}

pub fn count_overlapping_pairs(lines: Lines<'_>) -> Result<u32, ParseRangeError> {
  let scores: Result<Vec<bool>, ParseRangeError> = lines.map(|line| {
    let (r1, r2) = line.split_once(",")
      .ok_or(ParseRangeError{ message: format!("expected two ranges in: {}", line) })?;
    let r1 = Range::from_str(r1)?;
    let r2 = Range::from_str(r2)?;

    Ok(r1.overlaps(&r2))
  }).collect();

  scores.map(|s| s.iter().filter_map(|b| if *b { Some(0) } else { None }).count() as u32)
        .map_err(|_e| ParseRangeError { message: "oops".to_string() })
}

pub fn print_result(message: &str, r: Result<u32, ParseRangeError>) {
  match r {
    Ok(s) => println!("{} {}", message, s),
    Err(e) => println!("Error could not calculate: {}: {}", message, e.message)
  }
}

#[derive(Debug, Clone, Copy)]
struct Range {
  lower: i32,
  upper: i32
}

impl Range {
  pub fn fully_contains(&self, other: &Range) -> bool {
    other.lower >= self.lower && other.upper <= self.upper
  }

  pub fn contains(&self, point: i32) -> bool {
    point >= self.lower && point <= self.upper
  }

  pub fn overlaps(&self, other: &Range) -> bool {
    self.contains(other.lower) || self.contains(other.upper)
      || other.contains(self.lower) || other.contains(self.upper)
  }
}

impl FromStr for Range {
  type Err = ParseRangeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (l, u) = s
      .split_once('-')
      .ok_or(ParseRangeError{ message: format!("not a valid range: {}", s) })?;

    Ok(
      Range {
        lower: l.parse::<i32>().map_err(|_e| ParseRangeError { message: format!("not a valid range: {}", s) })?,
        upper: u.parse::<i32>().map_err(|_e| ParseRangeError { message: format!("not a valid range: {}", s) })?
      }
    )
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRangeError {
  message: String
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_count_fully_contained_pairsduplicate_item() {
    assert_eq!(Ok(2), count_fully_contained_pairs("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".lines()));
  }

  #[test]
  pub fn test_count_overlapping_pairs() {
    assert_eq!(Ok(4), count_overlapping_pairs("2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8".lines()));
  }

}
