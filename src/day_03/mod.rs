use itertools::Itertools;
use std::collections::HashSet;

fn priority_from_intersection(strs: &[&str]) -> u64 {
  let byte = strs
    .into_iter()
    .map(|raw| HashSet::<char>::from_iter(raw.chars()))
    .reduce(|acc, current| &acc & &current)
    .expect("strs cant be an empty slice")
    .into_iter()
    .next()
    .map(|letter| letter as u8)
    .expect("Failed to find item common to all of input strs");

  let priority = match byte {
    b'a'..=b'z' => byte - b'a' + 1,
    b'A'..=b'Z' => byte - b'A' + 27,
    _ => unreachable!("Char out of range {byte:?}"),
  };

  priority as u64
}

pub fn task_1(raw: &str) -> u64 {
  raw
    .lines()
    .map(|line| line.split_at(line.len() / 2))
    .map(|(a, b)| priority_from_intersection(&[a, b]))
    .sum::<u64>()
}

pub fn task_2(raw: &str) -> u64 {
  raw
    .lines()
    .tuples::<(_, _, _)>()
    .map(|(first, second, third)| priority_from_intersection(&[first, second, third]))
    .sum::<u64>()
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 157,
    task_1_actual = 7917,
    task_2_example = 70,
    task_2_actual = 2585
  );
}
