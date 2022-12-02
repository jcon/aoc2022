use std::cmp::{Ord, Ordering};
use std::str::FromStr;
mod ordinals;

use std::str::Lines;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  println!("predicted score if you play: {}", predict_score_from_move(content.lines()).expect("couldn't predict score"));
  println!("predicted score if you have suggested match result: {}", predict_score_from_result(content.lines()).expect("couldn't predict score"));
}

// NOTE: I was trying to figure out how to instead accept an Iterator of string instead, since it felt
// more flexible and less tied to strings. (for instance it'd pipe better into file i/o if we didn't
// read everything into a string). It is possible, but more verbose, and involves declaring lifetimes
// fn predict_score_from_move<'a>(lines: impl Iterator<Item = &'a str>) -> Result<i32, ParseMatchError> {
fn predict_score_from_move(lines: Lines<'_>) -> Result<i32, ParseMatchError> {
  Ok(
    lines.map(|l| Match::from_str(l))
      .collect::<Result<Vec<Match>,_>>()?
      .iter()
      .fold(0, |acc, m1| acc + m1.score_from_move())
  )
}

fn predict_score_from_result(lines: Lines<'_>) -> Result<i32, ParseMatchError> {
  Ok(
    lines.map(|l| Match::from_str(l))
      .collect::<Result<Vec<Match>,_>>()?
      .iter()
      .fold(0, |acc, m1| acc + m1.score_from_result())
  )
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

  pub fn get_move(&self, ord: Ordering) -> Move {
    if ord == Ordering::Equal {
      return self.clone();
    }
    match self {
      Self::Rocks if ord == Ordering::Less => Self::Scissors,
      Self::Rocks if ord == Ordering::Greater => Self::Paper,
      Self::Paper if ord == Ordering::Less => Self::Rocks,
      Self::Paper if ord == Ordering::Greater => Self::Scissors,
      Self::Scissors if ord == Ordering::Less => Self::Paper,
      Self::Scissors if ord == Ordering::Greater => Self::Rocks,
      _ => panic!("Should not get here")
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
  you_should_be: Ordering,
}

impl Match {
  pub fn score_from_move(&self) -> i32 {
    (match self.your_move.cmp(&self.their_move) {
      Ordering::Greater => 6,
      Ordering::Equal => 3,
      Ordering::Less => 0,
    }) + self.your_move.score()
  }

  pub fn score_from_result(&self) -> i32 {
    let your_move = self.their_move.get_move(self.you_should_be);
    (match your_move.cmp(&self.their_move) {
      Ordering::Greater => 6,
      Ordering::Equal => 3,
      Ordering::Less => 0,
    }) + your_move.score()
  }
}

#[derive(Debug, Clone)]
enum ParseMatchError {
  ExpectedMove{ s: String },
  InvalidMove{ s: String },
  InvalidOrdering{ s: String }
}

impl FromStr for Match {
    type Err = ParseMatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = s.split(" ");
        let their_move =
          moves.next()
               .ok_or(ParseMatchError::ExpectedMove { s: s.to_string() })?
               .parse::<Move>().map_err(|_| ParseMatchError::InvalidMove { s: s.to_string() })?;
        let next =
          moves.next()
               .ok_or(ParseMatchError::ExpectedMove { s: s.to_string() })?;
        Ok(Match {
          their_move,
          your_move: next.parse().map_err(|_| ParseMatchError::InvalidMove { s: s.to_string() })?,
          you_should_be: match next {
            "X" => Ordering::Less,
            "Y" => Ordering::Equal,
            "Z" => Ordering::Greater,
            _ => return Err(ParseMatchError::InvalidOrdering { s: s.to_string() })
          }
        })
    }
}

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
        you_should_be: Less,
      }.score_from_move(), 8);
    assert_eq!(Match {
        their_move: Paper,
        your_move: Rocks,
        you_should_be: Greater,
      }.score_from_move(), 1);
    assert_eq!(Match {
        their_move: Scissors,
        your_move: Rocks,
        you_should_be: Greater,
      }.score_from_move(), 7);
    assert_eq!(Match {
        their_move: Paper,
        your_move: Paper,
        you_should_be: Greater,
      }.score_from_move(), 5);
  }

  #[test]
  fn test_move_get_move() {
    assert_eq!(Move::Scissors, Move::Rocks.get_move(Less));
    assert_eq!(Move::Paper, Move::Rocks.get_move(Greater));
    assert_eq!(Move::Rocks, Move::Paper.get_move(Less));
    assert_eq!(Move::Scissors, Move::Paper.get_move(Greater));
    assert_eq!(Move::Paper, Move::Scissors.get_move(Less));
    assert_eq!(Move::Rocks, Move::Scissors.get_move(Greater));
  }

  #[test]
  fn test_match_parsing() {
    assert_eq!(8, Match::from_str("A Y").unwrap().score_from_move());
    assert_eq!(4, Match::from_str("A Y").unwrap().score_from_result());
    assert_eq!(1, Match::from_str("B X").unwrap().score_from_move());
    assert_eq!(1, Match::from_str("B X").unwrap().score_from_result());
    assert_eq!(6, Match::from_str("C Z").unwrap().score_from_move());
    assert_eq!(7, Match::from_str("C Z").unwrap().score_from_result());
  }

  #[test]
  fn test_predict_score_from_move() {
    assert_eq!(15, predict_score_from_move("A Y
B X
C Z".lines()).expect("oops"));
  }

  #[test]
  fn test_predict_score_from_result() {
    assert_eq!(12, predict_score_from_result("A Y
B X
C Z".lines()).expect("oops"));
  }
}
