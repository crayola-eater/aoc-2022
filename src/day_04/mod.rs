use itertools::Itertools;

#[inline(always)]
pub fn parse_line(line: &str) -> [[u32; 2]; 2] {
  line
    .split(',')
    .next_tuple::<(_, _)>()
    .map(|(first, second)| {
      [first, second].map(|pair| {
        let (start, end) = pair.split('-').next_tuple::<(_, _)>().unwrap();
        [start, end].map(|raw| raw.parse::<u32>().expect("Invalid number"))
      })
    })
    .expect("Failed to get two pairs")
}

pub fn task_1(raw: &str) -> usize {
  raw
    .lines()
    .map(|line| parse_line(line))
    .filter(|[[a, b], [c, d]]| (a <= c && b >= d) || (c <= a && d >= b))
    .count()
}

pub fn task_2(raw: &str) -> usize {
  raw
    .lines()
    .map(|line| parse_line(line))
    .filter(|[[a, b], [c, d]]| !(b < c || d < a))
    .count()
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 2,
    task_1_actual = 524,
    task_2_example = 4,
    task_2_actual = 798
  );
}
