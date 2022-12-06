use utils::cli::Cli;

use std::{str::Lines};
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  print_result("fully contained pairs", count_fully_contained_pairs(content.lines()));
}

pub fn count_fully_contained_pairs(lines: Lines<'_>) -> Result<u32, ParseRangeError> {
  let scores: Result<Vec<bool>, ParseRangeError> = lines.map(|line| {
    let ranges: Result<Vec<Range>, _> = line.split(",").map(|e| {
      Range::from_str(e)
      // Range::par
      // let indicies: Vec<i32> = e.split("-").map(|i| i.parse::<i32>().unwrap()).collect();
      // Range {
      //   lower: indicies[0],
      //   upper: indicies[1]
      // }
    }).collect();

    // println!("ranges: {:?}", ranges);
    ranges.map(|r| {
      r[0].contains(&r[1]) || r[1].contains(&r[0])
    })
    // Ok(true)

//     Ok(ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]))
  }).collect();

  // println!("contained: {:?}", scores);

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
  pub fn contains(&self, other: &Range) -> bool {
    other.lower >= self.lower && other.upper <= self.upper
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

}
