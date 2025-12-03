use crate::Row;

pub fn solve(input: Vec<Row>) -> u32 {
  input.into_iter().map(|row| {
    // let lul = row.split("").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let digits: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let l = digits.len();
    let d2 = digits.clone().into_iter().take(l - 1).max().unwrap();
    let (i, _) = digits.clone().into_iter().enumerate().find(|(_, x)| *x == d2).unwrap();
    let d1 = digits.clone().into_iter().skip(i+1).max().unwrap();
      d2*10 + d1
  }).sum()
}