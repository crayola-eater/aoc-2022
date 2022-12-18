use std::collections::VecDeque;
type Grid = Vec<Vec<(u8, bool)>>;
type Position = (usize, usize);

#[inline(always)]
fn find_position(maze: &Grid, byte_to_find: u8) -> Option<Position> {
  maze.iter().enumerate().find_map(|(row, cells)| {
    cells
      .iter()
      .position(|(byte, _)| &byte_to_find == byte)
      .map(|column| (row, column))
  })
}

#[inline(always)]
fn parse_grid(raw: &str) -> Grid {
  raw
    .lines()
    .map(|line| line.as_bytes().iter().map(|&byte| (byte, false)).collect())
    .collect()
}

pub fn task_1(raw: &str) -> usize {
  let mut grid = parse_grid(raw);
  let start = find_position(&grid, b'S').unwrap();
  let end = find_position(&grid, b'E').unwrap();

  grid[end.0][end.1].0 = b'z';
  grid[start.0][start.1].0 = b'a';

  let mut min_steps = usize::MAX;
  let mut queue = VecDeque::<(Position, usize)>::from_iter([(end, 0)]);

  while let Some((current @ (row, column), steps)) = queue.pop_front() {
    let byte = if let Some((byte, false)) = grid.get(row).and_then(|cells| cells.get(column)) {
      *byte
    } else {
      continue;
    };

    if current == start {
      min_steps = min_steps.min(steps);
      continue;
    }

    grid[row][column].1 = true;

    let neighbours =
      [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(row_delta, column_delta)| {
          let next_row = row.checked_add_signed(row_delta)?;
          let next_column = column.checked_add_signed(column_delta)?;
          let (next_byte, already_visited) = grid.get(next_row)?.get(next_column)?;

          if !*already_visited && next_byte >= &byte.saturating_sub(1) {
            Some(((next_row, next_column), steps + 1))
          } else {
            None
          }
        });

    queue.extend(neighbours);
  }

  min_steps
}

pub fn task_2(raw: &str) -> usize {
  let mut grid = parse_grid(raw);
  let end = find_position(&grid, b'E').unwrap();
  grid[end.0][end.1].0 = b'z';

  let mut min_steps = usize::MAX;
  let mut queue = VecDeque::<(Position, usize)>::from_iter([(end, 0)]);

  while let Some(((row, column), steps)) = queue.pop_front() {
    let byte = if let Some((byte, false)) = grid.get(row).and_then(|cells| cells.get(column)) {
      *byte
    } else {
      continue;
    };

    if byte == b'a' {
      min_steps = min_steps.min(steps);
      break;
    }

    grid[row][column].1 = true;

    let neighbours =
      [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(row_delta, column_delta)| {
          let next_row = row.checked_add_signed(row_delta)?;
          let next_column = column.checked_add_signed(column_delta)?;
          let (next_byte, already_visited) = grid.get(next_row)?.get(next_column)?;

          if !*already_visited && next_byte >= &byte.saturating_sub(1) {
            Some(((next_row, next_column), steps + 1))
          } else {
            None
          }
        });

    queue.extend(neighbours);
  }

  min_steps
}

#[cfg(test)]
mod tests {

  use crate::write_tests;

  write_tests!(
    task_1_example = 31,
    task_1_actual = 449,
    task_2_example = 29,
    task_2_actual = 443
  );
}
