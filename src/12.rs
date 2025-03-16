
pub fn solve_random_math_problem() {
  let num1 = rand::thread_rng().gen_range(0..10);
  let num2 = rand::thread_rng().gen_range(0..10);
  return (num1 + num2).to_string();
}