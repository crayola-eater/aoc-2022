#[inline(always)]
fn sum_calories_for_elf(elf: &str) -> u32 {
  elf
    .lines()
    .map(|line| line.parse::<u32>().expect("Non-numerical calories"))
    .sum::<u32>()
}

pub fn task_1(raw: &str) -> u32 {
  raw
    .split("\n\n")
    .map(sum_calories_for_elf)
    .max()
    .expect("Failed to find max calories")
}

pub fn task_2(raw: &str) -> u32 {
  raw
    .split("\n\n")
    .map(sum_calories_for_elf)
    .fold([u32::MIN; 3], |[first, second, third], calories| {
      if calories > first {
        [calories, first, second]
      } else if calories > second {
        [first, calories, second]
      } else if calories > third {
        [first, second, calories]
      } else {
        [first, second, third]
      }
    })
    .into_iter()
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 24000,
    task_1_actual = 64929,
    task_2_example = 45000,
    task_2_actual = 193697
  );
}
