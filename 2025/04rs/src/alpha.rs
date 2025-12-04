use crate::Row;

fn check(grid: Vec<Vec<char>>, x: usize, y: usize, w: usize, h: usize) -> u32 {
  let tl = if x > 0 && y > 0 { if grid[y-1][x-1] == '@' { 1 } else { 0 } } else {0};
  let t = if y > 0           { if grid[y-1][x] == '@' { 1 } else { 0 } } else {0};
  let tr = if x < w && y > 0 { if grid[y-1][x+1] == '@' { 1 } else { 0 } } else {0};
  let r = if x < w           { if grid[y][x+1] == '@' { 1 } else { 0 } } else {0};
  let br = if x < w && y < h { if grid[y+1][x+1] == '@' { 1 } else { 0 } } else {0};
  let b = if y < h           { if grid[y+1][x] == '@' { 1 } else { 0 } } else {0};
  let bl = if x > 0 && y < h { if grid[y+1][x-1] == '@' { 1 } else { 0 } } else {0};
  let l = if x > 0           { if grid[y][x-1] == '@' { 1 } else { 0 } } else {0};
  (tl + t + tr + r + br + b + bl + l) as u32
}

pub fn solve(input: Vec<Row>) -> i32 {
  let grid: Vec<Vec<char>> = input
    .into_iter()
    .map(|row| row.chars().collect::<Vec<char>>())
    .collect();
  let h = grid.len();
  let w = grid[0].len();
  
  let mut res = 0;
  for y in 0..h  {
    for x in 0..w  {
      if grid[y][x] == '@' {
        let n = check(grid.clone(), x, y, w - 1, h - 1);
        if n < 4 {
          res += 1;
        }
      }
    }
  }
  res
}