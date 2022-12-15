use std::collections::HashSet;
use std::str::Lines;
use std::cmp::{min, max};

use nom::IResult;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map_res};
use nom::sequence::{tuple, pair};

fn main() {
    let input = include_str!("../../inputs/day15.sample.txt");
    impossible_positions_brute(input.lines(), 10);
    let input = include_str!("../../inputs/day15.sample.txt");
    impossible_positions(input.lines(), 10);
    let input = include_str!("../../inputs/day15.txt");
    impossible_positions(input.lines(), 2_000_000);

}

fn impossible_positions(lines: Lines<'_>, pos: i32) -> i32 {
  let pairs: Vec<_> =
    lines.map(|line| {
      let (_, pair) = parse_line(line).unwrap();
      pair
    })
    .collect();

  println!("parsed pairs");

  let (min_corner, max_corner) = get_bounds(&pairs);

  let nearby_pairs =
    pairs.iter().filter(|(s, b)| {
      (s.y < pos) && (s.y + manhattan_distance(s, b) >= pos) ||
      (s.y > pos) && (s.y - manhattan_distance(s, b) <= pos)
    })
    .copied()
    .collect::<Vec<_>>();

  println!("len of pairs around point: {}", nearby_pairs.len());

  let mut impossible_count = 0;
  // let mut filled = HashSet::new();
  // for (sensor, beacon) in &nearby_pairs {
  //   filled.insert(sensor);
  //   filled.insert(beacon);
  // }
  // still brute force; requires us looking at millions of points.
  // we can speed this up by making ranges of points at row 'pos'
  // that would be there, merge overlapping lists, then count the length
  // of each range.
  // println!("checking {} points", max_corner.x - min_corner.x);
  // 'next_x: for x in min_corner.x..max_corner.x+1 {
  //   // Count any space that's a manhattan_distance from a nearby pair
  //   // that isn't filled by a beacon or sensor
  //   for (sensor, beacon) in &nearby_pairs {
  //     if !filled.contains(&Point { x, y: pos }) &&
  //       manhattan_distance(&Point { x, y: pos }, &sensor) <=
  //         manhattan_distance(&sensor, &beacon) {
  //       impossible_count += 1;
  //       continue 'next_x;
  //     }
  //   }
  // }
  let mut ranges: Vec<Range> = vec![];
  for (sensor, beacon) in &nearby_pairs {
    let distance = manhattan_distance(&sensor, &beacon);
    let line_distance = pos - sensor.y;

    ranges.push(Range {
      start: sensor.x - (distance - line_distance.abs()),
      stop: sensor.x + (distance - line_distance.abs()),
    });
  }


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
      // let last_range = keep_ranges.last_mut().unwrap();
      last_range.stop = max(last_range.stop, ranges[current].stop);
    } else {
      // println!("{:?} DOES NOT overlap {:?}", ranges[last], ranges[current]);
      keep_ranges.push(ranges[current]);
    }
    // last = current;
  }

  // dbg!(&keep_ranges);

  impossible_count = keep_ranges
    .iter()
    .map(|r| r.stop - r.start)
    .sum();

  println!("(FAST) impossible positions: {}", impossible_count);

  impossible_count
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

fn impossible_positions_brute(lines: Lines<'_>, pos: i32) -> i32 {
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

  impossible_positions as i32
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
  (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn print_screen(min: &Point, screen: &Vec<Vec<char>>) {
  // let mut line = 0;
  // for x in (min.x)..(min.x+(screen[0].len() as i32) +1) {
  //   if x % 5 == 0 && x < 0 {
  //     buf[x] = '-';
  //   }
  // }
  // let buf = String::from_iter(vec![' '; screen[0].len()]);
  // for x in (min.x)..(min.x+(screen[0].len() as i32) +1) {
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
  x: i32,
  y: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Range {
  start: i32,
  stop: i32,
}

impl Range {
  fn overlaps(&self, other: &Range) -> bool {
    self.start <= other.start && other.start <= self.stop
  }
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

fn parse_number(i: &str) -> IResult<&str, i32> {
  map_res(
    take_while1(|c: char| c.is_ascii_digit() || c == '-'),
    |s: &str| {
      s.parse()
    })(i)
}


