use std::str::Lines;
use std::cmp::max;
use utils::cli::Cli;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");

  println!("visible trees: {}", count_visible_trees(content.lines()));
  println!("highest scenic score: {}", highest_scenic_score(content.lines()));
}

pub fn count_visible_trees(lines: Lines<'_>) -> i32 {
  let grid = parse(lines);
  let mut board: Vec<Vec<bool>> = grid.iter().map(|_| vec![false; grid[0].len()]).collect();
  // dbg!(&grid, &board);

  for i in 0..grid.len() {
    let mut highest = -1;
    for j in 0..grid[i].len() {
      board[i][j] = (grid[i][j] > highest) | board[i][j];
      highest = max(highest, grid[i][j]);
    }
    highest = -1;
    for j in (0..grid[i].len()).rev() {
      board[i][j] = (grid[i][j] > highest) | board[i][j];
      highest = max(highest, grid[i][j]);
    }
  }
  for j in 0..grid[0].len() {
    let mut highest = -1;
    for i in 0..grid.len() {
      board[i][j] = (grid[i][j] > highest) | board[i][j];
      highest = max(highest, grid[i][j]);
    }
    highest = -1;
    for i in (0..grid.len()).rev() {
      board[i][j] = (grid[i][j] > highest) | board[i][j];
      highest = max(highest, grid[i][j]);
    }
  }

  board
    .iter()
    .flatten()
    .filter(|i| **i)
    .count() as i32
}

pub fn highest_scenic_score(lines: Lines<'_>) -> i32 {
  let grid = parse(lines);
  let mut board: Vec<Vec<i32>> = grid.iter().map(|_| vec![0; grid[0].len()]).collect();

  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      board[i][j] = 1;
      let mut score = 0;
      for col in (0..j).rev() {
        score = score + 1;
        if grid[i][col] >= grid[i][j] {
          break;
        }
      }
      board[i][j] = board[i][j] * score;
      score = 0;
      for col in (j + 1)..grid[i].len() {
        score = score + 1;
        if grid[i][col] >= grid[i][j] {
          break;
        }
      }
      board[i][j] = board[i][j] * score;
      score = 0;
      for row in (0..i).rev() {
        score = score + 1;
        if grid[row][j] >= grid[i][j] {
          break;
        }
      }
      board[i][j] = board[i][j] * score;
      score = 0;
      for row in (i+1)..grid.len() {
        score = score + 1;
        if grid[row][j] >= grid[i][j] {
          break;
        }
      }
      board[i][j] = board[i][j] * score;
    }
  }

  // dbg!(&board);

  board
    .iter()
    .flatten()
    .copied()
    .max()
    .unwrap()
  // dbg!(&board);

}

// fn traverse(grid: &Vec<Vec<i32>>, board: &mut Vec<Vec<bool>>, i_range: &Vec<usize>, j_range: &Vec<usize>) {
//   let mut highest = -1;
//   // for i in i_range.0..i_range.1 {
//   for i in i_range.clone() {
//     for j in j_range.clone() {
//       board[i][j] = (grid[i][j] > highest) | board[i][j];
//       highest = max(highest, grid[i][j]);
//     }
//   }
// }

fn parse(lines: Lines<'_>) -> Vec<Vec<i32>> {
  lines
    .into_iter()
    .map(|s| s
      .chars()
      .map(|c| c.to_digit(10).unwrap() as i32)
      .collect())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_visible_trees() {
    assert_eq!(21, count_visible_trees(
"30373
25512
65332
33549
35390".lines()));
  }
  #[test]
  pub fn test_scenic_score() {
    assert_eq!(8, highest_scenic_score(
"30373
25512
65332
33549
35390".lines()));
  }

  #[test]
  pub fn test_vec_addressing() {
    assert_eq!(8, vec![0,1,2,3,4,5,6,7,8][9 - 1]);
  }
}
