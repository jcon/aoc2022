use std::cmp::{Ord, Ordering};
use std::str::FromStr;

use std::str::Lines;

struct Cli {
    pub path: std::path::PathBuf,
}

// NOTE: could use something like clap instead.
impl Cli {
    pub fn parse() -> Self {
        let path = std::env::args().nth(1).expect("no path given");
        Cli {
            path: std::path::PathBuf::from(path),
        }
    }
}

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  println!("predicted score: {}", predict_score(content.lines()).expect("couldn't predict score"));
}

fn predict_score(lines: Lines<'_>) -> Result<i32, ParseMatchError> {
  Ok(lines.map(|l| Match::from_str(l))
      // .try_fold(0, |accum, m1| m1.map(|m2| accum + m2.score()))
      .fold(0, |accum, m1| accum + m1.expect("should have a match").score()))

}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
enum Move {
  Rocks,
  Paper,
  Scissors
}

impl Move {
  pub fn score(&self) -> i32 {
    match *self {
      Self::Rocks => 1,
      Self::Paper => 2,
      Self::Scissors => 3,
    }
  }
}

impl Ord for Move {
  fn cmp(&self, other: &Self) -> Ordering {
    if self == other {
      return Ordering::Equal;
    }

    match *other {
      Self::Rocks if *self == Self::Scissors => Ordering::Less,
      Self::Rocks if *self == Self::Paper => Ordering::Greater,
      Self::Paper if *self == Self::Rocks => Ordering::Less,
      Self::Paper if *self == Self::Scissors => Ordering::Greater,
      Self::Scissors if *self == Self::Paper => Ordering::Less,
      Self::Scissors if *self == Self::Rocks => Ordering::Greater,
      _ => panic!("Should not be here")
    }
  }
}

#[derive(Debug, Clone)]
struct ParseMoveError;

impl FromStr for Move {
  type Err = ParseMoveError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let m = match s {
      "A" | "X" => Move::Rocks,
      "B" | "Y" => Move::Paper,
      "C" | "Z" => Move::Scissors,
      _ => return Err(ParseMoveError{}),
    };
    Ok(m)
  }
}

struct Match {
  their_move: Move,
  your_move: Move,
}

impl Match {
  pub fn score(&self) -> i32 {
    (match self.your_move.cmp(&self.their_move) {
      Ordering::Greater => 6,
      Ordering::Equal => 3,
      Ordering::Less => 0,
    }) + self.your_move.score()
  }
}

#[derive(Debug, Clone)]
struct ParseMatchError;

impl FromStr for Match {
    type Err = ParseMatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = s.split(" ");
        Ok(Match {
          their_move: moves.next().expect("Expected their move").parse().expect("not a valid move"),
          your_move: moves.next().expect("Expected my move").parse().expect("not a valid move"),
        })
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::Move::*;
  use std::cmp::Ordering::*;

  #[test]
  fn move_comparisons() {
      assert_eq!(Rocks.cmp(&Scissors), Greater);
      assert_eq!(Rocks.cmp(&Paper), Less);
      assert_eq!(Paper.cmp(&Rocks), Greater);
      assert_eq!(Paper.cmp(&Scissors), Less);
      assert_eq!(Scissors.cmp(&Paper), Greater);
      assert_eq!(Scissors.cmp(&Rocks), Less);
      assert_eq!(Paper.cmp(&Paper), Equal);
      assert_eq!(Scissors.cmp(&Scissors), Equal);
  }

  #[test]
  fn test_match_score() {
    assert_eq!(Match {
        their_move: Rocks,
        your_move: Paper,
      }.score(), 8);
    assert_eq!(Match {
        their_move: Paper,
        your_move: Rocks,
      }.score(), 1);
    assert_eq!(Match {
        their_move: Scissors,
        your_move: Rocks,
      }.score(), 7);
    assert_eq!(Match {
        their_move: Paper,
        your_move: Paper,
      }.score(), 5);
  }

  #[test]
  fn test_match_parsing() {
    assert_eq!(8, Match::from_str("A Y").unwrap().score());
    assert_eq!(1, Match::from_str("B X").unwrap().score());
    assert_eq!(6, Match::from_str("C Z").unwrap().score());
  }

  #[test]
  fn test_predict_score() {
    assert_eq!(15, predict_score("A Y
B X
C Z".lines()).expect("oops"));
  }
}
