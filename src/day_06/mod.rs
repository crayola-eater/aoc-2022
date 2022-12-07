fn position_of_n_unique_consecutive_bytes(bytes: &[u8], n: usize) -> usize {
  let mut ascii_table = [0_u32; 123];
  let mut unique_count = 0;
  let mut i = 0;

  while i < n {
    let index = bytes[i] as usize;
    ascii_table[index] += 1;
    if 1 == ascii_table[index] {
      unique_count += 1;
    }
    i += 1;
  }

  while i < bytes.len() && unique_count < n {
    let index_to_remove = bytes[i - n] as usize;
    let index_to_add = bytes[i] as usize;

    i += 1;

    if index_to_add != index_to_remove {
      ascii_table[index_to_remove] -= 1;

      if 0 == ascii_table[index_to_remove] {
        unique_count -= 1;
      }

      ascii_table[index_to_add] += 1;

      if 1 == ascii_table[index_to_add] {
        unique_count += 1;
      }
    }
  }

  if unique_count != n {
    panic!(
      "Failed to find {n} unique consecutive bytes in {:?}",
      std::str::from_utf8(bytes)
    );
  }

  i
}

pub fn task_1(raw: &str) -> usize {
  position_of_n_unique_consecutive_bytes(raw.as_bytes(), 4)
}

pub fn task_2(raw: &str) -> usize {
  position_of_n_unique_consecutive_bytes(raw.as_bytes(), 14)
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 7,
    task_1_actual = 1544,
    task_2_example = 19,
    task_2_actual = 2145
  );

  const EXAMPLE_2_PUZZLE_INPUT: &str = include_str!("example_2.txt");
  const EXAMPLE_3_PUZZLE_INPUT: &str = include_str!("example_3.txt");
  const EXAMPLE_4_PUZZLE_INPUT: &str = include_str!("example_4.txt");
  const EXAMPLE_5_PUZZLE_INPUT: &str = include_str!("example_5.txt");

  #[test]
  fn test_task_1_example_2() {
    let actual = task_1(EXAMPLE_2_PUZZLE_INPUT);
    let expected = 5;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_task_1_example_3() {
    let actual = task_1(EXAMPLE_3_PUZZLE_INPUT);
    let expected = 6;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_task_1_example_4() {
    let actual = task_1(EXAMPLE_4_PUZZLE_INPUT);
    let expected = 10;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_task_1_example_5() {
    let actual = task_1(EXAMPLE_5_PUZZLE_INPUT);
    let expected = 11;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_task_2_example_2() {
    let actual = task_2(EXAMPLE_2_PUZZLE_INPUT);
    let expected = 23;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_task_2_example_3() {
    let actual = task_2(EXAMPLE_3_PUZZLE_INPUT);
    let expected = 23;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_task_2_example_4() {
    let actual = task_2(EXAMPLE_4_PUZZLE_INPUT);
    let expected = 29;
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_task_2_example_5() {
    let actual = task_2(EXAMPLE_5_PUZZLE_INPUT);
    let expected = 26;
    assert_eq!(actual, expected);
  }
}
