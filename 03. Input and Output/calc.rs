use std::io;

fn main() {
    let mut input = String::new();

    // Read first number (a)
    println!("Enter the first number (a):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    // Read second number (b)
    println!("Enter the second number (b):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");

    // Perform operations
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);
}
