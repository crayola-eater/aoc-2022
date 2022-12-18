mod packet;
use itertools::Itertools;
use packet::Packet;

pub fn task_1(raw: &str) -> usize {
  raw
    .split("\n\n")
    .enumerate()
    .filter_map(|(i, group)| {
      let (left, right) = group
        .lines()
        .map(Packet::new)
        .next_tuple::<(_, _)>()
        .unwrap();

      if left < right {
        Some(i + 1)
      } else {
        None
      }
    })
    .sum::<usize>()
}

pub fn task_2(raw: &str) -> usize {
  let [(_, a), (_, b)] = raw
    .lines()
    .filter(|line| !line.is_empty())
    .map(Packet::new)
    .fold(
      ["[[2]]", "[[6]]"].map(|raw| (Packet::new(raw), 1)),
      |[mut smaller, mut larger], packet| {
        if packet < larger.0 {
          larger.1 += 1;
          if packet < smaller.0 {
            smaller.1 += 1;
          }
        }
        [smaller, larger]
      },
    );

  a * (b + 1)
}

#[cfg(test)]
mod tests {

  use crate::write_tests;

  write_tests!(
    task_1_example = 13,
    task_1_actual = 5882,
    task_2_example = 140,
    task_2_actual = 24948
  );
}
