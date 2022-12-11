use std::collections::HashSet;

#[inline(always)]
fn update_head(direction: &str, head: &mut (i32, i32)) {
  match direction {
    "U" => head.0 += 1,
    "D" => head.0 -= 1,
    "L" => head.1 -= 1,
    "R" => head.1 += 1,
    _ => unreachable!("Unknown direction {direction}"),
  };
}

#[inline(always)]
fn get_leader_and_follower(i: usize, knots: &mut [(i32, i32)]) -> ((i32, i32), &mut (i32, i32)) {
  let (first_half, second_half) = knots.split_at_mut(i + 1);
  let leader = first_half[i];
  let follower = &mut second_half[0];
  (leader, follower)
}

fn solve<const N: usize>(raw: &str) -> usize {
  // Caller can specify knot count at compile time
  // So can use array (instead of vector to avoid heap allocation)
  // and use const generics
  let mut knots = [(0_i32, 0_i32); N];

  // Need to keep track of visited nodes. Can't see a way to avoid the heap here.
  let mut visited = HashSet::<(i32, i32)>::new();

  visited.insert(*knots.last().unwrap());

  raw
    .lines()
    .map(|line| line.split_once(' ').unwrap())
    .map(|(direction, raw_distance)| (direction, raw_distance.parse::<u32>().unwrap()))
    .for_each(|(direction, step)| {
      (1..=step).for_each(|_| {
        update_head(direction, &mut knots[0]);

        (0..knots.len() - 1).for_each(|i| {
          let (leader, follower) = get_leader_and_follower(i, &mut knots);

          let should_update_follower =
            leader.0.abs_diff(follower.0) > 1 || leader.1.abs_diff(follower.1) > 1;

          if should_update_follower {
            if leader.1 == follower.1 && 2 == (leader.0 - follower.0) {
              // directly up
              follower.0 += 1;
            } else if leader.1 == follower.1 && -2 == (leader.0 - follower.0) {
              // directly down
              follower.0 -= 1;
            } else if leader.0 == follower.0 && -2 == (leader.1 - follower.1) {
              // directly left
              follower.1 -= 1;
            } else if leader.0 == follower.0 && 2 == (leader.1 - follower.1) {
              // directly right
              follower.1 += 1;
            } else {
              if leader.0 > follower.0 {
                follower.0 += 1
              } else {
                follower.0 -= 1;
              }

              if leader.1 > follower.1 {
                follower.1 += 1;
              } else {
                follower.1 -= 1;
              }
            }
          }
        });

        visited.insert(*knots.last().unwrap());
      });
    });

  visited.len()
}

pub fn task_1(raw: &str) -> usize {
  solve::<2>(raw)
}

pub fn task_2(raw: &str) -> usize {
  solve::<10>(raw)
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 13,
    task_1_actual = 6563,
    task_2_example = 1,
    task_2_actual = 2653
  );

  const EXAMPLE_2_PUZZLE_INPUT: &str = include_str!("example_2.txt");

  #[test]
  fn test_task_2_example_2() {
    let actual = task_2(EXAMPLE_2_PUZZLE_INPUT);
    let expected = 36;
    assert_eq!(actual, expected);
  }
}
