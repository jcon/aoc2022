
use std::str::FromStr;

use color_eyre::eyre::Context;
use utils::cli::Cli;

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).wrap_err("while reading file")?;
  let instructions = content
    .lines()
    .map(|l| Instruction::from_str(l).unwrap());
  let mut cycles = 0;
  let mut x_register = 1;
  let mut signals: Vec<i32> = vec![];
  let mut crt: Vec<char> = vec!['.'; 40*6];
  let mut crt_pos = 0;
  for instruction in instructions {
    let mut instruction_cycles = instruction.cycle_length();
    println!("cycle length: {}", instruction_cycles);
    for i in (0..instruction_cycles).rev() {
      cycles = cycles + 1;
      if (cycles - 20) % 40 == 0 {
        signals.push(x_register * cycles);
      }
      let x_pos = (crt_pos as i32) % 40;
      crt[crt_pos] = if ((x_register - 1)..(x_register + 2)).contains(&x_pos) {
        '#'
      } else {
        '.'
      };
      crt_pos = crt_pos + 1;
      match instruction {
        Instruction::Noop => (),
        Instruction::AddX(amount) => {
          if i == 0 {
            x_register = x_register + amount;
          }
        }
      }
      println!("cycle {} x: {}", cycles, x_register);
    }
  }
  dbg!(&signals);
  println!("signal stregnth: {}", signals.iter().sum::<i32>());
  for crt_line in crt.chunks(40) {
    println!("{}", crt_line.iter().collect::<String>());
  }
  Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
  Noop,
  AddX(i32),
}

impl Instruction {
  pub fn cycle_length(&self) -> i32 {
    match self {
      Self::Noop => 1,
      Self::AddX(_) => 2,
    }
  }
}

impl FromStr for Instruction {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.split_once(" ") {
        Some((i, a)) => Ok(Self::AddX(a.parse::<i32>().unwrap())),
        _ if s == "noop" => Ok(Self::Noop),
        _ => panic!("can't understand instruction"),
      }
    }
}
