fn parse_top_half(raw: &str) -> Vec<Vec<char>> {
  let mut lines = raw.lines().rev();
  let iter = lines.by_ref();
  let width = iter.next().unwrap().split_ascii_whitespace().count();

  iter.fold(vec![vec![]; width], |mut stacks, line| {
    (0..width)
      .map(|n| (4 * n) + 1)
      .map(|index| line.chars().nth(index).unwrap())
      .enumerate()
      .filter(|(_, letter)| letter.is_ascii_alphabetic())
      .for_each(|(column, letter)| {
        stacks[column].push(letter);
      });

    stacks
  })
}

fn parse_bottom_half(raw: &str) -> impl Iterator<Item = [usize; 3]> + '_ {
  raw.lines().map(|line| {
    let mut iter = line.split_ascii_whitespace();

    match (
      iter.next(),
      iter.next().and_then(|raw| raw.parse::<usize>().ok()),
      iter.next(),
      iter.next().and_then(|raw| raw.parse::<usize>().ok()),
      iter.next(),
      iter.next().and_then(|raw| raw.parse::<usize>().ok()),
    ) {
      (Some("move"), Some(count), Some("from"), Some(from), Some("to"), Some(to)) => {
        [count, from - 1, to - 1]
      }
      _ => unreachable!("Unexpected line {line}"),
    }
  })
}

pub fn task_1(raw: &str) -> String {
  let (a, b) = raw
    .split_once("\n\n")
    .expect("No separator between top and bottom halves");

  let mut stacks = parse_top_half(a);

  for [count, from, to] in parse_bottom_half(b) {
    for _ in 0..count {
      let item = stacks[from].pop().expect("Popped empty stack");
      stacks[to].push(item);
    }
  }

  stacks
    .into_iter()
    .map(|stack| *stack.last().unwrap())
    .collect()
}

pub fn task_2(raw: &str) -> String {
  let (top_half, bottom_half) = raw
    .split_once("\n\n")
    .expect("No separator between top and bottom halves");

  let mut stacks = parse_top_half(top_half);

  for [count, from, to] in parse_bottom_half(bottom_half) {
    // this can be simplified by using split at mut
    let (from_stack, to_stack) = {
      let mut iter = stacks
        .iter_mut()
        .enumerate()
        .filter(|(i, _)| i == &from || i == &to);

      match (iter.next().unwrap(), iter.next().unwrap()) {
        ((a, a_stack), (b, b_stack)) if a == from && b == to => (a_stack, b_stack),
        ((a, a_stack), (b, b_stack)) if a == to && b == from => (b_stack, a_stack),
        _ => unreachable!(" no stacks matching {from} and {to}"),
      }
    };

    let iter = {
      let new_length = from_stack.len() - count;
      from_stack.drain(new_length..)
    };

    to_stack.extend(iter);
  }

  stacks
    .into_iter()
    .map(|stack| *stack.last().unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = "CMZ".to_string(),
    task_1_actual = "WHTLRMZRC".to_string(),
    task_2_example = "MCD".to_string(),
    task_2_actual = "GMPMLWNMG".to_string()
  );
}
