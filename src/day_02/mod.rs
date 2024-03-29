pub fn task_1(raw: &str) -> u32 {
  raw
    .lines()
    .map(|line| match line {
      "A X" => 1 + 3,
      "A Y" => 2 + 6,
      "A Z" => 3,
      "B X" => 1,
      "B Y" => 2 + 3,
      "B Z" => 3 + 6,
      "C X" => 1 + 6,
      "C Y" => 2,
      "C Z" => 3 + 3,
      _ => unreachable!("Unexpected line {line}"),
    })
    .sum::<u32>()
}

pub fn task_2(raw: &str) -> u32 {
  raw
    .lines()
    .map(|line| match line {
      "A X" => 3,
      "A Y" => 1 + 3,
      "A Z" => 2 + 6,
      "B X" => 1,
      "B Y" => 2 + 3,
      "B Z" => 3 + 6,
      "C X" => 2,
      "C Y" => 3 + 3,
      "C Z" => 1 + 6,
      _ => unreachable!("Unexpected line {line}"),
    })
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 15,
    task_1_actual = 14827,
    task_2_example = 12,
    task_2_actual = 13889
  );
}
