fn create_instructions_iterator(raw: &str) -> impl Iterator<Item = Option<i32>> + '_ {
  raw.lines().map(|line| {
    line
      .split_once(' ')
      .map(|(raw_instr, raw_arg)| match raw_instr {
        "addx" => raw_arg.parse::<i32>().unwrap(),
        _ => unreachable!("Unknown instruction {raw_instr}"),
      })
  })
}

pub fn task_1(raw: &str) -> i32 {
  let mut x = 1;
  let mut cycle = 1;
  let mut target_cycle = 20;
  let mut signal_strength = 0;

  for parsed in create_instructions_iterator(raw) {
    let limit = cycle + parsed.map_or(0, |_| 1);

    while cycle <= limit {
      if target_cycle == cycle {
        signal_strength += x * cycle;
        target_cycle += 40;
      }

      cycle += 1;
    }

    if let Some(increment) = parsed {
      x += increment;
    }
  }

  signal_strength
}

pub fn task_2(raw: &str) -> String {
  let mut x = 1;
  let mut cycle = 1;

  let mut output = String::with_capacity(245);

  for parsed in create_instructions_iterator(raw) {
    let limit = cycle + parsed.map_or(0, |_| 1);

    while cycle <= limit {
      let row = (cycle - 1) / 40;
      let column = (cycle - 1) % 40;

      if row > 0 && 0 == column {
        output.push('\n');
      }

      let overlaps = column >= (x - 1) && column <= (x + 1);

      let pixel = if overlaps { '#' } else { '.' };
      output.push(pixel);

      cycle += 1;
    }

    if let Some(increment) = parsed {
      x += increment;
    }
  }

  output
}

#[cfg(test)]
mod tests {

  use crate::write_tests;

  write_tests!(
    task_1_example = 13140,
    task_1_actual = 15020,
    task_2_example = String::from(
      r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
    ),
    task_2_actual = String::from(
      r#"####.####.#..#..##..#....###...##..###..
#....#....#..#.#..#.#....#..#.#..#.#..#.
###..###..#..#.#....#....#..#.#..#.#..#.
#....#....#..#.#.##.#....###..####.###..
#....#....#..#.#..#.#....#....#..#.#....
####.#.....##...###.####.#....#..#.#...."#
    )
  );
}
