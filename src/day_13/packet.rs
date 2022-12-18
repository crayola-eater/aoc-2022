use std::cmp::Ordering;

#[derive(Debug)]
pub enum Packet {
  Number(u32),
  List(Vec<Self>),
}

impl Ord for Packet {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::Number(left), Self::Number(right)) => left.cmp(right),
      (Self::List(left), Self::List(right)) => left.cmp(right),
      (Self::Number(left), Self::List(_)) => Self::List(vec![Self::Number(*left)]).cmp(other),
      (Self::List(_), Self::Number(right)) => self.cmp(&Self::List(vec![Self::Number(*right)])),
    }
  }
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Packet {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other).is_eq()
  }
}

impl Eq for Packet {}

impl Packet {
  pub fn new(raw: &str) -> Self {
    if let Ok(number) = raw.parse::<u32>() {
      Self::Number(number)
    } else if "[]" == raw {
      Self::List(Vec::new())
    } else if raw.starts_with('[') && raw.ends_with(']') {
      let mut depth = 0;

      let packets = raw
        .strip_prefix('[')
        .unwrap()
        .strip_suffix(']')
        .unwrap()
        .split(move |ch| {
          ch == ',' && depth == 0 || {
            if '[' == ch {
              depth += 1
            } else if ']' == ch {
              depth -= 1;
            };

            false
          }
        })
        .map(Packet::new)
        .collect::<Vec<_>>();

      Self::List(packets)
    } else {
      panic!("Invalid parser input: {raw}");
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  use Packet::*;

  #[test]
  fn test_new_empty_vec() {
    let actual = Packet::new("[]");
    let expected = List(vec![]);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_new_one_number() {
    let actual = Packet::new("[45]");
    let expected = List(vec![Number(45)]);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_two_numbers() {
    let actual = Packet::new("[1,3]");
    let expected = List(vec![Number(1), Number(3)]);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_empty_nested_list() {
    let actual = Packet::new("[0,[]]");
    let expected = List(vec![Number(0), List(vec![])]);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_triple_empty_nested_list() {
    let actual = Packet::new("[[[]]]");
    let expected = List(vec![List(vec![List(vec![])])]);
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_list_number_list() {
    let actual = Packet::new("[[1,2,3],4,[5,6,7]]");
    let expected = List(vec![
      List(vec![Number(1), Number(2), Number(3)]),
      Number(4),
      List(vec![Number(5), Number(6), Number(7)]),
    ]);
    assert_eq!(actual, expected);
  }
}
