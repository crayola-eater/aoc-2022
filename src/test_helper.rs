#[macro_export]
macro_rules! write_tests {
  (
    task_1_example = $a:expr,
    task_1_actual = $b:expr,
    task_2_example = $c:expr,
    task_2_actual = $d:expr
  ) => {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = include_str!("example.txt");
    const ACTUAL_PUZZLE_INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_task_1_example() {
      let actual = task_1(EXAMPLE_PUZZLE_INPUT);
      let expected = $a;
      assert_eq!(actual, expected);
    }

    #[test]
    fn test_task_1_actual() {
      let actual = task_1(ACTUAL_PUZZLE_INPUT);
      let expected = $b;
      assert_eq!(actual, expected);
    }

    #[test]
    fn test_task_2_example() {
      let actual = task_2(EXAMPLE_PUZZLE_INPUT);
      let expected = $c;
      assert_eq!(actual, expected);
    }

    #[test]
    fn test_task_2_actual() {
      let actual = task_2(ACTUAL_PUZZLE_INPUT);
      let expected = $d;
      assert_eq!(actual, expected);
    }
  };
}
