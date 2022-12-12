use std::char::from_digit;
use std::{str::Lines, collections::HashSet};
use std::cmp::{min, max};
use utils::cli::Cli;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");

  let commands: Vec<(Direction, i32)> = content
    .lines()
    .map(|line| {
      line.split_once(" ")
        .map(|(d, a)| (Direction::new(d), a.parse::<i32>().unwrap()))
        .unwrap()
    })
    .collect();


  simulate_movements(&commands, 2);
  simulate_movements(&commands, 10);
}

fn simulate_movements(commands: &Vec<(Direction, i32)>, knot_count: usize) {
  let mut moves = HashSet::new();
  let mut knots = vec![Point{ x: 0, y: 0}; knot_count];
  moves.insert(knots.last().unwrap().clone());
  // let board = debug::get_bounds(commands);
  // debug::draw_board(&board, &knots);
  for (d, a) in commands {
    // println!("move {:?} {}", d, a);
    for _i in 0..*a {
      let head = knots.first_mut().unwrap();
      head.r#move(&d);
      for j in 1..knot_count {
        if !knots[j].is_adjacent(&knots[j - 1]) {
          let tmp = knots[j - 1].clone();
          knots[j].move_adjacent(&tmp);
        } else {
          break;
        }
      }
      moves.insert(knots.last().unwrap().clone());
      // debug::draw_board(&board, &knots);
      // println!("");
    }
  }

  println!("{} nots, tail moves: {}", knots.len(), moves.len());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
  x: i32,
  y: i32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left
}

impl Point {
  pub fn r#move(&mut self, direction: &Direction) {
    self.y = self.y + match direction {
      Direction::Up => 1,
      Direction::Down => -1,
      _ => 0,
    };
    self.x = self.x + match direction {
      Direction::Right => 1,
      Direction::Left => -1,
      _ => 0,
    };
  }

  pub fn move_adjacent(&mut self, other: &Point) {
    if self.is_adjacent(other) {
      return;
    }

    // simplified logic because |diff_x|, |diff_x| never greater than 2
    let diff_x = other.x - self.x;
    let diff_y = other.y - self.y;
    let sign_x = if diff_x < 0 { - 1 } else { 1 };
    let sign_y = if diff_y < 0 { - 1 } else { 1 };
    self.x = self.x + match diff_x.abs() {
      1 => diff_x, // assumption if we're moving piece-meal, y must be 2
      2 => sign_x * (sign_x * diff_x - 1),
      _ => 0,
    };
    self.y = self.y + match diff_y.abs() {
      1 => diff_y, // assumption if we're moving piece-meal, x must be 2
      2 => sign_y * (sign_y * diff_y - 1),
      _ => 0,
    };
  }

  pub fn is_adjacent(&self, other: &Point) -> bool {
    (self.x - other.x).abs() < 2 &&
      (self.y - other.y).abs() < 2
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Move {
  direction: Direction,
  amount: u32,
}

impl Direction {
  pub fn new(s: &str) -> Self {
    match s {
      "U" => Direction::Up,
      "D" => Direction::Down,
      "R" => Direction::Right,
      "L" => Direction::Left,
      _ => panic!("unknown direction: {}", s)
    }
  }
}

// some utils for debugging the movement
mod debug {
  use super::*;

  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct Rect {
    bottom_left: Point,
    top_right: Point,
  }

  pub fn get_bounds(commands: &Vec<(Direction, i32)>) -> Rect {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
    let mut point = Point { x: 0, y: 0 };
    for (dir, amount) in commands {
      for _i in 0..*amount {
        point.r#move(dir);
        min_x = min(point.x, min_x);
        max_x = max(point.x, max_x);
        min_y = min(point.y, min_y);
        max_y = max(point.y, max_y);
      }
    }
    Rect {
      bottom_left: Point {
        x: min_x,
        y: min_y,
      },
      top_right: Point {
        x: max_x,
        y: max_y,
      }
    }
  }

  pub fn draw_board(board: &Rect, knots: &Vec<Point>) {
    // let (min, max) = {
    // }
    let knot_len = knots.len();

    println!("knots len: {}", knots.len());

    for y in (board.bottom_left.y..board.top_right.y+1).rev() {
      let mut s = String::new();
      'next_x: for x in board.bottom_left.x..board.top_right.x+1 {
        for i in 0..knots.len() {
          if knots[i].x == x && knots[i].y == y {
            let c = match i {
              0 => 'H',
              // knot_len => 'T',
              _ => char::from_digit(i as u32, 10).unwrap(),
            };
            s.push(c);
            continue 'next_x;
          }
        }
        s.push('.');
      }
      println!("[{}]", s);
    }

  }
}



