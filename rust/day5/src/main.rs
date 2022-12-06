use utils::cli::Cli;

use std::str::FromStr;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  print_result("top of stacks 9000", top_of_stacks_9000(&content));
  print_result("top of stacks 9001", top_of_stacks_9001(&content));
}

pub fn top_of_stacks_9000(content: &str) -> Result<String, ParseCommandsError> {
  let (stack_contents, rest) = content.split_once("\n 1").unwrap();
  let mut stacks = parse_stacks(stack_contents);

  let (_, raw_commands) = rest.split_once("\n\n").unwrap();
  raw_commands.lines()
    .for_each(|line| {
      let command = MoveCommand::from_str(line).unwrap();
      // println!("moving {:?}", command);
      for _ in 0..command.amount {
        let item = stacks[command.source as usize].pop().unwrap();
        // println!("moving {} from {} to {}", item, command.source, command.destination);
        stacks[command.destination as usize].push(item);
      }
    });

  let s: String = stacks.iter_mut().map(|s| {
    s.pop().unwrap_or(' ')
  }).collect();
  println!("s: {}", s);

  Ok(s)

}

pub fn top_of_stacks_9001(content: &str) -> Result<String, ParseCommandsError> {
  let (stack_contents, rest) = content.split_once("\n 1").unwrap();
  let mut stacks = parse_stacks(stack_contents);

  let (_, raw_commands) = rest.split_once("\n\n").unwrap();
  raw_commands.lines()
    .for_each(|line| {
      let command = MoveCommand::from_str(line).unwrap();
      // println!("moving {:?}", command);
      // create a temporary package of items in reverse, so that when we copy them
      // over they appear in the same order.
      let mut tmp_items = vec![];
      for _ in 0..command.amount {
        let item = stacks[command.source as usize].pop().unwrap();
        tmp_items.push(item);
      }

      for _ in 0..command.amount {
        let item = tmp_items.pop().unwrap();
        // println!("moving {} from {} to {}", item, command.source, command.destination);
        stacks[command.destination as usize].push(item);
      }
    });

  let s: String = stacks.iter_mut().map(|s| {
    s.pop().unwrap_or(' ')
  }).collect();
  println!("s: {}", s);

  // let line = parse_stack_row("[C] [M] [Z]");

  // let s = line.iter().map(|s| s.unwrap_or(' ')).collect::<String>();
  // println!("s: {}", s);

  Ok(s)

}

fn parse_stacks(stack_contents: &str) -> Vec<Vec<char>> {
  let mut stacks: Vec<Vec<char>> = vec![];
  let mut n_stacks = 0;

  stack_contents.lines().for_each(|line| {
    // println!("parsing stack row: {}", line);
    let row = parse_stack_row(line);
    if stacks.is_empty() {
      n_stacks = parse_stack_row(line).iter().count();
      // println!(" there are {} stacks in line {}", n_stacks, line);
      for _ in 0..n_stacks {
        // println!("pushing empty stack");
        stacks.push(vec![]);
      }
    }
    for i in 0..n_stacks {
      // println!("trying to push into stack {}: row {:?}", i, row);
      if let Some(c) = row[i] {
        // stacks.get_or_insert(i, vec![]).push(c);
        stacks[i].push(c);
      }
    }
  });

  stacks
    .iter_mut()
    .for_each(|s| s.reverse());

  stacks
}

fn parse_stack_row(row: &str) -> Vec<Option<char>> {
  row.chars()
    .collect::<Vec<_>>()
    .chunks(4)
    .map(|s| {
      match s[0] {
        '[' => Some(s[1]),
        _ => None
      }
    }).collect()
}

pub fn print_result(message: &str, r: Result<String, ParseCommandsError>) {
  match r {
    Ok(s) => println!("{} '{}'", message, s),
    Err(e) => println!("Error could not calculate: {}: {}", message, e.message)
  }
}

#[derive(Debug, Clone, Copy)]
struct MoveCommand {
  amount: u32,
  source: u32,
  destination: u32,
}



impl FromStr for MoveCommand {
  type Err = ParseCommandsError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let chunks: Vec<_> = s.split(" ").collect();
    // TODO: error handling
    Ok(
      MoveCommand {
        amount: chunks[1].parse::<u32>().unwrap(),
        source: chunks[3].parse::<u32>().unwrap() - 1,
        destination: chunks[5].parse::<u32>().unwrap() - 1
      }
    )
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCommandsError {
  message: String
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_top_of_stacks() {
    let input = format!("    [D] {}
[N] [C] {}
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2", "   ", "   ");
    assert_eq!(Ok("CMZ".to_string()), top_of_stacks_9000(&input));
    assert_eq!(Ok("MCD".to_string()), top_of_stacks_9001(&input));
  }
}
