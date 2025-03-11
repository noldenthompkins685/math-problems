  fn main() {
    let n = 12;
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    println!("The sum of 1 to {} is: {}", n, sum);
}