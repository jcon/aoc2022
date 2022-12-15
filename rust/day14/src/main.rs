use std::cmp::{min, max};
use std::iter::FromIterator;
use std::str::{Lines};
use nom::IResult;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map_res, opt};
use nom::multi::many1;
use nom::sequence::tuple;

fn main() {
  println!("Hello, world!");
  // let input = include_str!("../../inputs/day14.sample.txt");
  let input = include_str!("../../inputs/day14.txt");
  min_units_for_infinite_flow(input.lines());
  min_units_until_full(input.lines());
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

fn min_units_for_infinite_flow(lines: Lines<'_>) {
  let paths = parse_input(lines).unwrap();
  let (top_left, bottom_right) = get_bounds(&paths);

  let mut screen = vec![vec!['.'; bottom_right.x+1]; bottom_right.y+1];

  // Draw spigot to on screen
  screen[0][500] = '+';

  draw_paths(&mut screen, &paths);

  print_screen(&top_left, &screen);

  // simulate falling sand
  let mut sand_units = 0;
  loop {
    let mut y = 0;
    let mut x = 500;
    while y < bottom_right.y {
      if is_blocked(screen[y+1][x]) {
        if is_blocked(screen[y+1][x-1]) {
          if is_blocked(screen[y+1][x+1]) {
            screen[y][x] = 'o';
            break;
          } else {
            x += 1;
          }
        } else {
          x -= 1;
        }
      }
      y += 1;
    }
    if y == bottom_right.y {
      break;
    }
    sand_units += 1;
  }
  print_screen(&top_left, &screen);
  println!("sand units before abyss: {}", sand_units);
}

fn min_units_until_full(lines: Lines<'_>) {
  let paths = parse_input(lines).unwrap();
  let (mut top_left, mut bottom_right) = get_bounds(&paths);
  bottom_right.y += 2;
  top_left.x -= 10;

  let mut screen = vec![vec!['.'; bottom_right.x+1]; bottom_right.y+2];

  // Draw spigot to on screen
  screen[0][500] = '+';

  draw_paths(&mut screen, &paths);
  for x in 0..bottom_right.x+1 {
    screen[bottom_right.y][x] = '#';
  }

  print_screen(&top_left, &screen);

  // simulate falling sand
  let mut sand_units = 0;
  'is_full: loop {
    let mut y = 0;
    let mut x = 500;
    while y < bottom_right.y {
      if screen[y][x] == 'o' {
        break 'is_full;
      }
      // grow the screen if we need to explore further to the right.
      if x+1 >= screen[y+1].len() {
        let grow_amount = 10;
        bottom_right.x += grow_amount;
        for line in &mut screen {
          line.extend_from_slice(&vec!['.'; grow_amount]);
        }
        // extend infinite floor
        for x in bottom_right.x-grow_amount..bottom_right.x+1 {
          screen[bottom_right.y][x] = '#';
        }
      }
      // grow the visible window to the left if needed.
      if top_left.x > x {
        top_left.x = x;
      }
      if is_blocked(screen[y+1][x]) {
        if is_blocked(screen[y+1][x-1]) {
          if is_blocked(screen[y+1][x+1]) {
            screen[y][x] = 'o';
            break;
          } else {
            x += 1;
          }
        } else {
          x -= 1;
        }
      }
      y += 1;
    }
    if y == bottom_right.y {
      break;
    }
    sand_units += 1;
  }
  print_screen(&top_left, &screen);
  println!("sand units until full: {}", sand_units);
}

fn is_blocked(c: char) -> bool {
  c == '#' || c == 'o'
}

fn print_screen(top_left: &Point, screen: &Vec<Vec<char>>) {
  let mut line = 0_usize;
  for y in 0..screen.len() {
    // the screen can get wide, so only show the visible area.
    let s = String::from_iter(screen[y].to_vec());
    println!("{:2}: {}", line, &s[top_left.x-1..]);
    line += 1;
  }
}

fn draw_paths(screen: &mut Vec<Vec<char>>, paths: &Vec<Vec<Point>>) {
  for path in paths {
    let mut start = &path[0];
    for point in &path[1..] {
      let (start_y, stop_y) = get_range(start.y, point.y);
      for y in start_y..stop_y {
        let (start_x, stop_x) = get_range(start.x, point.x);
        for x in start_x..stop_x {
          screen[y][x] = '#';
        }
      }
      start = point;
    }
  }
}

fn get_bounds(paths: &Vec<Vec<Point>>) -> (Point, Point) {
  let (mut min_x, mut max_x) = (1_000_000_000, 0);
  let (mut min_y, mut max_y) = (1_000_000_000, 0);
  for path in paths {
    for point in path {
      min_x = min(min_x, point.x);
      max_x = max(max_x, point.x);
      min_y = min(min_y, point.y);
      max_y = max(max_y, point.y);
    }
  }
  (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
}

fn get_range(a: usize, b: usize) -> (usize, usize) {
  if a > b {
    (b, a+1)
  } else {
    (a, b+1)
  }
}

fn parse_input(lines: Lines<'_>) -> Option<Vec<Vec<Point>>> {
  let paths = lines.map(|line| {
    let (_, points) = parse_path(line).unwrap();
    points
  }).collect::<Vec<_>>();
  Some(paths)
}

fn parse_path(input: &str) -> IResult<&str, Vec<Point>> {
  let (input, points) = many1(parse_point)(input)?;
  Ok((input, points))
}

fn parse_point(input: &str) -> IResult<&str, Point> {
  let (input, (x, _, y)) = tuple(
    (parse_number, tag(","), parse_number)
  )(input)?;
  let (input, _) = opt(tag(" -> "))(input)?;

  Ok((input, Point { x, y }))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
  map_res(
    take_while1(|c: char| c.is_ascii_digit()),
    |s: &str| {
      s.parse()
    })(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_parse_path() {
    let (_, path) = parse_path("457,106 -> 457,99 -> 457,106 -> 459,106").unwrap();
    assert_eq!(path, vec![
      Point { x: 457, y: 106 },
      Point { x: 457, y: 99 },
      Point { x: 457, y: 106 },
      Point { x: 459, y: 106 },
    ]);
  }
}