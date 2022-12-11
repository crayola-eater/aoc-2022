use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Monkey {
  pub items: VecDeque<usize>,
  pub number_of_items_inspected: usize,
  pub operator: char,
  pub rhs_operand: Option<usize>,
  pub divisor: usize,
  pub next_monkeys: (usize, usize),
}

impl Monkey {
  pub fn try_to_throw(&mut self) -> Option<usize> {
    self.items.pop_front().map(|worry_level| {
      self.number_of_items_inspected += 1;
      worry_level
    })
  }

  pub fn get_next_worry_level(&self, worry_level: usize) -> usize {
    match self.operator {
      '+' => worry_level + self.rhs_operand.unwrap_or(worry_level),
      '*' => worry_level * self.rhs_operand.unwrap_or(worry_level),
      _ => unreachable!("Invalid operator {}", self.operator),
    }
  }

  pub fn get_next_monkey_index(&self, worry_level: usize) -> usize {
    if 0 == (worry_level % self.divisor) {
      self.next_monkeys.0
    } else {
      self.next_monkeys.1
    }
  }

  pub fn catch(&mut self, worry_level: usize) {
    self.items.push_back(worry_level);
  }
}

pub fn get_monkey_business(monkeys: &[Monkey]) -> usize {
  monkeys
    .iter()
    .map(|monkey| monkey.number_of_items_inspected)
    .fold([Option::<usize>::None; 2], |[first, second], current| {
      if first.is_none() || Some(current) > first {
        [Some(current), first]
      } else if second.is_none() || Some(current) > second {
        [first, Some(current)]
      } else {
        [first, second]
      }
    })
    .iter()
    .map(|opt| opt.unwrap())
    .product()
}

fn parse_one_monkey(raw: &str) -> Monkey {
  let raw_chunks = raw
    .lines()
    .next_tuple::<(_, _, _, _, _, _)>()
    .expect("Invalid information for monkey");

  let items = parse_starting_items(raw_chunks.1);
  let (operator, rhs_operand) = parse_operation(raw_chunks.2);
  let (divisor, next_monkeys) = parse_test(raw_chunks.3, raw_chunks.4, raw_chunks.5);

  Monkey {
    items,
    operator,
    rhs_operand,
    divisor,
    next_monkeys,
    number_of_items_inspected: 0,
  }
}

pub fn parse_monkeys(raw: &str) -> Vec<Monkey> {
  raw.split("\n\n").map(parse_one_monkey).collect()
}

fn parse_starting_items(raw: &str) -> VecDeque<usize> {
  raw[18..]
    .split(", ")
    .map(|item| item.parse::<usize>().expect("Invalid item"))
    .collect()
}

fn parse_operation(raw: &str) -> (char, Option<usize>) {
  let (operator, operand) = raw[23..]
    .split_ascii_whitespace()
    .next_tuple::<(_, _)>()
    .unwrap();

  let operator = operator.chars().next().unwrap();
  let operand = operand.parse::<usize>().ok();

  (operator, operand)
}

fn parse_test(test: &str, if_true: &str, if_false: &str) -> (usize, (usize, usize)) {
  (
    test[21..].parse::<usize>().unwrap(),
    (
      if_true[29..].parse::<usize>().unwrap(),
      if_false[30..].parse::<usize>().unwrap(),
    ),
  )
}
