mod monkey;
use monkey::{get_monkey_business, parse_monkeys};

fn solve(raw: &str, rounds: usize, divisor: Option<usize>) -> usize {
  let mut monkeys = parse_monkeys(raw);

  let worry_divisor = monkeys
    .iter()
    .map(|monkey| monkey.divisor)
    .product::<usize>();

  for _ in 1..=rounds {
    for index in 0..monkeys.len() {
      while let Some(worry_level) = monkeys[index].try_to_throw() {
        let (worry_level, index_to_throw_to) = {
          let monkey = &monkeys[index];
          let new_worry_level = if let Some(divisor) = divisor {
            monkey.get_next_worry_level(worry_level) / divisor
          } else {
            monkey.get_next_worry_level(worry_level) % worry_divisor
          };
          let index = monkey.get_next_monkey_index(new_worry_level);
          (new_worry_level, index)
        };

        monkeys[index_to_throw_to].catch(worry_level);
      }
    }
  }

  get_monkey_business(&monkeys)
}

pub fn task_1(raw: &str) -> usize {
  solve(raw, 20, Some(3))
}

pub fn task_2(raw: &str) -> usize {
  solve(raw, 10_000, None)
}

#[cfg(test)]
mod tests {

  use crate::write_tests;

  write_tests!(
    task_1_example = 10605,
    task_1_actual = 56595,
    task_2_example = 2713310158,
    task_2_actual = 15693274740
  );
}
