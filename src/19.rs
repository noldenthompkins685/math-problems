use std::collections::HashMap;

fn main() {
    let mut numbers: HashMap<String, isize> = HashMap::new();

    // Add numbers to the map
    numbers.insert(String::from("1"), 1);
    numbers.insert(String::from("2"), 2);
    numbers.insert(String::from("3"), 3);

    // Calculate the sum of the values in the map
    let total: isize = numbers.values().sum();

    println!("The total is {}", total);
}
