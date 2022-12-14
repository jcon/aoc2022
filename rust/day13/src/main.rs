use std::{cmp::Ordering};

fn main() {
  part1(include_str!("../../inputs/day13.sample.txt"));
  part1(include_str!("../../inputs/day13.txt"));
}

fn part1(input: &str) {
  let mut sum = 0;
  for (i, pairs) in input.split("\n\n").enumerate() {
    let i = i + 1;

    let mut nodes = pairs
      .lines()
      .map(|line| serde_json::from_str::<Node>(line).unwrap());
    let l = nodes.next().unwrap();
    let r = nodes.next().unwrap();
    // println!("compare(l, r) = {:?}", compare(&l, &r));
    if compare(&l, &r) == Ordering::Less {
      sum += i;
    }
  }
  dbg!(sum);
}

fn compare(left: &Node, right: &Node) -> Ordering {
  match (left, right) {
    (Node::Item(l), Node::Item(r)) => l.cmp(r),
    (Node::Item(_), r) => compare(&Node::Nested(vec![left.clone()]), r),
    (l, Node::Item(_)) => compare(l, &Node::Nested(vec![right.clone()])),
    (Node::Nested(l), Node::Nested(r)) => {
      let (mut l, mut r) = (l.iter(), r.iter());
      loop {
        match (l.next(), r.next()) {
          (Some(l), Some(r)) => {
            let c = compare(l, r);
            if c != Ordering::Equal {
              return c;
            }
          }
          (None, None) => return Ordering::Equal,
          (None, _) => return Ordering::Less,
          (_, None) => return Ordering::Greater,
          _ => todo!()
        }
      }
    },
  }
}

#[derive(serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
  Item(i32),
  Nested(Vec<Node>),
}

impl std::fmt::Debug for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Item(i) => f.debug_tuple("Item").field(i).finish(),
      Self::Nested(n) => f.debug_list().entries(n).finish(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_cases() {
    assert_eq!(Ordering::Less, compare(&Node::Item(1), &Node::Item(2)));
    assert_eq!(Ordering::Greater, compare(&Node::Item(10), &Node::Item(6)));
    assert_eq!(Ordering::Equal, compare(&Node::Item(8), &Node::Item(8)));

    assert_eq!(Ordering::Less, compare(&Node::Item(1), &Node::Nested(vec![Node::Item(2)])));
    let list1 = Node::Nested(vec![Node::Item(1), Node::Item(2)]);
    let list2 = Node::Nested(vec![Node::Item(1), Node::Item(2), Node::Item(3)]);
    let list3 = Node::Nested(vec![Node::Item(1), Node::Item(2)]);
    assert_eq!(Ordering::Less, compare(&list1, &list2));
    assert_eq!(Ordering::Greater, compare(&list2, &list1));
    assert_eq!(Ordering::Equal, compare(&list1, &list3));
  }

  #[test]
  pub fn test_sample() {
    let input = include_str!("../../inputs/day13.sample.txt");
    let left = Node::Nested(
      vec![
        Node::Nested(
          vec![
            Node::Item(1)
          ]
        ),
        Node::Nested(
          vec![
            Node::Item(2),
            Node::Item(3),
            Node::Item(4),
          ]
        )
      ]
    );
    let right = Node::Nested(
      vec![
        Node::Nested(
          vec![
            Node::Item(1)
          ]
        ),
        Node::Item(4)
      ]
    );
    assert_eq!(Ordering::Less, compare(&left, &right));
  }
}