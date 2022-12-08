use std::{str::Lines, iter::Peekable, any::Any};
use utils::cli::Cli;

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  let tree = build_tree(&mut content.lines().peekable());
  println!("part1: {}", part1(&tree));
  println!("part2: {:?}", part2(&tree));
}

fn part1(tree: &Dir) -> usize {
  let sizes = calculate_dir_sizes(tree);

  sizes
    .iter()
    .filter_map(|s| {
      if s.1 < 100000 { Some(s.1) } else { None }
    })
    .sum()
}

fn part2(tree: &Dir) -> (String, usize) {
  let mut sizes = calculate_dir_sizes(tree);
  let root_size = *sizes.last().map(|(_, size)| size).unwrap_or(&0);

  let disk_size = 70000000;
  let remaining_space = disk_size - root_size;
  let needed_space = 30000000;
  // println!("remaining space: {}. Need {}", remaining_space, needed_space - remaining_space);

  sizes.sort_by(|(_, size1), (_, size2)| {
    size1.cmp(size2)
  });
  let smallest_dir_big_enough = sizes
    .into_iter()
    // .inspect(|(name, size)| {
    //   println!("trying {} {}", name, size);
    // })
    .find(|(_, size)| {
      needed_space < remaining_space + size
    });
  smallest_dir_big_enough.unwrap()
}

fn calculate_dir_sizes(tree: &Dir) -> Vec<(String, usize)> {
  let mut sizes = vec![];
  let mut size: usize = 0;
  for entry in &tree.entries {
    if let Some(dir) = entry.as_ref().as_any().downcast_ref::<Dir>() {
      let mut dir_sizes = calculate_dir_sizes(dir);
      let dir_size = *dir_sizes.last().map(|(_, size)| size).unwrap_or(&0);
      // println!("Adding {} with size {}", entry.get_name(), dir_size);
      sizes.append(&mut dir_sizes);
      size = size + dir_size;
    } else {
      size = size + entry.get_size();
    }
  }

  sizes.push((tree.get_name().to_string(), size));
  sizes
}

fn build_tree(lines: &mut Peekable<Lines<'_>>) -> Dir {
  let mut dir = Dir {
    name: {
        let l = lines.next().unwrap().split(" ");
        l.skip(2).next().unwrap().to_string()
    },
    entries: vec![],
  };
  loop {
    let peek = lines.peek();
    if peek == None {
      break;
    }

    if peek.map(|s| s.starts_with("$")).or(Some(false)).unwrap() {
      let (command, args) = {
        let mut l = peek.unwrap().split(" ");
        l.next();
        (l.next().unwrap(), l.next())
      };
      match command {
        "cd" if Some("..") == args => {
          lines.next();
          break;
        }
        "cd" => {
          dir.entries.push(Box::from(build_tree(lines)));
        }
        "ls" => {
          parse_ls(lines).into_iter().for_each(|l| {
            dir.entries.push(l);
          });
        }
        _ => panic!("unrecognized command: {}", command)
      }
    }
  }

  // println!("built tree: {}", dir.get_name());

  dir
}

fn parse_ls(lines: &mut Peekable<Lines<'_>>) -> Vec<Box<dyn DirEntry>> {
  let mut entries: Vec<Box<dyn DirEntry>> = vec![];
  lines.next();
  loop {
    if let Some(true) = lines.peek().map(|s| s.starts_with("$")).or(Some(false)) {
      break;
    }
    match lines.next() {
      Some(line) => {
        // println!("parsing {line}");
        let parts = line.split_once(" ").unwrap();
        match parts.0 {
          "dir" => (),
          _ => {
            entries.push(Box::new(File {
              name: parts.1.to_string(),
              size: parts.0.parse::<usize>().unwrap(),
            }));
          }
        }
      },
      None => break,
    }
  }

  entries
}

// A special trait that allows us to convert to Any, so that we can downcast.
pub trait AToAny: 'static {
  fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
  fn as_any(&self) -> &dyn Any {
      self
  }
}

#[derive(Debug)]
struct File {
  name: String,
  size: usize
}

trait DirEntry: AToAny {
  fn get_name(&self) -> &str;
  fn get_size(&self) -> usize;
  fn is_dir(&self) -> bool;
}

struct Dir {
  name: String,
  entries: Vec<Box<dyn DirEntry>>,
}

impl DirEntry for File {
  fn get_size(&self) -> usize {
    self.size
  }
  fn get_name(&self) -> &str {
    &self.name
  }
  fn is_dir(&self) -> bool {
    false
  }
}

impl DirEntry for Dir {
  fn get_size(&self) -> usize {
    self.entries
      .iter()
      .map(|e| e.get_size()).sum()
  }
  fn get_name(&self) -> &str {
    &self.name
  }
  fn is_dir(&self) -> bool {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_get_dirs_under_100k() {
    let root = build_tree(&mut "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".lines().peekable());
    assert_eq!(95437, part1(&root));
  }

}
