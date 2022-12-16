use std::str::Lines;
use std::cmp::{min, max};

use nom::IResult;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map_res};
use nom::sequence::{tuple};

fn main() {
    let input = include_str!("../../inputs/day15.sample.txt");
    impossible_positions_brute(input.lines(), 10);
    let input = include_str!("../../inputs/day15.sample.txt");
    impossible_positions(input.lines(), 10);
    let input = include_str!("../../inputs/day15.txt");
    impossible_positions(input.lines(), 2_000_000);

    println!("***** part 2");

    let input = include_str!("../../inputs/day15.sample.txt");
    if let Some(p) = find_open_position(input.lines(), 20) {
      println!("in sample, found open point: {:?} tuning frequency {}", p, p.x * 4_000_000 + p.y);
    }

    let input = include_str!("../../inputs/day15.txt");
    if let Some(p) = find_open_position(input.lines(), 4_000_000) {
      println!("in real input, found open point: {:?} tuning frequency {}", p, p.x * 4_000_000 + p.y);
    }
}

fn impossible_positions(lines: Lines<'_>, pos: i64) -> i64 {
  // Improve on the brute force version, by just looking for ranges of
  // points that must be filled in. For a given sensor, beacon, and position line,
  // we can calculate the range by subtracting the distance from S to P from S's
  // manhattan distance to B, so we end up with a range of [s.x-leftover, s.x+leftover]
  //
  //    .....S.....
  //    .....#.....
  // P  .#########.
  //    ...........
  //    .........B.
  // Then we just have to merge the ranges, and add up the range lengths to figure
  // how many positions another beacon cannot be in.
  let pairs: Vec<_> = parse_lines(lines);

  let mut ranges: Vec<Range> = vec![];
  for (sensor, beacon) in &pairs {
    let distance = manhattan_distance(&sensor, &beacon);
    let line_distance = pos - sensor.y;
    if line_distance.abs() > distance {
      continue;
    }

    ranges.push(Range {
      start: sensor.x - (distance - line_distance.abs()),
      stop: sensor.x + (distance - line_distance.abs()),
    });
  }

  let ranges = merge_ranges(&ranges);

  // dbg!(&keep_ranges);

  let impossible_count = ranges
    .iter()
    .map(|r| r.stop - r.start)
    .sum();

  println!("(FAST) impossible positions: {}", impossible_count);

  impossible_count
}

fn find_open_position(lines: Lines<'_>, max_side: i64) -> Option<Point> {
  let pairs: Vec<_> = parse_lines(lines);

  let x_range = Range { start: 0, stop: max_side };
  for y in 0..max_side {
    if let Some(x) = open_position(&pairs, y, &x_range) {
      return Some(Point { x, y });
    }
  }
  return None;
}

fn open_position(pairs: &Vec<(Point, Point)>, pos: i64, x_range: &Range) -> Option<i64> {
  let mut ranges: Vec<Range> = vec![];
  for (sensor, beacon) in pairs {
    let distance = manhattan_distance(&sensor, &beacon);
    let line_distance = pos - sensor.y;
    if line_distance.abs() > distance {
      continue;
    }

    ranges.push(Range {
      start: sensor.x - (distance - line_distance.abs()),
      stop: sensor.x + (distance - line_distance.abs()),
    });
  }

  let ranges = merge_ranges(&ranges);

  // dbg!(&ranges);
  match ranges.len() {
    1 => None,
    2 => Some(ranges[0].stop + 1),
    _ => panic!("expected 1 or 2 ranges, not {}", ranges.len())
  }
}

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
  let mut ranges = ranges.clone();
  let mut keep_ranges = vec![];
  ranges.sort_by(|a, b| a.start.cmp(&b.start));
  // dbg!(&ranges);

  let mut indices = 0..ranges.len();
  let last = indices.next().unwrap();
  keep_ranges.push(ranges[last]);
  while let Some(current) = indices.next() {
    // println!("looking at range {}", current);
    let last_range = keep_ranges.last_mut().unwrap();
    if last_range.overlaps(&ranges[current]) {
      // println!("{:?} overlaps {:?}", ranges[last], ranges[current]);
      last_range.stop = max(last_range.stop, ranges[current].stop);
    } else {
      // println!("{:?} DOES NOT overlap {:?}", ranges[last], ranges[current]);
      keep_ranges.push(ranges[current]);
    }
  }

  keep_ranges
}

fn get_bounds(pairs: &Vec<(Point, Point)>) -> (Point, Point) {
  let mut min_corner = Point { x: 1_000_000_000, y: 1_000_000_000 };
  let mut max_corner = Point { x: 0, y: 0};
  for (sensor, beacon) in pairs {
    min_corner.y = min(min_corner.y, min(sensor.y - manhattan_distance(&sensor, &beacon), beacon.y));
    min_corner.x = min(min_corner.x, min(sensor.x - manhattan_distance(&sensor, &beacon), beacon.x));
    max_corner.y = max(max_corner.y, max(sensor.y + manhattan_distance(&sensor, &beacon), beacon.y));
    max_corner.x = max(max_corner.x, max(sensor.x + manhattan_distance(&sensor, &beacon), beacon.x));
  }
  (min_corner, max_corner)
}

fn impossible_positions_brute(lines: Lines<'_>, pos: i64) -> i64 {
  let pairs: Vec<_> =
    lines.map(|line| {
      let (_, pair) = parse_line(line).unwrap();
      pair
    })
    .collect();

  println!("parsed pairs");

  let (min_corner, max_corner) = get_bounds(&pairs);
  println!("found corners: {:?} {:?}", min_corner, max_corner);

  // account for possible negative coordinates in our min_corner by
  // possibly translating them for screen coordinates
  let (tran_x, tran_y) = (
    if min_corner.x < 0 { min_corner.x.abs() } else { 0 },
    if min_corner.y < 0 { min_corner.y.abs() } else { 0 },
  );
  let mut screen = vec![vec!['.'; (max_corner.x - min_corner.x).abs() as usize + 1]; (max_corner.y - min_corner.y).abs() as usize + 1];
  println!("built screen");

  for (sensor, beacon) in &pairs {
    // println!("filling in S={:?} B={:?}", sensor, beacon);
    screen[(sensor.y + tran_y) as usize][(sensor.x + tran_x) as usize] = 'S';
    screen[(beacon.y + tran_y) as usize][(beacon.x + tran_x) as usize] = 'B';
    // fill in areas other beacons cannot be.
    let distance = manhattan_distance(&sensor, &beacon);
    for y in (sensor.y-distance)..(sensor.y+distance+1) {
      for x in (sensor.x-distance)..(sensor.x+distance+1) {
        // if sensor.x != 8 || sensor.y != 7 {
        //   continue;
        // }
        if manhattan_distance(&Point { x, y }, &sensor) > distance {
          continue;
        }
        // if x < min_corner.x || x >= max_corner.x || y < min_corner.y || y >= max_corner.y {
        //   continue;
        // }
        let (y, x) = ((y + tran_y) as usize, (x + tran_x) as usize);
        if screen[y][x] == '.' {
          screen[y][x] = '#';
        }
      }
    }
  }

  print_screen(&min_corner, &screen);

  let line = (pos + tran_y) as usize;
  // println!("{}", String::from_iter(screen[line].to_vec()));
  let impossible_positions = screen[line]
    .iter()
    // .filter(|c| **c == '#' || **c == 'B' || **c == 'S')
    .filter(|c| **c == '#')
    .count();

  println!("impossible positions: {}", impossible_positions);

  impossible_positions as i64
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
  (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn print_screen(min: &Point, screen: &Vec<Vec<char>>) {
  // let mut line = 0;
  // for x in (min.x)..(min.x+(screen[0].len() as i64) +1) {
  //   if x % 5 == 0 && x < 0 {
  //     buf[x] = '-';
  //   }
  // }
  // let buf = String::from_iter(vec![' '; screen[0].len()]);
  // for x in (min.x)..(min.x+(screen[0].len() as i64) +1) {
  //   if x % 5 == 0 && x < 0 {
  //     buf[x] = '-';
  //   }
  // }
  let mut line = min.y;
  for y in 0..screen.len() {
    println!("{:4} {}", line, String::from_iter(screen[y].to_vec()));
    line += 1;
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
  x: i64,
  y: i64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Range {
  start: i64,
  stop: i64,
}

impl Range {
  fn overlaps(&self, other: &Range) -> bool {
    self.start <= other.start && other.start <= self.stop
  }
}

fn parse_lines(lines: Lines<'_>) -> Vec<(Point, Point)> {
  let pairs: Vec<_> =
    lines.map(|line| {
      let (_, pair) = parse_line(line).unwrap();
      pair
    })
    .collect();

  println!("parsed pairs");
  pairs
}

// Sensor at x=9, y=16: closest beacon is at x=10, y=16
fn parse_line(input: &str) -> IResult<&str, (Point, Point)> {
  let (input, _) = tag("Sensor at ")(input)?;
  let (input, sensor) = parse_point(input)?;
  let (input, _) = tag(": closest beacon is at ")(input)?;
  let (input, beacon) = parse_point(input)?;
  Ok(
    (input, (sensor, beacon))
  )
}

fn parse_point(input: &str) -> IResult<&str, Point> {
  let (input, (_, x, _, y)) = tuple(
    (tag("x="), parse_number, tag(", y="), parse_number)
  )(input)?;
  Ok((input, Point { x, y }))
}

fn parse_number(i: &str) -> IResult<&str, i64> {
  map_res(
    take_while1(|c: char| c.is_ascii_digit() || c == '-'),
    |s: &str| {
      s.parse()
    })(i)
}


