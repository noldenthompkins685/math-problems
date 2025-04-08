use std::vec::Vec;

fn main() {
    let input = vec![1, 2, 3];
    let output = sum_elements(&input);
    println!("Output: {:?}", output);
}

fn sum_elements(input: &[i32]) -> i32 {
    input.iter().sum()
}
