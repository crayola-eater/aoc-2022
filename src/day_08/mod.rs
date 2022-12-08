type Grid<'a> = Vec<&'a [u8]>;

#[inline(always)]
fn parse_grid(raw: &str) -> (Grid, usize, usize) {
  let grid = raw.lines().map(|line| line.as_bytes()).collect::<Grid>();
  let height = grid.len();
  let width = grid[0].len();
  (grid, height, width)
}

pub fn task_1(raw: &str) -> usize {
  let (grid, height, width) = parse_grid(raw);

  let outer_count = 2 * (width + height - 2);
  let inner_count = (1..height - 1)
    .flat_map(|row| (1..width - 1).map(move |column| (row, column)))
    .filter(|(row, column)| {
      let current = grid[*row][*column];
      [
        (0..*column, true),
        (column + 1..width, true),
        (0..*row, false),
        (row + 1..height, false),
      ]
      .into_iter()
      .any(|(iter, is_horizontal)| {
        iter
          .map(|i| {
            if is_horizontal {
              grid[*row][i]
            } else {
              grid[i][*column]
            }
          })
          .all(|neighbour| neighbour < current)
      })
    })
    .count();

  inner_count + outer_count
}

#[inline(always)]
fn get_leftward_scenic_score(grid: &Grid, row: usize, column: usize) -> usize {
  let current = grid[row][column];
  let mut score = 0;

  for neighbour in (0..column).rev().map(|x| grid[row][x]) {
    score += 1;
    if neighbour >= current {
      break;
    }
  }

  score
}

#[inline(always)]
fn get_rightward_scenic_score(grid: &Grid, row: usize, column: usize) -> usize {
  let current = grid[row][column];
  let width = grid[0].len();
  let mut score = 0;

  for neighbour in (column + 1..width).map(|x| grid[row][x]) {
    score += 1;
    if neighbour >= current {
      break;
    }
  }

  score
}

#[inline(always)]
fn get_upward_scenic_score(grid: &Grid, row: usize, column: usize) -> usize {
  let current = grid[row][column];
  let mut score = 0;

  for neighbour in (0..row).rev().map(|y| grid[y][column]) {
    score += 1;
    if neighbour >= current {
      break;
    }
  }

  score
}

#[inline(always)]
fn get_downward_scenic_score(grid: &Grid, row: usize, column: usize) -> usize {
  let current = grid[row][column];
  let height = grid.len();
  let mut score = 0;

  for neighbour in (row + 1..height).map(|y| grid[y][column]) {
    score += 1;
    if neighbour >= current {
      break;
    }
  }

  score
}

pub fn task_2(raw: &str) -> usize {
  let (grid, height, width) = parse_grid(raw);

  (1..height - 1)
    .flat_map(|row| (1..width - 1).map(move |column| (row, column)))
    .map(|(row, column)| {
      get_leftward_scenic_score(&grid, row, column)
        * get_rightward_scenic_score(&grid, row, column)
        * get_upward_scenic_score(&grid, row, column)
        * get_downward_scenic_score(&grid, row, column)
    })
    .max()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 21,
    task_1_actual = 1700,
    task_2_example = 8,
    task_2_actual = 470596
  );
}
