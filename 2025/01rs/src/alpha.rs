use crate::Row;

pub fn solve(input: Vec<Row>) -> i32 {
  let instructions: Vec<(String, i32)> = input.into_iter().map(|x: Row| {
    let direction: String = x.clone().chars().take(1).collect();
    let steps: String = x.clone().chars().skip(1).collect();
    (direction, steps.parse::<i32>().unwrap())
  }).collect();

  let (_, zeroes) = instructions.into_iter().fold((50, 0), |(value, zeroes), (direction, steps)| {
    let tick =  ((if direction == "R" { value + steps } else { value - steps }) + 100) % 100;
    let z = if tick == 0 { zeroes + 1} else { zeroes };
    (tick, z)
  });
  zeroes
}