fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1..=20);
    let b = rng.gen_range(1..=20);
    println!("{} + {}", a, b);
}