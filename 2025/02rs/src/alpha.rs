use crate::Row;

fn test_repeat(n: i64) -> i64 {
  let s = n.to_string();
  let length = s.len();
  let size = length / 2;
  let first: String = s.chars().take(size).collect();
  let last: String = s.chars().skip(size).take(size).collect();
  if length % 2 != 0 { 
    return 0
  }
  if first == last { n } else { 0 }
}

fn get_range(range: String) -> Vec<i64> {
  let limits: Vec<i64> = range.split("-").into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
  let start = limits[0];
  let end = limits[1];
  
  let v: Vec<i64> = vec![start; (end-start + 1) as usize]
    .into_iter()
    .enumerate()
    .map(|(i, x)| x + i as i64)
    .map(|n| test_repeat(n))
    .filter(|n| *n != 0)
    .collect();
  v
}

pub fn solve(input: Vec<Row>) -> i64 {
  input.into_iter().map(|range| get_range(range)).flatten().sum::<i64>()
}