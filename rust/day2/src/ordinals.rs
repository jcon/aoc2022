use std::cmp::{Ord, Ordering};
use std::str::FromStr;

use std::str::Lines;

#[allow(dead_code)]
fn predict_score_from_move(lines: Lines<'_>) -> Result<i32, ParseMatchError> {
  Ok(
    lines.map(|l| Match::from_str(l))
      .collect::<Result<Vec<Match>,_>>()?
      .iter()
      .fold(0, |acc, m1| acc + m1.score_from_move())
  )
}

#[allow(dead_code)]
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
    (*self as i32) + 1
 }

  pub fn get_move(&self, ord: Ordering) -> Move {
    // Take advantage of of a few things:
    // - Move is ordered so that the next element in the enum (with modulo arithmetic) is greater than the previous element.
    // - Ordering's ordinal values are: -1 for less, 0 for equal, and 1 for greater
    // - modulo math gets the correct results: Less than Rocks => (-1 + 0).rem_euclid(3) = 2 (Scissors, which loses to rocks)
    //    NOTE: % in rust is remainder, not modulo as might be expected. i8::rem_euclid performs modulo math instead.
    let move_ordinal = ((*self as i8) + (ord as i8)).rem_euclid(3);
    match move_ordinal {
      0 => Self::Rocks,
      1 => Self::Paper,
      2 => Self::Scissors,
      _ => panic!("Should not be here; ordinal can only be [0, 2] {}", move_ordinal)
    }
  }
}

impl Ord for Move {
  // Rocks - 0
  // Paper - 1
  // Scissors - 2
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
struct ParseMatchError;

impl FromStr for Match {
    type Err = ParseMatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = s.split(" ");
        let their_move = moves.next().expect("Expected their move").parse().expect("not a valid move");
        let next = moves.next().expect("Expected my move");
        Ok(Match {
          their_move,
          your_move: next.parse().expect("not a valid move"),
          you_should_be: match next {
            "X" => Ordering::Less,
            "Y" => Ordering::Equal,
            "Z" => Ordering::Greater,
            _ => return Err(ParseMatchError{})
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
