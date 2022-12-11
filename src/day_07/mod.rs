// this approach cuts some corners by assuming that the input always translates to a dfs
// and doesn't cd up until all descendants have been visited
pub fn task_1(raw: &str) -> u32 {
  let mut total = 0;

  // could maybe avoid allocation by creating an array of length N
  // where N is some hard upper bound const and also having a tail pointer to keep track of the top of the stack.
  // if the tail pointer (zero-based) ever reaches N (one-based)
  // consider it a "stack overflow" and panic.
  let mut stack = Vec::<u32>::new();

  for line in raw.lines() {
    if line.starts_with("$ cd") {
      match line.split(' ').nth(2) {
        Some("..") => {
          stack
            .pop()
            .and_then(|directory_size| {
              if directory_size <= 100_000 {
                total += directory_size;
              }
              stack.last_mut().map(|last| *last += directory_size)
            })
            .unwrap();
        }
        Some(_directory_name) => {
          stack.push(0);
        }
        _ => unreachable!("Invalid command {line}"),
      }
    } else if line != "$ ls" {
      match line.split_once(' ') {
        Some(("dir", _)) => continue,
        Some((file_size, _)) => {
          if let Some(last) = stack.last_mut() {
            *last += file_size.parse::<u32>().unwrap();
          }
        }
        _ => unreachable!("Invalid line {line}"),
      }
    }
  }

  while let Some(directory_size) = stack.pop() {
    if directory_size > 100_000 {
      break;
    }
    total += directory_size;
    if let Some(last) = stack.last_mut() {
      *last += directory_size;
    };
  }

  total
}

pub fn task_2(raw: &str) -> u32 {
  let mut minimum = Option::<u32>::None;

  // makes one pass to figure out total_space used
  // cuts some corners by assuming every line that has number followed by space is a file
  // could make it a bit more r
  let total_space_used = raw
    .lines()
    .filter_map(|line| line.split_once(' ')?.0.parse::<u32>().ok())
    .sum::<u32>();

  let unused_space_before_deletion = 70000000 - total_space_used;

  dbg!(total_space_used, unused_space_before_deletion);

  let mut stack = Vec::<u32>::new();

  for line in raw.lines() {
    if line.starts_with("$ cd") {
      match line.split(' ').nth(2) {
        Some("..") => {
          stack
            .pop()
            .and_then(|directory_size| {
              let unused_space_after_deletion = unused_space_before_deletion + directory_size;

              if unused_space_after_deletion >= 30_000_000
                && (minimum.is_none() || Some(directory_size) < minimum)
              {
                minimum = Some(directory_size);
              }

              stack.last_mut().map(|last| *last += directory_size)
            })
            .unwrap();
        }
        Some(_directory_name) => {
          stack.push(0);
        }
        _ => unreachable!("Invalid command {line}"),
      }
    } else if line != "$ ls" {
      match line.split_once(' ') {
        Some(("dir", _)) => continue,
        Some((file_size, _)) => {
          if let Some(last) = stack.last_mut() {
            *last += file_size.parse::<u32>().unwrap();
          }
        }
        _ => unreachable!("Invalid line {line}"),
      }
    }
  }

  while let Some(directory_size) = stack.pop() {
    let unused_space_after_deletion = unused_space_before_deletion + directory_size;

    if unused_space_after_deletion >= 30_000_000
      && (minimum.is_none() || Some(directory_size) < minimum)
    {
      minimum = Some(directory_size);
    }
    if let Some(last) = stack.last_mut() {
      *last += directory_size;
    }
  }

  minimum.unwrap()
}

#[cfg(test)]
mod tests {
  use crate::write_tests;

  write_tests!(
    task_1_example = 95437,
    task_1_actual = 1454188,
    task_2_example = 24933642,
    task_2_actual = 4183246
  );
}
