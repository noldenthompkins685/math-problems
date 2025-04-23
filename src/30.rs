use std::fs;

fn main() {
    let file_path = "math-problems.txt";
    if let Ok(file_content) = fs::read_to_string(file_path) {
        println!("File content:");
        for line in file_content.lines() {
            println!("{}", line);
        }
    } else {
        println!("Failed to read the file.");
    }
}
