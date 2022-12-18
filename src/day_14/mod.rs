use itertools::Itertools;
use std::fs::write;
use std::path::Path;

pub fn task_1(raw: &str) -> usize {
  let mut grid = parse_grid(raw);
  grid[0][500] = Square::Entry;

  let mut count = 0;

  while count <= 1_000 {
    let (mut row, mut column) = (0_usize, 500_usize);

    let fell_into_abyss = loop {
      {
        let can_move_down = row.checked_add(1).and_then(|new_row| {
          grid
            .get(new_row)?
            .get(column)
            .map(|square| square == &Square::Air)
        });

        if Some(true) == can_move_down {
          row += 1;
          continue;
        }

        if can_move_down.is_none() {
          break true;
        }
      }

      {
        let can_move_down_left = row.checked_add(1).and_then(|new_row| {
          let new_column = column.checked_sub(1)?;
          grid
            .get(new_row)?
            .get(new_column)
            .map(|square| square == &Square::Air)
        });

        if Some(true) == can_move_down_left {
          row += 1;
          column -= 1;
          continue;
        }

        if can_move_down_left.is_none() {
          break true;
        }
      }

      {
        let can_move_down_right = row.checked_add(1).and_then(|new_row| {
          let new_column = column.checked_add(1)?;
          grid
            .get(new_row)?
            .get(new_column)
            .map(|square| square == &Square::Air)
        });

        if Some(true) == can_move_down_right {
          row += 1;
          column += 1;
          continue;
        }

        if can_move_down_right.is_none() {
          break true;
        }
      }

      break false;
    };

    if fell_into_abyss {
      break;
    }

    grid[row][column] = Square::Sand;

    count += 1;
  }

  count
}

pub fn task_2(raw: &str) -> usize {
  let mut grid = parse_grid(raw);
  grid[0][500] = Square::Entry;

  let min_row = grid
    .iter()
    .rposition(|row| row.contains(&Square::Rock))
    .unwrap()
    + 2;

  grid[min_row].fill(Square::Rock);

  let mut count = 0;

  while count <= 1_000_000 {
    let (mut row, mut column) = (0_usize, 500_usize);

    let sand_is_blocking_entry = loop {
      {
        let next_cell = row
          .checked_add(1)
          .and_then(|new_row| grid.get(new_row.min(min_row))?.get(column));

        if Some(&Square::Air) == next_cell {
          row += 1;
          continue;
        }

        if Some(&Square::Entry) == next_cell {
          break true;
        }
      }

      {
        let next_cell = row.checked_add(1).and_then(|new_row| {
          let new_column = column.checked_sub(1)?;
          grid.get(new_row.min(min_row))?.get(new_column)
        });

        if Some(&Square::Air) == next_cell {
          row += 1;
          column -= 1;
          continue;
        }

        if Some(&Square::Entry) == next_cell {
          break true;
        }
      }

      {
        let next_cell = row.checked_add(1).and_then(|new_row| {
          let new_column = column.checked_add(1)?;
          grid.get(new_row.min(min_row))?.get(new_column)
        });

        if Some(&Square::Air) == next_cell {
          row += 1;
          column += 1;
          continue;
        }

        if Some(&Square::Entry) == next_cell {
          break true;
        }
      }

      break false;
    };

    if sand_is_blocking_entry {
      break;
    }

    grid[row][column] = Square::Sand;

    count += 1;

    if row == 0 && column == 500 {
      break;
    }
  }

  count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
  Entry,
  Air,
  Rock,
  Sand,
}

const GRID_WIDTH: usize = 1_000;
const GRID_HEIGHT: usize = 1_000;

type Grid = [[Square; GRID_WIDTH]; GRID_HEIGHT];

fn create_iterators(
  raw: &str,
) -> impl Iterator<Item = impl Iterator<Item = ((usize, usize), (usize, usize))> + '_> + '_ {
  let iterators = raw.lines().map(|line| {
    line
      .split(" -> ")
      .map(|raw_coord| {
        raw_coord
          .split(',')
          .map(|raw| raw.parse::<usize>().unwrap())
          .next_tuple::<(_, _)>()
          .unwrap()
      })
      .tuple_windows::<(_, _)>()
  });

  iterators
}

fn parse_grid(raw: &str) -> Grid {
  let mut grid = [[Square::Air; GRID_WIDTH]; GRID_HEIGHT];

  for iter in create_iterators(raw) {
    for ((start_col, start_row), (end_col, end_row)) in iter {
      if start_col == end_col {
        let slice = if start_row <= end_row {
          start_row..=end_row
        } else {
          end_row..=start_row
        };

        grid[slice]
          .iter_mut()
          .for_each(|row| row[start_col] = Square::Rock);
      } else if start_row == end_row {
        let slice = if start_col <= end_col {
          start_col..=end_col
        } else {
          end_col..=start_col
        };

        grid[start_row][slice].fill(Square::Rock);
      } else {
        unreachable!("Unexpected movement");
      }
    }
  }

  grid
}

fn grid_to_string(grid: &[[Square; GRID_WIDTH]]) -> String {
  grid
    .iter()
    .map(|row| {
      row
        .iter()
        .map(|cell| match cell {
          Square::Air => '.',
          Square::Rock => '#',
          Square::Sand => 'o',
          Square::Entry => '+',
        })
        .collect::<String>()
    })
    .join("\n")
}

#[allow(dead_code)]
fn debug_grid(grid: &Grid, filename: &str) {
  let path = Path::new("debug-14").join(filename);
  let content = grid_to_string(grid);
  write(path, content.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {

  use crate::write_tests;

  write_tests!(
    task_1_example = 24,
    task_1_actual = 618,
    task_2_example = 93,
    task_2_actual = 26358
  );

  const EMPTY_GRID: Grid = [[Square::Air; GRID_WIDTH]; GRID_HEIGHT];

  #[test]
  fn test_parse_grid_empty_grid() {
    let actual = parse_grid("");
    let expected = &EMPTY_GRID;
    assert_eq!(&actual, expected);
  }
}
