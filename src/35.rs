// Rust code snippet for solving the given math problem and generating random numbers.
// The problem is: "Find the sum of all multiples of 3 or 7 within the range [100, 500].
fn main() {
    let mut total_sum = 0;
    for i in 100..=500 {
        if i % 3 == 0 || i % 7 == 0 {
            total_sum += i;
        }
    }
    println!("Sum of multiples of 3 or 7 between 100 and 500: {}", total_sum);
}
